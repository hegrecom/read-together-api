ALTER TABLE `group_members` ADD COLUMN `admin` tinyint(1) NOT NULL DEFAULT 0 COMMENT "whether a member is admin" AFTER `group_id`;
