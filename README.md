# fix_akonadi

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
