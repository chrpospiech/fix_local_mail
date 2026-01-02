# fix_local_mail

## Introduction

According to the following URL
<http://techbase.kde.org/Projects/PIM/Akonadi#Where_does_Akonadi_store_my_data.3F>,
akonadi merely acts as a cache for your data, the actual content
stays where it has always been, .ics/.vcf/MBOX files, local maildirs,
IMAP- and groupware servers. There is only a limited amount of data stored
exclusively in Akonadi:

- Data not supported by the corresponding backends, such as email flags in
  case of maildir/mbox. This is comparable to KMail's binary index files
  stored alongside these files in pre-Akonadi times.
- Internal meta-data used by application or resources, such as information
  about the last synchronization with a backend or translated folder names.
- Data that has been changed while the corresponding backend has been offline
  and has not yet been uploaded.

This script deals with some cases where this concept goes wrong.

- Some cases have been seen, when akonadi cannot write to a
  local mail dir. Mails cannot be stored in this location and hence
  stay in cache. If akonadi is re-indexed, the non existent mail dir
  and all mails therein may be erased from akonadi. In this case these
  mails are lost forever.
- Each maildir folder has sub-folders tmp, new and cur. Mails
  should be moved from new to cur and renamed by appending `:2,S`
  once they have been opened and read. Kmail as of 2025 does not
  follow this rule in all cases, but sometimes only updates akonadi
  without updating the maildir structure accordingly.
  If the akonadi database is scratched and recreated, these mails
  are marked \"unread\" again.

This tool is written to reconstruct and repair the mail dir
structure and recover mails from the akonadi database cache.

## Naming of emails

According to <https://cr.yp.to/proto/maildir.html>
we have the following.

- When you move a file from new to cur, you have to change
  its name from `<uniq>` to `<uniq:info>`.
- `info` starting with `2,`. Each character after the comma
  is an independent flag.

   - Flag "P" (passed): the user has resent/forwarded/bounced
     this message to someone else.
   - Flag "R" (replied): the user has replied to this message.
   - Flag "S" (seen): the user has viewed this message, though
     perhaps he didn't read all the way through it.
   - Flag "T" (trashed): the user has moved this message to
     the trash; the trash will be emptied by a later user action.
   - Flag "D" (draft): the user considers this message a draft;
     toggled at user discretion.
   - Flag "F" (flagged): user-defined flag; toggled at user discretion.

- Flags must be stored in ASCII order: e.g., "2,FRS".

## Usage of the tool

```bash
Usage: fix_local_mail [OPTIONS]

Options:
  -D, --dry-run
          Perform a dry run without making actual changes
  -n, --limit <LIMIT>
          Limit the number of processed messages [default: 0]
  -m, --min-id <MIN_ID>
          Minimum message ID to process [default: 0]
  -p, --maildir-path <MAILDIR_PATH>
          maildir root path if not determined automatically [default: auto]
  -c, --mail-cache-path <MAIL_CACHE_PATH>
          mail cache path if not determined automatically [default: auto]
  -u, --db-url <DB_URL>
          Database URL if not determined automatically [default: auto]
  -i, --ignore-new-dirs
          Ignore list of mails in new directories
  -a, --stop-akonadi
          Stop Kmail and Akonadi after processing
  -k, --stop-kmail
          Stop Kmail after processing
  -v, --verbose
          Verbose output
  -h, --help
          Print help
  -V, --version
          Print version
```

## Implementation

- Find the emails that - potentially - need a change.

   - Unless args.ignore_new_dirs is set, walk through all `new` directories
     and record the basenames found that match r"/new/(\d+.*\:2\,.*)$".
     New mails are by definition neither read, forwarded or answered.
     Feed these names into an SQL query for the Akonadi database to
     additionally filter emails to meet the following two conditions.
   - All email with the dirty flag set to indicate that they are kept in
     the email cache.
   - All emails with basenames not meeting `%:2,%S` as SQL wild card.
     This is based on the assumption that all emails are read (and marked
     "seen") before being moved to the local mail folder.
   - All emails marked `answered`, but not matching `%2%RS`. `RS` stands
     for "replied and seen".
   - If args.limit is set to a non-zero value, limit the number of results
     to this value.
   - The complete `SQL` query looks like the following for a non-empty list
     of emails in `new` directories and a positive value of args.limit.

     ```SQL
     SELECT `id`,
            CONVERT(`remoteId`, CHAR) AS `remote_id`,
            `collectionId` AS `collection_id`
        FROM `pimitemtable`
        WHERE `mimeTypeId` = 2
        AND `id` >= <value of args.min_id>
        AND (`dirty` = 1 OR `remoteId` NOT LIKE '%:2%S'
             OR (`id` IN (SELECT `pimItem_Id`
                          FROM `pimitemflagrelation`
                          WHERE `flag_Id` IN (SELECT `id`
                                              FROM `flagtable`
                                              WHERE `name` LIKE '%ANSWERED'))
                  AND `remoteId` NOT LIKE '%:2%RS'
                )
             OR `remoteId` IN (<list of emails in new dirs>)
             )
        AND `collectionId` IN (
            SELECT id FROM `collectiontable` WHERE `resourceId` = 3
        )
        ORDER BY `id`
        LIMIT <args.limit>
        ```

