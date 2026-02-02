
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

--
-- Table structure for table `collectiontable`
--

CREATE TABLE `collectiontable` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `remoteId` varbinary(255) DEFAULT NULL,
  `remoteRevision` varbinary(255) DEFAULT NULL,
  `name` varbinary(255) NOT NULL,
  `parentId` bigint(20) DEFAULT NULL,
  `resourceId` bigint(20) NOT NULL,
  `enabled` tinyint(1) NOT NULL DEFAULT 1,
  `syncPref` tinyint(4) DEFAULT 2,
  `displayPref` tinyint(4) DEFAULT 2,
  `indexPref` tinyint(4) DEFAULT 2,
  `cachePolicyInherit` tinyint(1) NOT NULL DEFAULT 1,
  `cachePolicyCheckInterval` int(11) NOT NULL DEFAULT -1,
  `cachePolicyCacheTimeout` int(11) NOT NULL DEFAULT -1,
  `cachePolicySyncOnDemand` tinyint(1) NOT NULL DEFAULT 0,
  `cachePolicyLocalParts` varbinary(255) DEFAULT NULL,
  `queryString` varbinary(32768) DEFAULT NULL,
  `queryAttributes` varbinary(255) DEFAULT NULL,
  `queryCollections` varbinary(255) DEFAULT NULL,
  `isVirtual` tinyint(1) DEFAULT 0,
  PRIMARY KEY (`id`),
  UNIQUE KEY `CollectionTable_parentAndNameIndex` (`parentId`,`name`),
  KEY `CollectionTable_parentIndex` (`parentId`),
  KEY `CollectionTable_resourceIndex` (`resourceId`),
  KEY `CollectionTable_enabledIndex` (`enabled`),
  KEY `CollectionTable_syncPrefIndex` (`syncPref`),
  KEY `CollectionTable_displayPrefIndex` (`displayPref`),
  KEY `CollectionTable_indexPrefIndex` (`indexPref`),
  CONSTRAINT `collectiontable_ibfk_2` FOREIGN KEY (`resourceId`) REFERENCES `resourcetable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=395 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci ROW_FORMAT=DYNAMIC;

--
-- Dumping data for table `collectiontable`
--

INSERT INTO `collectiontable` VALUES
(1,NULL,NULL,'Search',NULL,1,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,1),
(9,'/home/cp/.local/share/contacts/',NULL,'Personal Contacts',NULL,5,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(11,'/home/cp/.local/share/apps/korganizer/std.ics',NULL,'akonadi_ical_resource_0',NULL,6,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(13,'/home/cp/.local/share/notes/',NULL,'Notes',NULL,7,1,2,2,2,0,-1,1,1,'ENVELOPE',NULL,NULL,NULL,0),
(14,'imap://pospiech-HD@t-online.de@secureimap.t-online.de/',NULL,'t-online (pospiech-HD)',NULL,8,1,2,2,2,0,5,-1,1,'ENVELOPE HEAD RFC822',NULL,NULL,NULL,0),
(15,'.INBOX',NULL,'INBOX',14,8,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(16,'.SprachBox',NULL,'SprachBox',15,8,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(17,'.Sent',NULL,'Sent',15,8,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(18,'.Spam',NULL,'Spam',15,8,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(19,'.Trash',NULL,'Trash',15,8,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(20,'.Junk',NULL,'Junk',15,8,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(21,'.Drafts',NULL,'Drafts',15,8,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(36,'.templates',NULL,'templates',15,8,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(46,'/home/cp/.local/share/akonadi_maildir_resource_0/',NULL,'Local Folders',NULL,3,1,2,2,2,0,-1,1,1,'ENVELOPE',NULL,NULL,NULL,0),
(47,'drafts',NULL,'drafts',46,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(48,'inbox',NULL,'inbox',46,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(49,'outbox','0','outbox',46,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(50,'sent-mail','0','sent-mail',46,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(51,'templates','0','templates',46,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(52,'trash','0','trash',46,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(53,'AltHendesse','1759137451171','AltHendesse',46,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(55,'FormerIBMers','1766491530770','FormerIBMers',46,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(58,'misc','0','misc',46,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(59,'PIK_workshop','1763024439726','PIK_workshop',368,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(60,'TanzClub',NULL,'TanzClub',46,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(61,'Thinkpad','1741176815565','Thinkpad',368,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(62,'tools',NULL,'tools',46,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(63,'weather','1753118448427','weather',46,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(64,'followup','1767023432805','followup',48,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(66,'to_be_filed','1395749532000','to_be_filed',48,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(129,'Abitreffen','1767111554732','Abitreffen',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(130,'Accurada','1491558533658','Accurada',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(131,'Amazon','1462447100333','Amazon',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(132,'AMEX','1605292511949','AMEX',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(133,'BitBucket','1744011044032','BitBucket',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(135,'DBahn','1764279183425','DBahn',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(137,'DHL','1743842320329','DHL',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(139,'DMV','1760540677130','DMV',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(140,'ebay','1754384089213','ebay',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(141,'English_Gardens','1589189517061','English_Gardens',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(142,'FernReisen','1763971051064','FernReisen',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(143,'gesche','1763971073963','gesche',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(144,'GitHub','1741088977793','GitHub',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(145,'Greenwheels','1343914784000','Greenwheels',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(146,'H&H','1764519604143','H&H',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(148,'Hertz','1745510150990','Hertz',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(149,'Hujer','1753866162437','Hujer',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(150,'Irmgard','1631815935202','Irmgard',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(151,'Jacques','1739557590101','Jacques',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(152,'Lenovo_shares','1761641548695','Lenovo_shares',359,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(153,'Leupold','1747034116463','Leupold',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(154,'Logis','1758562631582','Logis',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(155,'Mediencenter','1486381526267','Mediencenter',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(156,'Meissner_Raeder','1763970907156','Meissner_Raeder',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(157,'misc','1765531121500','misc',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(158,'OpenTable','1697881815275','OpenTable',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(160,'Rebecca','1755868491453','Rebecca',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(162,'SLUB','1744359744275','SLUB',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(163,'Solveig','1764519586931','Solveig',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(165,'StudStif','1765634374605','StudStif',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(166,'T-com','1767089623698','T-com',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(167,'TeilAuto','1764854664296','TeilAuto',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(168,'Tempo30','1505814667440','Tempo30',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(169,'VCD','1758562718775','VCD',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(170,'VMware','1330783245000','VMware',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(171,'Volker','1763971085205','Volker',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(173,'Warentest','1557663992967','Warentest',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(174,'Wikimedia','1762418097783','Wikimedia',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(175,'Zahn+Art','1727172837720','Zahn+Art',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(176,'Couronne','1763117009600','Couronne',60,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(177,'Lippmann','1766660742689','Lippmann',60,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(178,'Residenz','1766846355151','Residenz',60,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(179,'TC_Saxonia','1517662162984','TC_Saxonia',60,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(184,'CMake','1713441017902','CMake',62,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(190,'HW_counters','1610723976629','HW_counters',62,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(191,'jube','1472735560172','jube',62,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(192,'Jupyter','1636562888970','Jupyter',62,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(195,'LICO','1716362985362','LICO',62,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(201,'OpenDX','1381917543000','OpenDX',62,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(205,'postp2sql','1765042652980','postp2sql',62,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(208,'Sphinx','1533807893541','Sphinx',62,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(211,'Version Control','1670427419601','Version Control',62,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(216,'HR','1743498223958','HR',368,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(218,'MAK','1759425878196','MAK',368,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(300,'Adventsturnier','1733719697125','Adventsturnier',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(301,'Anfragen','1744621175634','Anfragen',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(302,'Auftritt','1664871356613','Auftritt',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(303,'Covid-19','1645611628191','Covid-19',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(304,'Einbruch','1489394289764','Einbruch',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(305,'Finanz','1764520867532','Finanz',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(306,'Followup','1652338105761','Followup',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(307,'Fusion','1707464407779','Fusion',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(308,'JMD','1515225525046','JMD',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(309,'Loebtau','1703083251156','Loebtau',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(310,'LTVS','1733719404304','LTVS',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(311,'marketting','1465372663731','marketting',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(312,'Misc','1758180126227','Misc',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(313,'Mitgl-Info','1762418519994','Mitgl-Info',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(314,'Mitglieder','1762417929789','Mitglieder',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(315,'MV','1745315298964','MV',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(316,'October3','1515226445710','October3',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(317,'Praesidium','1754469340199','Praesidium',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(318,'Praesidiumssitzung','1732448907491','Praesidiumssitzung',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(319,'Protokolle','1665049216602','Protokolle',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(320,'REHA','1703158562055','REHA',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(321,'Rollies','1733719305328','Rollies',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(322,'Stadtsportbund','1748593664968','Stadtsportbund',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(323,'Turnier','1623826950996','Turnier',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(324,'unsorted','1743502346591','unsorted',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(325,'Veranstaltungen','1762418469177','Veranstaltungen',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(326,'Vermietung','1712578364755','Vermietung',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(327,'WebSeite','1609256286201','WebSeite',179,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(344,NULL,NULL,'OpenInvitations',1,1,1,2,2,2,1,-1,-1,0,NULL,'{\n    \"limit\": -1,\n    \"negated\": false,\n    \"rel\": 1,\n    \"subTerms\": [\n        {\n            \"cond\": 0,\n            \"key\": \"partstatus\",\n            \"negated\": false,\n            \"value\": \"pospiech-HD@t-online.de0\"\n        }\n    ]\n}\n',NULL,'0',1),
(345,NULL,NULL,'DeclinedInvitations',1,1,1,2,2,2,1,-1,-1,0,NULL,'{\n    \"limit\": -1,\n    \"negated\": false,\n    \"rel\": 1,\n    \"subTerms\": [\n        {\n            \"cond\": 0,\n            \"key\": \"partstatus\",\n            \"negated\": false,\n            \"value\": \"pospiech-HD@t-online.de2\"\n        }\n    ]\n}\n',NULL,'0',1),
(347,NULL,NULL,'Last Search',1,1,1,2,2,2,1,-1,-1,0,NULL,'{\n    \"limit\": -1,\n    \"negated\": false,\n    \"rel\": 1,\n    \"subTerms\": [\n        {\n            \"negated\": false,\n            \"rel\": 1,\n            \"subTerms\": [\n                {\n                    \"cond\": 5,\n                    \"key\": \"subject\",\n                    \"negated\": false,\n                    \"value\": \"FeuerlÃ¶scher\"\n                }\n            ]\n        }\n    ]\n}\n',' RECURSIVE','64',1),
(358,'Lenovo_Rewards','1761994211863','Lenovo_Rewards',359,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(359,'Finanz',NULL,'Finanz',46,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(360,'DKB','1764065918436','DKB',359,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(361,'CreditAgricole','1758179699575','CreditAgricole',359,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(362,'VR_DD_Bautzen','1731853876463','VR_DD_Bautzen',359,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(363,'DeBeKa','1738662824298','DeBeKa',359,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(364,'PayPal','1766516395007','PayPal',359,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(365,'PostBank','1760709566062','PostBank',359,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(366,'Hoelschke','1758179367680','Hoelschke',60,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(368,'Lenono',NULL,'Lenono',46,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(369,'Radreisen','1763970934041','Radreisen',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(371,'LocalTransport','1767001957598','LocalTransport',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(372,'ResearchGate','1759397542458','ResearchGate',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(373,'Congstar','1765634358049','Congstar',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(374,'Docker','1761576145099','Docker',62,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(375,'LMX_trace','1765275650320','LMX_trace',62,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(379,'misc','1765031873854','misc',62,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(381,'FireExtinguisher','1765906377747','FireExtinguisher',58,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(387,'Rust','1766172622075','Rust',62,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(388,'temporary','0','temporary',48,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0),
(394,'nirwana',NULL,'nirwana',48,3,1,2,2,2,1,-1,-1,0,NULL,NULL,NULL,NULL,0);

--
-- Table structure for table `collectionattributetable`
--

CREATE TABLE `collectionattributetable` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `collectionId` bigint(20) NOT NULL,
  `type` longblob NOT NULL,
  `value` longblob DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `CollectionAttributeTable_collectionIndex` (`collectionId`),
  CONSTRAINT `collectionattributetable_ibfk_1` FOREIGN KEY (`collectionId`) REFERENCES `collectiontable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=195 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Dumping data for table `collectionattributetable`
