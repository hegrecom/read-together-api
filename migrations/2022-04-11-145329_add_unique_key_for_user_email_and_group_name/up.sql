DROP INDEX `idx_email` ON `users`;
CREATE UNIQUE INDEX `idx_email` ON `users` (`email`);
CREATE UNIQUE INDEX `idx_name` ON `groups` (`name`);

