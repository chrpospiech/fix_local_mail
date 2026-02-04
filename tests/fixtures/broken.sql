
--
-- Table structure for table `resourcetable`
--

DROP TABLE IF EXISTS `resourcetable`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `resourcetable` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `name` varbinary(255) NOT NULL,
  `isVirtual` tinyint(1) DEFAULT 0,
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=12 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Dumping data for table `resourcetable`
--

INSERT INTO `resourcetable` VALUES
(1,'akonadi_search_resource',1),
(2,'akonadi_unifiedmailbox_agent',0),
(3,'akonadi_maildir_resource_0',0),
(4,'akonadi_birthdays_resource',0),
(5,'akonadi_contacts_resource_0',0),
(6,'akonadi_ical_resource_0',0),
(7,'akonadi_akonotes_resource_0',0),
(8,'akonadi_imap_resource_0',0);