--

INSERT INTO `collectionattributetable` VALUES
(15,9,'AccessRights','wcdWC'),
(18,11,'AccessRights','wcdW'),
(19,11,'ENTITYDISPLAY','(\"Personal Calendar\" \"office-calendar\" \"\" ())'),
(21,13,'AccessRights','C'),
(22,14,'noselect','1'),
(23,15,'ENTITYDISPLAY','(\"Inbox\" \"mail-folder-inbox\" \"\" ())'),
(24,15,'SpecialCollectionAttribute','inbox'),
(25,15,'collectionflags','\\Answered \\Flagged \\Deleted \\Seen \\Draft $FORWARDED DTAG_document DTAG_image $ATTACHMENT $SENT DTAG_audio DTAG_video $HasAttachment $HasNoAttachment $SIGNED $INVITATION \\*'),
(26,15,'highestmodseq','114370'),
(27,15,'uidnext','23714'),
(28,15,'uidvalidity','1125619489'),
(29,15,'collectionannotations',NULL),
(30,15,'collectionquota','354304 1073741824'),
(31,15,'imapquota','global quota % mailbox quota % local quota INBOX %%%% MESSAGE % 62000 %% STORAGE % 1048576 %%% LEVEL % 3 %% MAILBOX % 31 %%% MESSAGE % 62000 %%%% MESSAGE % 10 %% STORAGE % 346 %%% LEVEL % 1 %% MAILBOX % 2 %%% MESSAGE % 9'),
(32,21,'collectionflags','\\Answered \\Flagged \\Deleted \\Seen \\Draft $HasAttachment $ATTACHMENT $HasNoAttachment \\*'),
(33,21,'highestmodseq','58'),
(34,21,'uidnext','15'),
(35,21,'uidvalidity','1156153876'),
(36,21,'collectionannotations',NULL),
(37,21,'collectionquota','355328 1073741824'),
(38,21,'imapquota','global quota % mailbox quota % local quota INBOX.Drafts %%%% MESSAGE % 62000 %% STORAGE % 1048576 %%% LEVEL % 3 %% MAILBOX % 31 %%% MESSAGE % 62000 %%%% MESSAGE % 11 %% STORAGE % 347 %%% LEVEL % 1 %% MAILBOX % 2 %%% MESSAGE % 0'),
(39,20,'collectionflags','\\Answered \\Flagged \\Deleted \\Seen \\Draft \\*'),
(40,20,'highestmodseq','1'),
(41,20,'uidnext','1'),
(42,20,'uidvalidity','1389978560'),
(43,20,'collectionannotations',NULL),
(44,20,'collectionquota','355328 1073741824'),
(45,20,'imapquota','global quota % mailbox quota % local quota INBOX.Junk %%%% MESSAGE % 62000 %% STORAGE % 1048576 %%% LEVEL % 3 %% MAILBOX % 31 %%% MESSAGE % 62000 %%%% MESSAGE % 11 %% STORAGE % 347 %%% LEVEL % 1 %% MAILBOX % 2 %%% MESSAGE % 0'),
(46,19,'collectionflags','\\Answered \\Flagged \\Deleted \\Seen \\Draft DTAG_image DTAG_document $FORWARDED $HasNoAttachment $HasAttachment $ATTACHMENT $SIGNED $INVITATION $SENT \\*'),
(47,19,'highestmodseq','16251'),
(48,19,'uidnext','5762'),
(49,19,'uidvalidity','1362735290'),
(50,19,'collectionannotations',NULL),
(51,19,'collectionquota','354304 1073741824'),
(52,19,'imapquota','global quota % mailbox quota % local quota INBOX.Trash %%%% MESSAGE % 62000 %% STORAGE % 1048576 %%% LEVEL % 3 %% MAILBOX % 31 %%% MESSAGE % 62000 %%%% MESSAGE % 10 %% STORAGE % 346 %%% LEVEL % 1 %% MAILBOX % 2 %%% MESSAGE % 0'),
(53,18,'collectionflags','\\Answered \\Flagged \\Deleted \\Seen \\Draft \\*'),
(54,18,'highestmodseq','1'),
(55,18,'uidnext','1'),
(56,18,'uidvalidity','1389978562'),
(57,18,'collectionannotations',NULL),
(58,18,'collectionquota','354304 1073741824'),
(59,18,'imapquota','global quota % mailbox quota % local quota INBOX.Spam %%%% MESSAGE % 62000 %% STORAGE % 1048576 %%% LEVEL % 3 %% MAILBOX % 31 %%% MESSAGE % 62000 %%%% MESSAGE % 10 %% STORAGE % 346 %%% LEVEL % 1 %% MAILBOX % 2 %%% MESSAGE % 0'),
(60,17,'collectionflags','\\Answered \\Flagged \\Deleted \\Seen \\Draft DTAG_image $HasNoAttachment $HasAttachment DTAG_document $QUEUED $SENT $ATTACHMENT $SIGNED $FORWARDED \\*'),
(61,17,'highestmodseq','3093'),
(62,17,'uidnext','830'),
(63,17,'uidvalidity','1156153875'),
(64,17,'collectionannotations',NULL),
(65,17,'collectionquota','354304 1073741824'),
(66,17,'imapquota','global quota % mailbox quota % local quota INBOX.Sent %%%% MESSAGE % 62000 %% STORAGE % 1048576 %%% LEVEL % 3 %% MAILBOX % 31 %%% MESSAGE % 62000 %%%% MESSAGE % 10 %% STORAGE % 346 %%% LEVEL % 1 %% MAILBOX % 2 %%% MESSAGE % 1'),
(67,16,'collectionflags','\\Answered \\Flagged \\Deleted \\Seen \\Draft DTAG_image $FORWARDED DTAG_document $HasNoAttachment $HasAttachment \\*'),
(68,16,'highestmodseq','3891'),
(69,16,'uidnext','949'),
(70,16,'uidvalidity','1389978561'),
(71,16,'collectionannotations',NULL),
(72,16,'collectionquota','354304 1073741824'),
(73,16,'imapquota','global quota % mailbox quota %%%% MESSAGE % 62000 %% STORAGE % 1048576 %%% LEVEL % 3 %% MAILBOX % 31 %%%% MESSAGE % 10 %% STORAGE % 346 %%% LEVEL % 1 %% MAILBOX % 2'),
(103,19,'SpecialCollectionAttribute','trash'),
(104,19,'ENTITYDISPLAY','(\"\" \"user-trash\" \"\" ())'),
(105,17,'ENTITYDISPLAY','(\"\" \"mail-folder-sent\" \"\" ())'),
(106,17,'SpecialCollectionAttribute','sent-mail'),
(107,21,'ENTITYDISPLAY','(\"\" \"document-properties\" \"\" ())'),
(108,21,'SpecialCollectionAttribute','drafts'),
(109,36,'ENTITYDISPLAY','(\"\" \"document-new\" \"\" ())'),
(110,36,'SpecialCollectionAttribute','templates'),
(125,36,'collectionflags','\\Answered \\Flagged \\Deleted \\Seen \\Draft \\*'),
(126,36,'highestmodseq','1'),
(127,36,'uidnext','1'),
(128,36,'uidvalidity','1389978563'),
(129,36,'collectionannotations',NULL),
(130,36,'collectionquota','355328 1073741824'),
(131,36,'imapquota','global quota % mailbox quota % local quota INBOX.templates %%%% MESSAGE % 62000 %% STORAGE % 1048576 %%% LEVEL % 3 %% MAILBOX % 31 %%% MESSAGE % 62000 %%%% MESSAGE % 11 %% STORAGE % 347 %%% LEVEL % 1 %% MAILBOX % 2 %%% MESSAGE % 0'),
(132,46,'AccessRights','C'),
(133,48,'SpecialCollectionAttribute','inbox'),
(134,46,'ENTITYDISPLAY','(\"Local Folders\" \"folder\" \"\" ())'),
(135,46,'SpecialCollectionAttribute','local-mail'),
(136,47,'ENTITYDISPLAY','(\"drafts\" \"document-properties\" \"\" ())'),
(137,47,'SpecialCollectionAttribute','drafts'),
(138,49,'ENTITYDISPLAY','(\"outbox\" \"mail-folder-outbox\" \"\" ())'),
(139,49,'SpecialCollectionAttribute','outbox'),
(140,50,'ENTITYDISPLAY','(\"sent-mail\" \"mail-folder-sent\" \"\" ())'),
(141,50,'SpecialCollectionAttribute','sent-mail'),
(142,51,'ENTITYDISPLAY','(\"templates\" \"document-new\" \"\" ())'),
(143,51,'SpecialCollectionAttribute','templates'),
(144,52,'ENTITYDISPLAY','(\"trash\" \"user-trash\" \"\" ())'),
(145,52,'SpecialCollectionAttribute','trash'),
(146,344,'AccessRights','luD'),
(147,345,'AccessRights','luD'),
(148,344,'ENTITYDISPLAY','(\"Open Invitations\" \"\" \"\" ())'),
(149,344,'PERSISTENTSEARCH','(QUERYSTRING \"{\\n    \\\"limit\\\": -1,\\n    \\\"negated\\\": false,\\n    \\\"rel\\\": 1,\\n    \\\"subTerms\\\": [\\n        {\\n            \\\"cond\\\": 0,\\n            \\\"key\\\": \\\"partstatus\\\",\\n            \\\"negated\\\": false,\\n            \\\"value\\\": \\\"pospiech-HD@t-online.de0\\\"\\n        }\\n    ]\\n}\\n\" QUERYCOLLECTIONS (0))'),
(150,345,'ENTITYDISPLAY','(\"Declined Invitations\" \"\" \"\" ())'),
(151,345,'PERSISTENTSEARCH','(QUERYSTRING \"{\\n    \\\"limit\\\": -1,\\n    \\\"negated\\\": false,\\n    \\\"rel\\\": 1,\\n    \\\"subTerms\\\": [\\n        {\\n            \\\"cond\\\": 0,\\n            \\\"key\\\": \\\"partstatus\\\",\\n            \\\"negated\\\": false,\\n            \\\"value\\\": \\\"pospiech-HD@t-online.de2\\\"\\n        }\\n    ]\\n}\\n\" QUERYCOLLECTIONS (0))'),
(154,347,'AccessRights','luD'),
(155,347,'kmailsearchdescription','\0\0\0\0\0\0\0@\0\0\0C\0\0\0\0o\0r\0\0\0Subject\0\0\0\0c\0o\0n\0t\0a\0i\0n\0s\0\0\0\0F\0e\0u\0e\0r\0l\0�\0s\0c\0h\0e\0r\0\0\0\0'),
(156,347,'PERSISTENTSEARCH','(QUERYSTRING \"{\\n    \\\"limit\\\": -1,\\n    \\\"negated\\\": false,\\n    \\\"rel\\\": 1,\\n    \\\"subTerms\\\": [\\n        {\\n            \\\"negated\\\": false,\\n            \\\"rel\\\": 1,\\n            \\\"subTerms\\\": [\\n                {\\n                    \\\"cond\\\": 5,\\n                    \\\"key\\\": \\\"subject\\\",\\n                    \\\"negated\\\": false,\\n                    \\\"value\\\": \\\"FeuerlÃ¶scher\\\"\\n                }\\n            ]\\n        }\\n    ]\\n}\\n\" QUERYCOLLECTIONS (64) RECURSIVE)');