- Find the original location of the email in absolute path names

   - If `dirty` is not set, glob in the original directory to find whether
     the email is in the `new`, `cur` or `tmp` directory.
   - If the dirty flag is set, the email is kept in cache. Inspect the
     `parttable` table in the Akonadi database whether the email is kept
     on disk or in the database. In case of the letter, copy the email
     into a temporary file on disk in the cache directory.
   - The root of the local mail directory and the location of the mail cache
     can be changed by command line options for debugging and unit testing.

- Compose the absolute path name with the correct email naming by inspecting
  the table `pimitemflagrelation` in the Akonadi database.
- Create the destination path if it does not exist and check for correct
  file and directory permissions.
- Unless the dry-run flag is set, execute the following operations.

   - Move the email from the original location (or temporary file) to the
     absolute path name with the correct email naming by a `UNIX` rename
     (`inode` operation only).
   - Delete the original `id` from the `pimitemtable`, which clears all
     related entries in tables `parttable` and `pimitemflagrelation` through
     a delete cascade.
   - Trigger Akonadi synchronization from disk by sending a `synchronization`
     request through DBus.
   - If requested through command line option, stop Kmail and Akonadi through
     appropriate DBus requests.

Except for the last three items, the Akonadi database is accessed only
with read operations which should not interfere with Akonadi. Akonadi
is contacted only through DBus requests, except for SQL delete operation.
The latter avoids confusion as the corrected emails are seen as newly
imported emails that create new appropriate entries in the Akonadi
database --- based in their name on path location.

## Debugging and Unit Testing

- To help with debugging and unit testing, the list of mails in "/new/"
  directories can be ignored and an alternative URL to a database can
  be provided via command line options. In case of an
  alternative URL, no mail directories or mail caches are accessed
  to avoid clashes of current mail directories or mail caches and different
  data in the database.
- For unit testing, command line options

   - `-p, --maildir-path <MAILDIR_PATH>`
   - `-c, --mail-cache-path`

  allow for a mock up of a full mail environment.

## Lessons Learned

### Correct Way of Integrating CMakeLists.txt

- A `CMakeLists.txt` on the top level directory helps `VS code` to recognize that
  there is a C++ build process and enables the correct tools.
- When triggered by an entry `build = "build.rs"` in `cargo.toml`, `cargo build`
  is calling `build.rs` which executes `cmake` to configure, build and install.
- The `build.rs` in commit `bd478043` executes the same `cmake` commands as `VS code`
  does. This is different from what is typically suggested by AI.

### Correct Way of Integrating a C++ Library

- Called functions need to be exported in the dynamic symbol table.
- `cargo` needs to set the `rpath` in order to find the dynamic library at run time.

### Volatile flag ids

The `flagtable`, particularly the `flagtable`.`id` entries, might be different for
each user. Hence they should not be used in SQL queries. Instead a four letter
acronym `flag` is used instead. For example, the table might look like this.

```SQL
MariaDB [akonadi]> SELECT *,
   SUBSTR(CONVERT(`flagtable`.`name`, CHAR), 2,4) AS `flag`
   FROM `flagtable`;
+----+------------------+------+
| id | name             | flag |
+----+------------------+------+
|  4 | $ATTACHMENT      | ATTA |
| 13 | $ENCRYPTED       | ENCR |
| 15 | $ERROR           | ERRO |
|  6 | $FORWARDED       | FORW |
|  5 | $HasAttachment   | HasA |
|  3 | $HasNoAttachment | HasN |
|  2 | $IGNORED         | IGNO |
|  8 | $INVITATION      | INVI |
| 10 | $QUEUED          | QUEU |
| 12 | $REPLIED         | REPL |
| 11 | $SENT            | SENT |
|  7 | $SIGNED          | SIGN |
|  9 | \ANSWERED        | ANSW |
| 16 | \DELETED         | DELE |
| 14 | \FLAGGED         | FLAG |
|  1 | \SEEN            | SEEN |
+----+------------------+------+
16 rows in set (0,000 sec)
```
