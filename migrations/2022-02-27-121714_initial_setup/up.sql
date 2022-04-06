CREATE TABLE `users`
(
    `id`            int(8) NOT NULL AUTO_INCREMENT,
    `email`         varchar(255) NOT NULL COMMENT "user's email for identification",
    `password`      varchar(255) NOT NULL COMMENT "user's encrypted password",
    `name`          varchar(255) NOT NULL COMMENT "user's displayed name",
    `created_at`    datetime(6) DEFAULT CURRENT_TIMESTAMP(6) COMMENT "record created time", 
    `updated_at`    datetime(6) DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6) COMMENT "record updated time", 
    PRIMARY KEY (`id`),
    KEY `idx_email` (`email`),
    KEY `idx_created_at` (`created_at`),
    KEY `idx_updated_at` (`updated_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

CREATE TABLE `user_tokens`
(
    `id`            int(8) NOT NULL AUTO_INCREMENT,
    `user_id`       int(8) NOT NULL COMMENT "associated user id",
    `token`         varchar(255) NOT NULL COMMENT "user token for identification",
    `created_at`    datetime(6) DEFAULT CURRENT_TIMESTAMP(6) COMMENT "record created time", 
    `updated_at`    datetime(6) DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6) COMMENT "record updated time", 
    PRIMARY KEY (`id`),
    KEY `idx_token` (`token`),
    KEY `idx_created_at` (`created_at`),
    KEY `idx_updated_at` (`updated_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

CREATE TABLE `groups`
(
    `id`            int(8) NOT NULL AUTO_INCREMENT,
    `name`          varchar(255) NOT NULL COMMENT "group's displayed name",
    `created_at`    datetime(6) DEFAULT CURRENT_TIMESTAMP(6) COMMENT "record created time", 
    `updated_at`    datetime(6) DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6) COMMENT "record updated time", 
    PRIMARY KEY (`id`),
    KEY `idx_created_at` (`created_at`),
    KEY `idx_updated_at` (`updated_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

CREATE TABLE `group_members`
(
    `id`            int(8) NOT NULL AUTO_INCREMENT,
    `user_id`       int(8) NOT NULL COMMENT "associated user id",
    `group_id`      int(8) NOT NULL COMMENT "associated group id",
    `created_at`    datetime(6) DEFAULT CURRENT_TIMESTAMP(6) COMMENT "record created time", 
    `updated_at`    datetime(6) DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6) COMMENT "record updated time", 
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_group_id` (`group_id`),
    KEY `idx_created_at` (`created_at`),
    KEY `idx_updated_at` (`updated_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

CREATE TABLE `plans`
(
    `id`            int(8) NOT NULL AUTO_INCREMENT,
    `group_id`      int(8) NOT NULL COMMENT "associated group id",
    `name`          varchar(255) NOT NULL COMMENT "plan's displayed name",
    `created_at`    datetime(6) DEFAULT CURRENT_TIMESTAMP(6) COMMENT "record created time", 
    `updated_at`    datetime(6) DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6) COMMENT "record updated time", 
    PRIMARY KEY (`id`),
    KEY `idx_group_id` (`group_id`),
    KEY `idx_created_at` (`created_at`),
    KEY `idx_updated_at` (`updated_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

CREATE TABLE `plans_participants`
(
    `id`            int(8) NOT NULL AUTO_INCREMENT,
    `user_id`       int(8) NOT NULL COMMENT "associated user id",
    `plan_id`       int(8) NOT NULL COMMENT "associated plan id",
    `created_at`    datetime(6) DEFAULT CURRENT_TIMESTAMP(6) COMMENT "record created time", 
    `updated_at`    datetime(6) DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6) COMMENT "record updated time", 
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_plan_id` (`plan_id`),
    KEY `idx_created_at` (`created_at`),
    KEY `idx_updated_at` (`updated_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

CREATE TABLE `plan_contents`
(
    `id`            int(8) NOT NULL AUTO_INCREMENT,
    `plan_id`       int(8) NOT NULL COMMENT "associated plan id",
    `sequence`      int(8) NOT NULL COMMENT "content's sequence in a plan",
    `content`       text   NOT NULL COMMENT "content",
    `created_at`    datetime(6) DEFAULT CURRENT_TIMESTAMP(6) COMMENT "record created time", 
    `updated_at`    datetime(6) DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6) COMMENT "record updated time", 
    PRIMARY KEY (`id`),
    KEY `idx_plan_id` (`plan_id`),
    KEY `idx_sequence` (`sequence`),
    KEY `idx_created_at` (`created_at`),
    KEY `idx_updated_at` (`updated_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

CREATE TABLE `plan_content_user_activities`
(
    `id`                int(8) NOT NULL AUTO_INCREMENT,
    `plan_content_id`   int(8) NOT NULL COMMENT "associated plan content id",
    `user_id`           int(8) NOT NULL COMMENT "associated user id",
    `activity`          enum("start", "read") NOT NULL COMMENT "user activity",
    `created_at`        datetime(6) DEFAULT CURRENT_TIMESTAMP(6) COMMENT "record created time", 
    `updated_at`        datetime(6) DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6) COMMENT "record updated time", 
    PRIMARY KEY (`id`),
    KEY `idx_plan_content_id` (`plan_content_id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_activity` (`activity`),
    KEY `idx_created_at` (`created_at`),
    KEY `idx_updated_at` (`updated_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