--
-- Table structure for table `mimetypetable`
--

CREATE TABLE `mimetypetable` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `name` varbinary(255) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=14 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Dumping data for table `mimetypetable`
--

INSERT INTO `mimetypetable` VALUES
(6,'application/x-vnd.akonadi.calendar.event'),
(9,'application/x-vnd.akonadi.calendar.freebusy'),
(8,'application/x-vnd.akonadi.calendar.journal'),
(7,'application/x-vnd.akonadi.calendar.todo'),
(13,'application/x-vnd.kde.alarm.active'),
(12,'application/x-vnd.kde.alarm.archived'),
(11,'application/x-vnd.kde.alarm.template'),
(4,'application/x-vnd.kde.contactgroup'),
(3,'inode/directory'),
(2,'message/rfc822'),
(5,'text/calendar'),
(1,'text/directory'),
(10,'text/x-vnd.akonadi.note');

--
-- Table structure for table `collectionmimetyperelation`
--

CREATE TABLE `collectionmimetyperelation` (
  `Collection_id` bigint(20) NOT NULL,
  `MimeType_id` bigint(20) NOT NULL,
  PRIMARY KEY (`Collection_id`,`MimeType_id`),
  KEY `CollectionMimeTypeRelation_Collection_idIndex` (`Collection_id`),
  KEY `CollectionMimeTypeRelation_MimeType_idIndex` (`MimeType_id`),
  CONSTRAINT `collectionmimetyperelation_ibfk_1` FOREIGN KEY (`Collection_id`) REFERENCES `collectiontable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `collectionmimetyperelation_ibfk_2` FOREIGN KEY (`MimeType_id`) REFERENCES `mimetypetable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;
--
-- Dumping data for table `collectionmimetyperelation`
--

INSERT INTO `collectionmimetyperelation` VALUES
(9,1),
(9,3),
(9,4),
(11,5),
(11,6),
(11,7),
(11,8),
(11,9),
(13,3),
(13,10),
(14,3),
(15,2),
(15,3),
(16,2),
(16,3),
(17,2),
(17,3),
(18,2),
(18,3),
(19,2),
(19,3),
(20,2),
(20,3),
(21,2),
(21,3),
(36,2),
(36,3),
(46,2),
(46,3),
(47,2),
(47,3),
(48,2),
(48,3),
(49,2),
(49,3),
(50,2),
(50,3),
(51,2),
(51,3),
(52,2),
(52,3),
(53,2),
(53,3),
(55,2),
(55,3),
(58,2),
(58,3),
(59,2),
(59,3),
(60,2),
(60,3),
(61,2),
(61,3),
(62,2),
(62,3),
(63,2),
(63,3),
(64,2),
(64,3),
(66,2),
(66,3),
(129,2),
(129,3),
(130,2),
(130,3),
(131,2),
(131,3),
(132,2),
(132,3),
(133,2),
(133,3),
(135,2),
(135,3),
(137,2),
(137,3),
(139,2),
(139,3),
(140,2),
(140,3),
(141,2),
(141,3),
(142,2),
(142,3),
(143,2),
(143,3),
(144,2),
(144,3),
(145,2),
(145,3),
(146,2),
(146,3),
(148,2),
(148,3),
(149,2),
(149,3),
(150,2),
(150,3),
(151,2),
(151,3),
(152,2),
(152,3),
(153,2),
(153,3),
(154,2),
(154,3),
(155,2),
(155,3),
(156,2),
(156,3),
(157,2),
(157,3),
(158,2),
(158,3),
(160,2),
(160,3),
(162,2),
(162,3),
(163,2),
(163,3),
(165,2),
(165,3),
(166,2),
(166,3),
(167,2),
(167,3),
(168,2),
(168,3),
(169,2),
(169,3),
(170,2),
(170,3),
(171,2),
(171,3),
(173,2),
(173,3),
(174,2),
(174,3),
(175,2),
(175,3),
(176,2),
(176,3),
(177,2),
(177,3),
(178,2),
(178,3),
(179,2),
(179,3),
(184,2),
(184,3),
(190,2),
(190,3),
(191,2),
(191,3),
(192,2),
(192,3),
(195,2),
(195,3),
(201,2),
(201,3),
(205,2),
(205,3),
(208,2),
(208,3),
(211,2),
(211,3),
(216,2),
(216,3),
(218,2),
(218,3),
(300,2),
(300,3),
(301,2),
(301,3),
(302,2),
(302,3),
(303,2),
(303,3),
(304,2),
(304,3),
(305,2),
(305,3),
(306,2),
(306,3),
(307,2),
(307,3),
(308,2),
(308,3),
(309,2),
(309,3),
(310,2),
(310,3),
(311,2),
(311,3),
(312,2),
(312,3),
(313,2),
(313,3),
(314,2),
(314,3),
(315,2),
(315,3),
(316,2),
(316,3),
(317,2),
(317,3),
(318,2),
(318,3),
(319,2),
(319,3),
(320,2),
(320,3),
(321,2),
(321,3),
(322,2),
(322,3),
(323,2),
(323,3),
(324,2),
(324,3),
(325,2),
(325,3),
(326,2),
(326,3),
(327,2),
(327,3),
(344,6),
(344,7),
(344,8),
(345,6),
(345,7),
(345,8),
(347,2),
(358,2),
(358,3),
(359,2),
(359,3),
(360,2),
(360,3),
(361,2),
(361,3),
(362,2),
(362,3),
(363,2),
(363,3),
(364,2),
(364,3),
(365,2),
(365,3),
(366,2),
(366,3),
(368,2),
(368,3),
(369,2),
(369,3),
(371,2),
(371,3),
(372,2),
(372,3),
(373,2),
(373,3),
(374,2),
(374,3),
(375,2),
(375,3),
(379,2),
(379,3),
(381,2),
(381,3),
(387,2),
(387,3),
(388,2),
(388,3),
(394,2),
(394,3);

--
-- Table structure for table `flagtable`
--

CREATE TABLE `flagtable` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `name` varbinary(255) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=17 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Dumping data for table `flagtable`
--

INSERT INTO `flagtable` VALUES
(4,'$ATTACHMENT'),
(13,'$ENCRYPTED'),
(15,'$ERROR'),
(6,'$FORWARDED'),
(5,'$HasAttachment'),
(3,'$HasNoAttachment'),
(2,'$IGNORED'),
(8,'$INVITATION'),
(10,'$QUEUED'),
(12,'$REPLIED'),
(11,'$SENT'),
(7,'$SIGNED'),
(9,'\\ANSWERED'),
(16,'\\DELETED'),
(14,'\\FLAGGED'),
(1,'\\SEEN');

--
-- Table structure for table `tagtypetable`
--

CREATE TABLE `tagtypetable` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `name` varbinary(255) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Dumping data for table `tagtypetable`
--

INSERT INTO `tagtypetable` VALUES
(1,'PLAIN');

--
-- Table structure for table `tagtable`
--

CREATE TABLE `tagtable` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `gid` varbinary(255) NOT NULL,
  `parentId` bigint(20) DEFAULT NULL,
  `typeId` bigint(20) DEFAULT 1,
  PRIMARY KEY (`id`),
  KEY `TagTable_parentIndex` (`parentId`),
  KEY `TagTable_typeIndex` (`typeId`),
  CONSTRAINT `tagtable_ibfk_2` FOREIGN KEY (`typeId`) REFERENCES `tagtypetable` (`id`) ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Table structure for table `pimitemtable`
--

CREATE TABLE `pimitemtable` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `rev` int(11) NOT NULL DEFAULT 0,
  `remoteId` varbinary(1024) DEFAULT NULL,
  `remoteRevision` varbinary(255) DEFAULT NULL,
  `gid` varbinary(255) DEFAULT NULL,
  `collectionId` bigint(20) DEFAULT NULL,
  `mimeTypeId` bigint(20) DEFAULT NULL,
  `datetime` timestamp NULL DEFAULT current_timestamp(),
  `atime` timestamp NULL DEFAULT current_timestamp(),
  `dirty` tinyint(1) DEFAULT NULL,
  `size` bigint(20) NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  KEY `PimItemTable_collectionIndex` (`collectionId`),
  KEY `PimItemTable_mimeTypeIndex` (`mimeTypeId`),
  KEY `PimItemTable_gidIndex` (`gid`),
  KEY `PimItemTable_ridIndex` (`remoteId`),
  KEY `PimItemTable_idSortIndex` (`id` DESC),
  CONSTRAINT `pimitemtable_ibfk_1` FOREIGN KEY (`collectionId`) REFERENCES `collectiontable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `pimitemtable_ibfk_2` FOREIGN KEY (`mimeTypeId`) REFERENCES `mimetypetable` (`id`) ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=50646 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci ROW_FORMAT=DYNAMIC;

--
-- Dumping data for table `pimitemtable`
--

INSERT INTO `pimitemtable` VALUES
(206,5,'1291727681.2020.4jNSG:2,S',NULL,NULL,388,2,'2025-12-30 15:15:44','2025-12-30 15:15:44',0,41572),
(1207,2,'1491255228.R505.helios:2,PS',NULL,NULL,66,2,'2025-12-30 15:16:02','2025-12-30 15:16:07',0,1600),
(1322,4,'1330783242.R2038.sirius:2,S',NULL,NULL,66,2,'2025-12-21 14:30:20','2025-12-21 14:30:25',0,39110),
(50377,2,'1766493447052.R371.helios:2,S',NULL,NULL,388,2,'2025-12-30 15:13:01','2025-12-30 15:13:01',0,1707),
(50628,1,'1767111571664.R424.helios',NULL,NULL,394,2,'2025-12-30 15:19:31','2025-12-30 15:19:31',0,208687),
(50638,0,NULL,NULL,NULL,394,2,'2025-12-30 15:19:02','2025-12-30 15:19:02',1,166999),
(50642,1,NULL,NULL,NULL,394,2,'2025-12-30 15:19:02','2025-12-30 15:19:02',1,83352),
(50643,1,NULL,NULL,NULL,394,2,'2025-12-30 15:19:02','2025-12-30 15:19:02',1,94077),
(50645,1,NULL,NULL,NULL,394,2,'2025-12-30 15:19:02','2025-12-30 15:19:02',1,2348),
(132632,1,NULL,NULL,NULL,19,2,'2026-01-29 09:30:18','2026-01-29 09:30:18',1,301521);

--
-- Table structure for table `collectionpimitemrelation`
--

CREATE TABLE `collectionpimitemrelation` (
  `Collection_id` bigint(20) NOT NULL,
  `PimItem_id` bigint(20) NOT NULL,
  PRIMARY KEY (`Collection_id`,`PimItem_id`),
  KEY `CollectionPimItemRelation_Collection_idIndex` (`Collection_id`),
  KEY `CollectionPimItemRelation_PimItem_idIndex` (`PimItem_id`),
  CONSTRAINT `collectionpimitemrelation_ibfk_1` FOREIGN KEY (`Collection_id`) REFERENCES `collectiontable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `collectionpimitemrelation_ibfk_2` FOREIGN KEY (`PimItem_id`) REFERENCES `pimitemtable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Table structure for table `parttypetable`
--

CREATE TABLE `parttypetable` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `name` varbinary(255) NOT NULL,
  `ns` varbinary(255) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `PartTypeTable_partTypeNameIndex` (`ns`,`name`)
) ENGINE=InnoDB AUTO_INCREMENT=19 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Dumping data for table `parttypetable`
--

INSERT INTO `parttypetable` VALUES
(13,'AddressAttribute','ATR'),
(5,'DKIMResultAttribute','ATR'),
(14,'DispatchModeAttribute','ATR'),
(11,'ENTITYDISPLAY','ATR'),
(9,'ErrorAttribute','ATR'),
(1,'HIDDEN','ATR'),
(6,'MDNStateAttribute','ATR'),
(7,'MessageDisplayFormatAttribute','ATR'),
(8,'ScamAttribute','ATR'),
(15,'SentActionAttribute','ATR'),
(16,'SentBehaviourAttribute','ATR'),
(17,'TransportAttribute','ATR'),
(18,'contactmetadata','ATR'),
(10,'entityannotations','ATR'),
(12,'pop3resourceattribute','ATR'),
(4,'ENVELOPE','PLD'),
(3,'HEAD','PLD'),
(2,'RFC822','PLD');

--
-- Table structure for table `parttable`
--

CREATE TABLE `parttable` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `pimItemId` bigint(20) NOT NULL,
  `partTypeId` bigint(20) NOT NULL,
  `data` longblob DEFAULT NULL,
  `datasize` bigint(20) NOT NULL,
  `version` int(11) DEFAULT 0,
  `storage` tinyint(4) DEFAULT 0,
  PRIMARY KEY (`id`),
  UNIQUE KEY `PartTable_pimItemIdTypeIndex` (`pimItemId`,`partTypeId`),
  KEY `PartTable_pimItemIdSortIndex` (`pimItemId` DESC),
  KEY `PartTable_partTypeIndex` (`partTypeId`),
  CONSTRAINT `parttable_ibfk_1` FOREIGN KEY (`pimItemId`) REFERENCES `pimitemtable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `parttable_ibfk_2` FOREIGN KEY (`partTypeId`) REFERENCES `parttypetable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=152670 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Dumping data for table `parttable`
--

INSERT INTO `parttable` VALUES
(616,206,2,NULL,0,1,0),
(617,206,3,NULL,0,1,0),
(618,206,4,'\0\0\0\0\0%���~(\0\0\0D\0A\0M\0D\0 \0P\0e\0r\0f\0o\0r\0m\0a\0n\0c\0e\0 \0w\0o\0r\0k\0s\0h\0o\0p\0 \0n\0e\0x\0t\0 \0w\0e\0e\0k����\0\0\0�\0<\0K\0L\01\0P\0R\00\03\0M\0B\07\07\06\05\0F\00\01\0F\04\0F\0F\09\08\00\06\02\0A\0D\0E\0A\00\08\0E\05\0F\09\05\01\0A\0@\0K\0L\01\0P\0R\00\03\0M\0B\07\07\06\05\0.\0a\0p\0c\0p\0r\0d\00\03\0.\0p\0r\0o\0d\0.\0o\0u\0t\0l\0o\0o\0k\0.\0c\0o\0m\0>����\0\0\0\0\0\0\0S\0i\0m\0o\0n\0 \0T\0h\0o\0m\0p\0s\0o\0n\02\0\0\0\0s\0t\0h\0o\0m\0p\0s\0o\0n\02\0\0\0\0l\0e\0n\0o\0v\0o\0.\0c\0o\0m\0\0\0\0\0\0\0\0\0\0\0\n\0\0\0\0F\0l\0o\0r\0i\0a\0n\0 \0Z\0i\0l\0l\0n\0e\0r\0\0\0\0f\0z\0i\0l\0l\0n\0e\0r\0\0\0\0l\0e\0n\0o\0v\0o\0.\0c\0o\0m\0\0\0\0A\0r\0j\0u\0n\0 \0R\0a\0m\0a\0s\0w\0a\0m\0i\0\0\0\0a\0r\0a\0m\0a\0s\0w\0a\0m\0i\0\0\0\0l\0e\0n\0o\0v\0o\0.\0c\0o\0m\0\0\0$\0C\0h\0r\0i\0s\0t\0o\0p\0h\0 \0P\0o\0s\0p\0i\0e\0c\0h\0\0\0\0c\0p\0o\0s\0p\0i\0e\0c\0h\0\0\0\0l\0e\0n\0o\0v\0o\0.\0c\0o\0m\0\0\0\0F\0l\0o\0r\0i\0a\0n\0 \0M\0e\0r\0z\0\0\0\n\0f\0m\0e\0r\0z\0\0\0\0l\0e\0n\0o\0v\0o\0.\0c\0o\0m\0\0\0\0H\0o\0l\0g\0e\0r\0 \0H\0o\0l\0t\0h\0o\0f\0f\0\0\0\0h\0h\0o\0l\0t\0h\0o\0f\0f\0\0\0\0l\0e\0n\0o\0v\0o\0.\0c\0o\0m\0\0\0 \0S\0i\0g\0r\0u\0n\0 \0E\0g\0g\0e\0r\0l\0i\0n\0g\0\0\0\0s\0e\0g\0g\0e\0r\0l\0i\0n\0g\0\0\0\0l\0e\0n\0o\0v\0o\0.\0c\0o\0m\0\0\0\0E\0r\0i\0c\0 \0M\0i\0c\0h\0e\0l\0\0\0\0e\0m\0i\0c\0h\0e\0l\0\0\0\0l\0e\0n\0o\0v\0o\0.\0c\0o\0m\0\0\0 \0O\0l\0i\0v\0i\0e\0r\0 \0L\0a\0g\0r\0a\0s\0s\0e\0\0\0\0o\0l\0a\0g\0r\0a\0s\0s\0e\0\0\0\0l\0e\0n\0o\0v\0o\0.\0c\0o\0m\0\0\0\0N\0i\0l\0s\0 \0S\0m\0e\0d\0s\0\0\0\0n\0s\0m\0e\0d\0s\0\0\0\0l\0e\0n\0o\0v\0o\0.\0c\0o\0m\0\0\0\0L\0u\0i\0s\0 \0C\0e\0b\0a\0m\0a\0n\0o\0s\0\0\0\0l\0c\0e\0b\0a\0m\0a\0n\0o\0s\0\0\0\0l\0e\0n\0o\0v\0o\0.\0c\0o\0m\0\0\0\0\0\0\0\0',1133,2,0),
(2892,1207,4,'\0\0\0\0\0%��_`\0\0 \0\0\00\0M\0e\0i\0n\0 \0H\0o\0t\0e\0l\0 \0a\0u\0f\0 \0T\0e\0n\0e\0r\0i\0f\0f\0a����\0\0\06\0<\08\05\02\07\00\08\07\0.\07\0E\0A\0i\0n\0b\0c\00\0l\04\0@\0h\0e\0l\0i\0o\0s\0>����\0\0\0\0\0\0,\0D\0r\0.\0 \0C\0h\0r\0i\0s\0t\0o\0p\0h\0 \0P\0o\0s\0p\0i\0e\0c\0h\0\0\0\0p\0o\0s\0p\0i\0e\0c\0h\0-\0H\0D\0\0\0\0t\0-\0o\0n\0l\0i\0n\0e\0.\0d\0e\0\0\0\0\0\0\0\0\0\0\0����\0\0\0\0g\0e\0s\0c\0h\0e\0.\0p\0o\0s\0p\0i\0e\0c\0h\0\0\0\Z\0t\0u\0-\0d\0r\0e\0s\0d\0e\0n\0.\0d\0e\0\0\0\0\0\0\0\0',327,2,0),
(2893,1207,3,NULL,0,1,0),
(3122,1322,4,'\0\0\0\0\0%x-G��\0\0\0\0\0&\0I\0n\0f\0o\0:\0 \0B\0i\0n\0 \0i\0m\0 \0S\0t\0r\0e\0s\0s����\0\0\0�\0<\0O\0F\0D\0C\0F\0B\0A\01\0D\01\0.\09\09\05\08\06\03\09\07\0-\0O\0N\0C\01\02\05\07\08\02\0D\0.\00\00\03\04\0B\0E\06\03\0-\0C\01\02\05\07\08\02\0D\0.\00\00\03\04\0D\0F\0D\0B\0@\0L\0o\0c\0a\0l\0D\0o\0m\0a\0i\0n\0>����\0\0\0\0\0\0\Z\0M\0a\0r\0t\0i\0n\0 \0R\0o\0o\0s\0e\0n\0\0\0\0M\0R\0O\0O\0S\0E\0N\0\0\0\0d\0e\0.\0i\0b\0m\0.\0c\0o\0m\0\0\0\0\0\0\Z\0M\0a\0r\0t\0i\0n\0 \0R\0o\0o\0s\0e\0n\0\0\0\0M\0R\0O\0O\0S\0E\0N\0\0\0\0d\0e\0.\0i\0b\0m\0.\0c\0o\0m\0\0\0\0\0\0\0\0\0\0\0S\0T\0G\0 \0H\0P\0C\0\0\0\0s\0t\0g\0_\0h\0p\0c\0\0\0$\0w\0w\0p\0d\0l\0.\0v\0n\0e\0t\0.\0i\0b\0m\0.\0c\0o\0m\0\0\0\0\0\0\0\0',457,2,0),
(3123,1322,3,NULL,0,1,0),
(14311,1207,2,NULL,0,1,0),
(20862,1322,2,NULL,0,1,0),
(151905,50377,4,'\0\0\0\0\0%�i���\0\0\0\0\0`\0E\0m\0a\0i\0l\0-\0A\0d\0r\0e\0s\0s\0e\0 \0f\0�\0r\0 \0V\0e\0r\0s\0e\0n\0d\0e\0n\0 \0v\0o\0n\0 \0S\0c\0h\0n\0e\0t\0t\0 \0R\0e\0c\0h\0n\0u\0n\0g����\0\0\06\0<\02\08\01\02\02\06\04\0.\0m\0v\0X\0U\0D\0I\08\0C\00\0e\0@\0h\0e\0l\0i\0o\0s\0>����\0\0\0\0\0\0,\0D\0r\0.\0 \0C\0h\0r\0i\0s\0t\0o\0p\0h\0 \0P\0o\0s\0p\0i\0e\0c\0h\0\0\0\0p\0o\0s\0p\0i\0e\0c\0h\0-\0H\0D\0\0\0\0t\0-\0o\0n\0l\0i\0n\0e\0.\0d\0e\0\0\0\0\0\0\0\0\0\0\0\0\0\0&\0T\0a\0n\0z\0s\0c\0h\0u\0l\0e\0 \0L\0i\0p\0p\0m\0a\0n\0n\0\0\0\0i\0n\0f\0o\0\0\0,\0t\0a\0n\0z\0s\0c\0h\0u\0l\0e\0-\0l\0i\0p\0p\0m\0a\0n\0n\0.\0d\0e\0\0\0\0\0\0\0\0',409,2,0),
(151906,50377,3,NULL,0,1,0),
(152024,50377,2,NULL,0,1,0),
(152611,50628,2,NULL,0,1,0),
(152612,50628,3,NULL,0,1,0),
(152613,50628,4,'\0\0\0\0\0%�n�y�\0\0\0L\0S\0i\0l\0v\0e\0s\0t\0e\0r\0-\0W\0e\0i\0n\0e\0 \0-\0 \0m\0i\0t\0 \0u\0n\0d\0 \0o\0h\0n\0e\0 \0A\0l\0k\0o\0h\0o\0l����\0\0\0�\0<\00\01\00\07\00\01\09\0b\06\05\0e\0d\0b\04\08\08\0-\07\0a\04\08\04\03\0d\08\0-\0e\0c\02\0b\0-\04\0d\0e\0a\0-\09\06\0c\00\0-\03\06\08\07\00\06\0c\01\0b\0c\00\0a\0-\00\00\00\00\00\00\0@\0e\0u\0-\0c\0e\0n\0t\0r\0a\0l\0-\01\0.\0a\0m\0a\0z\0o\0n\0s\0e\0s\0.\0c\0o\0m\0>����\0\0\0\0\0\0&\0J\0a\0c\0q\0u\0e\0s\0\'\0 \0W\0e\0i\0n\0-\0D\0e\0p\0o\0t\0\0\0\0n\0e\0w\0s\0\0\0\0i\0n\0f\0o\0.\0j\0a\0c\0q\0u\0e\0s\0.\0d\0e\0\0\0\0\0\0\0����\0\0\0\0k\0o\0n\0t\0a\0k\0t\0\0\0\0j\0a\0c\0q\0u\0e\0s\0.\0d\0e\0\0\0����\0\0\0\0p\0o\0s\0p\0i\0e\0c\0h\0-\0h\0d\0\0\0\0t\0-\0o\0n\0l\0i\0n\0e\0.\0d\0e\0\0\0\0\0\0\0\0',497,2,0),
(152641,50638,3,'Return-Path: <no-reply@stadt.wiesbaden.de>\nReceived: from mailin27.aul.t-online.de ([10.223.144.67])\n	by ehead26a17.aul.t-online.de with LMTP\n	id Pt57OZaBU2kiPAAAzmg42g\n	(envelope-from <no-reply@stadt.wiesbaden.de>); Tue, 30 Dec 2025 08:39:02 +0100\nReceived: from stadt.wiesbaden.de ([24.40.191.66]) by mailin27.mgt.mul.t-online.de\n	with (TLSv1.3:TLS_AES_256_GCM_SHA384 encrypted)\n	esmtp id 1vaUJg-0uW6E50; Tue, 30 Dec 2025 08:38:52 +0100\nDKIM-Signature: v=1; a=rsa-sha256; c=relaxed/simple; d=stadt.wiesbaden.de;\n	s=2025; t=1767080331;\n	bh=cO7R+kXTPPCTPc+tgpDRVuKp4yMeozojvYGlhCjiybI=;\n	h=Date:From:Reply-To:To:Subject:List-Unsubscribe:From;\n	b=B9toE/y+lv6sEm/m3Xnm1h8tsHpLtP8K3jQ2qaYxoaMFWElvZMwYH2ze9+IeXLIV/\n	 HBxQ9cok8hKAihcjtunO9sHB19QUqqWefYyNyrNBQaeFWuTcioDQzckQKX9vCxfUvE\n	 pmnSbAas3PxpTWV6n5X6OGg8V8xEK8LVedAdPS8r0o9bjZJmEiD7M9lXqwGcg6GKs8\n	 IYIPGJNBs1S/qJ01CxhqFRat6AQyzO4AGVx2lDcfjaODe6OanKtMcoSF0HuYKI9wq5\n	 vsczg+GOg7EvTbUJl6I+hRba2h/LJM9N6WIm1qD9dfPM8Gem9nNLMq3g8WeTFFDabH\n	 3BJp/2w+A0g9w==\nReceived: from ies-wi.joomo.spml.de (preview-wi.joomo.spml.de [10.30.1.51])\n	by stadt.wiesbaden.de (Postfix) with ESMTPS id 1AE0F140109\n	for <pospiech-hd@t-online.de>; Tue, 30 Dec 2025 08:38:51 +0100 (CET)\nReceived: from wiesbaden01-cms (localhost [127.0.0.1])\n	by ies-wi.joomo.spml.de (Postfix) with ESMTP id 9AF316A005D\n	for <pospiech-hd@t-online.de>; Tue, 30 Dec 2025 08:38:50 +0100 (CET)\nDate: Tue, 30 Dec 2025 08:38:50 +0100 (CET)\nFrom: Landeshauptstadt Wiesbaden <no-reply@stadt.wiesbaden.de>\nReply-To: no-reply@stadt.wiesbaden.de\nTo: Christoph Pospiech <pospiech-hd@t-online.de>\nMessage-ID: <726721990.20028.1767080330633@wiesbaden01-cms>\nSubject: Das gibt es Neues - Newsletter der Stadt Wiesbaden\nMIME-Version: 1.0\nContent-Type: multipart/related; \n	boundary=\"----=_Part_20026_601733989.1767080330631\"\nList-Unsubscribe: https://www.wiesbaden.de/newsletter.php?edit=66e61633d05442766be12de5a4c0580957cb61a7ccb73f09\nX-TOI-VIRUSSCAN: unchecked\nX-TOI-EXPURGATEID: 149288::1767080332-9F7F14FD-F2085298/3/8621109840 BULK NORMAL\nX-TOI-MSGID: cb2d2d23-e125-4ac3-9531-c4810707e005\nX-ENVELOPE-TO: <pospiech-hd@t-online.de>\nAuthentication-Results: mailin27.aul.t-online.de;\n	dkim=pass (2048-bit key; unprotected) header.d=stadt.wiesbaden.de header.i=@stadt.wiesbaden.de header.a=rsa-sha256 header.s=2025 header.b=B9toE/y+;\n	dkim-atps=neutral\n',2346,1,0),
(152642,50638,2,'152642_r0',163243,1,1),
(152643,50638,4,'\0\0\0\0\0%�p��\0\0\0\0\0d\0D\0a\0s\0 \0g\0i\0b\0t\0 \0e\0s\0 \0N\0e\0u\0e\0s\0 \0-\0 \0N\0e\0w\0s\0l\0e\0t\0t\0e\0r\0 \0d\0e\0r\0 \0S\0t\0a\0d\0t\0 \0W\0i\0e\0s\0b\0a\0d\0e\0n����\0\0\0^\0<\07\02\06\07\02\01\09\09\00\0.\02\00\00\02\08\0.\01\07\06\07\00\08\00\03\03\00\06\03\03\0@\0w\0i\0e\0s\0b\0a\0d\0e\0n\00\01\0-\0c\0m\0s\0>����\0\0\0\0\0\04\0L\0a\0n\0d\0e\0s\0h\0a\0u\0p\0t\0s\0t\0a\0d\0t\0 \0W\0i\0e\0s\0b\0a\0d\0e\0n\0\0\0\0n\0o\0-\0r\0e\0p\0l\0y\0\0\0$\0s\0t\0a\0d\0t\0.\0w\0i\0e\0s\0b\0a\0d\0e\0n\0.\0d\0e\0\0\0\0\0\0\0����\0\0\0\0n\0o\0-\0r\0e\0p\0l\0y\0\0\0$\0s\0t\0a\0d\0t\0.\0w\0i\0e\0s\0b\0a\0d\0e\0n\0.\0d\0e\0\0\0\0\0\0$\0C\0h\0r\0i\0s\0t\0o\0p\0h\0 \0P\0o\0s\0p\0i\0e\0c\0h\0\0\0\0p\0o\0s\0p\0i\0e\0c\0h\0-\0h\0d\0\0\0\0t\0-\0o\0n\0l\0i\0n\0e\0.\0d\0e\0\0\0\0\0\0\0\0',523,2,0),
(152653,50642,3,'Return-Path: <bounce+aa75e8.680c81-pospiech-hd=t-online.de@software-dealz.de>\nReceived: from mailin24.aul.t-online.de ([10.223.144.64])\n	by ehead26a17.aul.t-online.de with LMTP\n	id DlHsJC+yU2kBpgAAzmg42g\n	(envelope-from <bounce+aa75e8.680c81-pospiech-hd=t-online.de@software-dealz.de>); Tue, 30 Dec 2025 12:06:23 +0100\nAuthentication-Results: mailin24.mgt.mul.t-online.de;\n	dkim=pass (1024-bit key; unprotected) header.d=software-dealz.de header.i=@software-dealz.de header.a=rsa-sha256 header.s=k1 header.b=W63h+1K3;\n	dkim-atps=neutral\nReceived: from g116.gb126d70.use4.send.mailgun.net ([204.220.169.116]) by mailin24.mgt.mul.t-online.de\n	with (TLSv1.2:ECDHE-RSA-AES256-GCM-SHA384 encrypted)\n	esmtp id 1vaXYT-34njzp0; Tue, 30 Dec 2025 12:06:21 +0100\nDKIM-Signature: a=rsa-sha256; v=1; c=relaxed/relaxed; d=software-dealz.de; q=dns/txt; s=k1; t=1767092780; x=1767099980;\n h=Message-Id: List-Unsubscribe: List-Unsubscribe-Post: To: To: From: From: Subject: Subject: Content-Type: Mime-Version: Date: Sender: Sender;\n bh=i/JGv/NdDbHP5oP00mR5gWw4RzOhVEKglaIVAAetIrc=;\n b=W63h+1K3zO3pGP01kpmaFKYvDIwghMlKnaQVBMhXpJAzRwuByqTxEfRz6Kt43ZVXwlvsbQGiYQkf19H6H0LXA+aP80jk0YTSPskzokTMn23mezZuhgP9ujb7d9mCnePQ46S/qJTAm0ScxvtY28K32/FdVtEwkbrhggm2jXsemvg=\nX-Mailgun-Sid: WyI4OGZkNyIsInBvc3BpZWNoLWhkQHQtb25saW5lLmRlIiwiNjgwYzgxIl0=\nReceived: by 2718711a182051a598eeccd19db36d28781d915108bd6ae1f6149243be0314d8 with HTTP\n id 6953b17b5134cc5f218a25ff; Tue, 30 Dec 2025 11:03:22 GMT\nX-Mailgun-Sending-Ip-Pool-Name: GS4\nX-Mailgun-Sending-Ip-Pool: 6655a294343e6a40aeea8d63\nX-Mailgun-Sending-Ip: 204.220.169.116\nX-Mailgun-Batch-Id: 6953b17abad0d7fe95d4d887\nSender: support@software-dealz.de\nDate: Tue, 30 Dec 2025 11:03:22 +0000\nMime-Version: 1.0\nContent-Type: multipart/alternative;\n boundary=\"8154ceac6c9463c25df5781e190f3ace30d029a70a9340da3ddf904b7a9a\"\nSubject: =?UTF-8?q?=F0=9F=9A=80_LAST_SALE_-_Spare_bis_zu_80%_auf_Top-Software!?=\nFrom: Software-Dealz <support@software-dealz.de>\nTo: pospiech-hd@t-online.de\nX-Mailgun-Tag: 6953b14573811e426fc424d4_6953b16166b26373bcf15f9c\nX-Mailgun-Track: false\nList-Unsubscribe-Post: List-Unsubscribe=One-Click\nFeedback-Id: 6953b0d773811e426fc424d0:6556c55f4f8febe42c821e75:campaign:omnisend\nList-Unsubscribe: <https://wme.soundestlink.com/contactsPreferences/v2/unsubscribe/?b=6556c55f4f8febe42c821e75&c=681ee517cbd2f9ecb5b1c8e2&payload=f2IA8we8mEZeupj3kv3t5HRMHgK16DNCSavw6eDu36DpUFV6wd-fGcvFrSv7D6mIRD6V0sWbUGKRJ84gyFjyzK7d2nfzgFd7hMf29fjkflaHZyXJUkHNSwXWBETO9eqc_pih&headerUnsub=true>\nX-Mailgun-Variables: {\"brandID\": \"6556c55f4f8febe42c821e75\", \"contactID\":\n \"681ee517cbd2f9ecb5b1c8e2\", \"progressID\": \"6953b14573811e426fc424d4\"}\nMessage-Id: <20251230110322.4521c221870b1300@software-dealz.de>\nX-TOI-VIRUSSCAN: unchecked\nX-TOI-EXPURGATEID: 149288::1767092781-B0FF69B8-BFC8C5B4/35/100204 BULK NORMAL\nX-TOI-MSGID: 8b164b57-eda0-4d9f-8cab-63aa1e25d54b\nX-ENVELOPE-TO: <pospiech-hd@t-online.de>\nAuthentication-Results: mailin24.aul.t-online.de;\n	dkim=pass (1024-bit key; unprotected) header.d=software-dealz.de header.i=@software-dealz.de header.a=rsa-sha256 header.s=k1 header.b=W63h+1K3;\n	dkim-atps=neutral\n',3130,1,0),
(152654,50642,2,'152654_r0',79765,1,1),
(152655,50642,4,'\0\0\0\0\0%�p_T�\0\0\0b�=ހ\0 \0L\0A\0S\0T\0 \0S\0A\0L\0E\0 \0-\0 \0S\0p\0a\0r\0e\0 \0b\0i\0s\0 \0z\0u\0 \08\00\0%\0 \0a\0u\0f\0 \0T\0o\0p\0-\0S\0o\0f\0t\0w\0a\0r\0e\0!����\0\0\0f\0<\02\00\02\05\01\02\03\00\01\01\00\03\02\02\0.\04\05\02\01\0c\02\02\01\08\07\00\0b\01\03\00\00\0@\0s\0o\0f\0t\0w\0a\0r\0e\0-\0d\0e\0a\0l\0z\0.\0d\0e\0>����\0\0\0\0\0\0\0S\0o\0f\0t\0w\0a\0r\0e\0-\0D\0e\0a\0l\0z\0\0\0\0s\0u\0p\0p\0o\0r\0t\0\0\0\"\0s\0o\0f\0t\0w\0a\0r\0e\0-\0d\0e\0a\0l\0z\0.\0d\0e\0\0\0����\0\0\0\0s\0u\0p\0p\0o\0r\0t\0\0\0\"\0s\0o\0f\0t\0w\0a\0r\0e\0-\0d\0e\0a\0l\0z\0.\0d\0e\0\0\0\0\0\0\0����\0\0\0\0p\0o\0s\0p\0i\0e\0c\0h\0-\0h\0d\0\0\0\0t\0-\0o\0n\0l\0i\0n\0e\0.\0d\0e\0\0\0\0\0\0\0\0',457,2,0),
(152656,50643,3,'Return-Path: <tap@mkt.flytap.com>\nReceived: from mailin40.aul.t-online.de ([10.223.144.80])\n	by ehead26a17.aul.t-online.de with LMTP\n	id ObhaMcTUU2lhEAAAzmg42g\n	(envelope-from <tap@mkt.flytap.com>); Tue, 30 Dec 2025 14:33:56 +0100\nAuthentication-Results: mailin40.mgt.mul.t-online.de;\n	dkim=pass (2048-bit key; unprotected) header.d=mkt.flytap.com header.i=@mkt.flytap.com header.a=rsa-sha256 header.s=tappt header.b=nWaSh24V;\n	dkim-atps=neutral\nReceived: from r223.info.tapmilesandgo.com ([130.248.205.223]) by mailin40.mgt.mul.t-online.de\n	with (TLSv1.2:ECDHE-RSA-AES256-GCM-SHA384 encrypted)\n	esmtp id 1vaZrC-1tgenH0; Tue, 30 Dec 2025 14:33:50 +0100\nDKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed; d=mkt.flytap.com;\n	s=tappt; t=1767101630;\n	bh=R3s7PLE8lvWWFRgTh9uhBTYXH/eLBtq39I855wFm2Pw=;\n	h=From:Subject:Date:To:MIME-Version:Message-ID:List-Unsubscribe:\n	 Content-Type;\n	b=nWaSh24VKGQcejonFq84jg8KtUlOXHGaXL/sXEVcPL6KHIwzHzMlzgs0k93VD7kwH\n	 JVmv+pVF/IwjCkcpXGgL3pQH8PtnjS7Qu4VGUdlysKYAQid/6xsYqbH+kXLH1OdlCF\n	 0N1BZDuKHie/U5TpR2nFHFOl60tzjARhhZKidGE1qnNET35b+e+UOeqTnvkbcfCUhM\n	 jq89r9xnucOaIJTs6RQjezwgOUwZhf95yy/aWgFwL/sF4w+gjp3YI3O7KXg1PwmmuC\n	 4sZYGh7CvHKw38PSPwwZ+FJ67BDWwtysJIMf3YirmB11mt3lBFIjvmck+479LUqE91\n	 t9syXknVhIKRw==\nX-MSFBL: fNA/xwC3CL+NORnCIRcqg0yj6MA2O9L8JlWg31WvkTo=|eyJyY3B0X21ldGEiOns\n	gImluIjogInRhcHB0X21pZF9wcm9kMyIsICJyIjogInBvc3BpZWNoLWhkQHQtb25\n	saW5lLmRlIiwgIm0iOiAiMzA3NjM3NzY2IiwgImQiOiAiNTE2MzkxNiIsICJpIjo\n	gIkE3NjcyQkE4NUVDRDY0RTEwQTQ5NUZGNEBBZG9iZU9yZyIgfSwiciI6InBvc3B\n	pZWNoLWhkQHQtb25saW5lLmRlIiwiYiI6InRhcHB0XzEzMC4yNDguMjA1LjIyM19\n	wcm9kLm1hcmtldGluZ19jYjM4NmM4Zi01NWY1LTQ5YWEtYTIwNC1iMjlkZjM0MWM\n	yMzkiLCJnIjoicHJvZC5tYXJrZXRpbmdfY2IzODZjOGYtNTVmNS00OWFhLWEyMDQ\n	tYjI5ZGYzNDFjMjM5In0=\nReceived: from [10.1.1.6] ([10.1.1.6:46177] helo=r223.info.tapmilesandgo.com) by irl1-prod1-mta-i-0f68260768887617a (envelope-from <tap@mkt.flytap.com>) (ecelerity 4.4.1.20033 r(msys-ecelerity:tags/4.4.1.0^0)) with ESMTP id B5/4A-37634-EB4D3596; Tue, 30 Dec 2025 13:33:50 +0000\nFrom: \"TAP Air Portugal Newsletter\" <newsletter@mkt.flytap.com>\nSubject: =?utf-8?B?Q2hyaXN0b3BoLCBmcm9oZXMgbmV1ZXMgUmVpc2V6aWVsOiBmbA==?=\n =?utf-8?B?aWVnZW4gU2llIGFiIDE2OeKCrCDwn46J?=\nDate: Tue, 30 Dec 2025 13:33:49 GMT\nTo: <pospiech-hd@t-online.de>\nReply-To: \"TAP Air Portugal Newsletter\" <no-reply@tap.pt>\nMIME-Version: 1.0\nX-mailer: nlserver, Build 8.6.4\nMessage-ID: <NM612562E06004ECB8Ctappt_mid_prod3@mkt.flytap.com>\nList-Unsubscribe: <mailto: tap@mkt.flytap.com?subject=unsubscribe%3CNM612562E06004ECB8Ctappt_mid_prod3@mkt.flytap.com%3E>\nPrecedence: Bulk\nContent-Type: text/html;\n	charset=\"utf-8\"\nContent-Transfer-Encoding: quoted-printable\nX-TOI-VIRUSSCAN: unchecked\nX-TOI-EXPURGATEID: 149288::1767101630-09FF0151-D86996B7/1/8621352127 SUSPECT MAIL-COUNT\nX-TOI-MSGID: c19773fc-d683-4eae-8285-2b071c044313\nX-ENVELOPE-TO: <pospiech-hd@t-online.de>\nAuthentication-Results: mailin40.aul.t-online.de;\n	dkim=pass (2048-bit key; unprotected) header.d=mkt.flytap.com header.i=@mkt.flytap.com header.a=rsa-sha256 header.s=tappt header.b=nWaSh24V;\n	dkim-atps=neutral\n',3079,1,0),
(152657,50643,2,'152657_r0',90465,1,1),
(152658,50643,4,'\0\0\0\0\0%�p�H\0\0\0r\0C\0h\0r\0i\0s\0t\0o\0p\0h\0,\0 \0f\0r\0o\0h\0e\0s\0 \0n\0e\0u\0e\0s\0 \0R\0e\0i\0s\0e\0z\0i\0e\0l\0:\0 \0f\0l\0i\0e\0g\0e\0n\0 \0S\0i\0e\0 \0a\0b\0 \01\06\09 �\0 �<߉����\0\0\0f\0<\0N\0M\06\01\02\05\06\02\0E\00\06\00\00\04\0E\0C\0B\08\0C\0t\0a\0p\0p\0t\0_\0m\0i\0d\0_\0p\0r\0o\0d\03\0@\0m\0k\0t\0.\0f\0l\0y\0t\0a\0p\0.\0c\0o\0m\0>����\0\0\0\0\0\06\0T\0A\0P\0 \0A\0i\0r\0 \0P\0o\0r\0t\0u\0g\0a\0l\0 \0N\0e\0w\0s\0l\0e\0t\0t\0e\0r\0\0\0\0n\0e\0w\0s\0l\0e\0t\0t\0e\0r\0\0\0\0m\0k\0t\0.\0f\0l\0y\0t\0a\0p\0.\0c\0o\0m\0\0\0\0\0\0\0\0\0\06\0T\0A\0P\0 \0A\0i\0r\0 \0P\0o\0r\0t\0u\0g\0a\0l\0 \0N\0e\0w\0s\0l\0e\0t\0t\0e\0r\0\0\0\0n\0o\0-\0r\0e\0p\0l\0y\0\0\0\0t\0a\0p\0.\0p\0t\0\0\0����\0\0\0\0p\0o\0s\0p\0i\0e\0c\0h\0-\0h\0d\0\0\0\0t\0-\0o\0n\0l\0i\0n\0e\0.\0d\0e\0\0\0\0\0\0\0\0',533,2,0),
(152667,50645,3,'Return-Path: <pospiech-HD@t-online.de>\nReceived: from fwd80.aul.t-online.de ([10.223.144.106])\n	by ehead26a17.aul.t-online.de with LMTP\n	id y30cFhv7U2k0fgAAzmg42g\n	(envelope-from <pospiech-HD@t-online.de>); Tue, 30 Dec 2025 17:17:31 +0100\nReceived: from helios.localnet ([84.156.124.227]) by fwd80.t-online.de\n	with (TLSv1.3:TLS_AES_256_GCM_SHA384 encrypted)\n	esmtp id 1vacPX-2VeEM40; Tue, 30 Dec 2025 17:17:27 +0100\nFrom: \"Dr. Christoph Pospiech\" <pospiech-HD@t-online.de>\nTo: \"Dr. Christoph Pospiech\" <pospiech-HD@t-online.de>\nSubject: test email\nDate: Tue, 30 Dec 2025 17:17:27 +0100\nMessage-ID: <3461859.mvXUDI8C0e@helios>\nMIME-Version: 1.0\nContent-Transfer-Encoding: quoted-printable\nContent-Type: text/plain; charset=\"iso-8859-1\"\nX-TOI-VIRUSSCAN: unchecked\nX-TOI-EXPURGATEID: 150726::1767111447-02FD43AD-E90106B3/0/0 CLEAN NORMAL\nX-TOI-MSGID: 766f708f-b257-411d-8604-d506d2904d17\nX-ENVELOPE-TO: <pospiech-HD@t-online.de>\n',927,1,0),
(152668,50645,2,'Return-Path: <pospiech-HD@t-online.de>\nReceived: from fwd80.aul.t-online.de ([10.223.144.106])\n	by ehead26a17.aul.t-online.de with LMTP\n	id y30cFhv7U2k0fgAAzmg42g\n	(envelope-from <pospiech-HD@t-online.de>); Tue, 30 Dec 2025 17:17:31 +0100\nReceived: from helios.localnet ([84.156.124.227]) by fwd80.t-online.de\n	with (TLSv1.3:TLS_AES_256_GCM_SHA384 encrypted)\n	esmtp id 1vacPX-2VeEM40; Tue, 30 Dec 2025 17:17:27 +0100\nFrom: \"Dr. Christoph Pospiech\" <pospiech-HD@t-online.de>\nTo: \"Dr. Christoph Pospiech\" <pospiech-HD@t-online.de>\nSubject: test email\nDate: Tue, 30 Dec 2025 17:17:27 +0100\nMessage-ID: <3461859.mvXUDI8C0e@helios>\nMIME-Version: 1.0\nContent-Transfer-Encoding: quoted-printable\nContent-Type: text/plain; charset=\"iso-8859-1\"\nX-TOI-VIRUSSCAN: unchecked\nX-TOI-EXPURGATEID: 150726::1767111447-02FD43AD-E90106B3/0/0 CLEAN NORMAL\nX-TOI-MSGID: 766f708f-b257-411d-8604-d506d2904d17\nX-ENVELOPE-TO: <pospiech-HD@t-online.de>\n\nTest\n=2D-=20\nMit freundlichen Gr=FC=DFen/Kind regards\n\nDr. Christoph Pospiech\nPhone: +49-351 86269826\nMobile: +49-1511-910-4597\nE-Mail: pospiech-HD@t-online.de\n\n\n',1090,1,0),
(152669,50645,4,'\0\0\0\0\0%�p��X\0\0\0\0\0\0t\0e\0s\0t\0 \0e\0m\0a\0i\0l����\0\0\06\0<\03\04\06\01\08\05\09\0.\0m\0v\0X\0U\0D\0I\08\0C\00\0e\0@\0h\0e\0l\0i\0o\0s\0>����\0\0\0\0\0\0,\0D\0r\0.\0 \0C\0h\0r\0i\0s\0t\0o\0p\0h\0 \0P\0o\0s\0p\0i\0e\0c\0h\0\0\0\0p\0o\0s\0p\0i\0e\0c\0h\0-\0H\0D\0\0\0\0t\0-\0o\0n\0l\0i\0n\0e\0.\0d\0e\0\0\0\0\0\0\0\0\0\0\0\0\0\0,\0D\0r\0.\0 \0C\0h\0r\0i\0s\0t\0o\0p\0h\0 \0P\0o\0s\0p\0i\0e\0c\0h\0\0\0\0p\0o\0s\0p\0i\0e\0c\0h\0-\0H\0D\0\0\0\0t\0-\0o\0n\0l\0i\0n\0e\0.\0d\0e\0\0\0\0\0\0\0\0',331,2,0),
(422897,132632,2,NULL,0,1,0);

--
-- Table structure for table `pimitemflagrelation`
--

CREATE TABLE `pimitemflagrelation` (
  `PimItem_id` bigint(20) NOT NULL,
  `Flag_id` bigint(20) NOT NULL,
  PRIMARY KEY (`PimItem_id`,`Flag_id`),
  KEY `PimItemFlagRelation_PimItem_idIndex` (`PimItem_id`),
  KEY `PimItemFlagRelation_Flag_idIndex` (`Flag_id`),
  KEY `PimItemFlagRelation_pimItemIdSortIndex` (`PimItem_id` DESC),
  CONSTRAINT `pimitemflagrelation_ibfk_1` FOREIGN KEY (`PimItem_id`) REFERENCES `pimitemtable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `pimitemflagrelation_ibfk_2` FOREIGN KEY (`Flag_id`) REFERENCES `flagtable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Dumping data for table `pimitemflagrelation`
--

INSERT INTO `pimitemflagrelation` VALUES
(206,1),
(206,3),
(1207,1),
(1207,6),
(1322,1),
(1322,4),
(50377,1),
(50628,1),
(50628,3),
(50638,1),
(50638,3),
(50642,1),
(50642,3),
(50643,1),
(50643,3),
(50645,1),
(50645,3);

--
-- Table structure for table `pimitemtagrelation`
--

CREATE TABLE `pimitemtagrelation` (
  `PimItem_id` bigint(20) NOT NULL,
  `Tag_id` bigint(20) NOT NULL,
  PRIMARY KEY (`PimItem_id`,`Tag_id`),
  KEY `PimItemTagRelation_PimItem_idIndex` (`PimItem_id`),
  KEY `PimItemTagRelation_Tag_idIndex` (`Tag_id`),
  CONSTRAINT `pimitemtagrelation_ibfk_1` FOREIGN KEY (`PimItem_id`) REFERENCES `pimitemtable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `pimitemtagrelation_ibfk_2` FOREIGN KEY (`Tag_id`) REFERENCES `tagtable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Table structure for table `relationtypetable`
--

CREATE TABLE `relationtypetable` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `name` varbinary(255) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Dumping data for table `relationtypetable`
--

INSERT INTO `relationtypetable` VALUES
(1,'GENERIC');


--
-- Table structure for table `relationtable`
--

CREATE TABLE `relationtable` (
  `leftId` bigint(20) NOT NULL,
  `rightId` bigint(20) NOT NULL,
  `typeId` bigint(20) DEFAULT 1,
  `remoteId` varbinary(255) DEFAULT NULL,
  UNIQUE KEY `RelationTable_RelationIndex` (`leftId`,`rightId`,`typeId`),
  KEY `RelationTable_leftIndex` (`leftId`),
  KEY `RelationTable_rightIndex` (`rightId`),
  KEY `RelationTable_typeIndex` (`typeId`),
  CONSTRAINT `relationtable_ibfk_1` FOREIGN KEY (`leftId`) REFERENCES `pimitemtable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `relationtable_ibfk_2` FOREIGN KEY (`rightId`) REFERENCES `pimitemtable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `relationtable_ibfk_3` FOREIGN KEY (`typeId`) REFERENCES `relationtypetable` (`id`) ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Table structure for table `schemaversiontable`
--

CREATE TABLE `schemaversiontable` (
  `version` int(11) NOT NULL DEFAULT 0,
  `generation` int(11) NOT NULL DEFAULT 0
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Dumping data for table `schemaversiontable`
--

INSERT INTO `schemaversiontable` VALUES
(41,1744032983);

--
-- Table structure for table `tagattributetable`
--

CREATE TABLE `tagattributetable` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `tagId` bigint(20) NOT NULL,
  `type` longblob NOT NULL,
  `value` longblob DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `TagAttributeTable_tagIndex` (`tagId`),
  CONSTRAINT `tagattributetable_ibfk_1` FOREIGN KEY (`tagId`) REFERENCES `tagtable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;

--
-- Table structure for table `tagremoteidresourcerelationtable`
--

CREATE TABLE `tagremoteidresourcerelationtable` (
  `tagId` bigint(20) NOT NULL,
  `resourceId` bigint(20) NOT NULL,
  `remoteId` varbinary(255) NOT NULL,
  UNIQUE KEY `TagRemoteIdResourceRelationTable_TagAndResourceIndex` (`tagId`,`resourceId`),
  KEY `TagRemoteIdResourceRelationTable_tagIndex` (`tagId`),
  KEY `TagRemoteIdResourceRelationTable_resourceIndex` (`resourceId`),
  CONSTRAINT `tagremoteidresourcerelationtable_ibfk_1` FOREIGN KEY (`tagId`) REFERENCES `tagtable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `tagremoteidresourcerelationtable_ibfk_2` FOREIGN KEY (`resourceId`) REFERENCES `resourcetable` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;
