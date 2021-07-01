-- MySQL dump 10.13  Distrib 5.7.33, for linux-glibc2.12 (x86_64)
--
-- Host: 127.0.0.1    Database: admin
-- ------------------------------------------------------
-- Server version	5.7.33

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `admin`
--

DROP TABLE IF EXISTS `admin`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `admin` (
  `sqn` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '顺序编号，程序内部使用，代表唯一客户',
  `account` varchar(32) NOT NULL COMMENT '账号',
  `pass` varchar(32) NOT NULL COMMENT '口令',
  `status` tinyint(4) NOT NULL DEFAULT '0' COMMENT '状态，0~2->正常,禁用,注销',
  `name` varchar(32) NOT NULL DEFAULT '' COMMENT '真实姓名',
  `sex` tinyint(4) NOT NULL DEFAULT '0' COMMENT '性别，0,1->男,女',
  `mobile` varchar(32) NOT NULL COMMENT '电话号码',
  `email` varchar(32) NOT NULL DEFAULT '' COMMENT '电子邮件',
  `icon` varchar(256) NOT NULL DEFAULT '' COMMENT '头像',
  `remark` varchar(255) NOT NULL DEFAULT '' COMMENT '备注',
  `ctime` datetime NOT NULL COMMENT '注册时间',
  PRIMARY KEY (`sqn`),
  UNIQUE KEY `account` (`account`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COMMENT='管理员信息';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `admin`
--

LOCK TABLES `admin` WRITE;
/*!40000 ALTER TABLE `admin` DISABLE KEYS */;
INSERT INTO `admin` VALUES (1,'admin','888888',0,'admin',1,'13512345678','','','','2021-01-01 00:00:00');
/*!40000 ALTER TABLE `admin` ENABLE KEYS */;
UNLOCK TABLES;

DROP TABLE IF EXISTS `service`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `service` (
  `sqn` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '序号',
  `wechat` varchar(32) DEFAULT NULL COMMENT '微信客服',
  `mobile` varchar(32) DEFAULT NULL COMMENT '电话客服',
  `address` varchar(255) DEFAULT NULL COMMENT '地址',
  PRIMARY KEY (`sqn`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `service`
--

LOCK TABLES `service` WRITE;
/*!40000 ALTER TABLE `service` DISABLE KEYS */;
INSERT INTO `service` VALUES (1,'wechat001','13512345678','深圳市');
/*!40000 ALTER TABLE `service` ENABLE KEYS */;
UNLOCK TABLES;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2021-06-30 17:22:23
