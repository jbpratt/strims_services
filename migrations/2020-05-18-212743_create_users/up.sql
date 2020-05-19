CREATE TABLE IF NOT EXISTS `users` (
  -- User ID
  `id` CHAR(36) NOT NULL PRIMARY KEY,

  -- Twitch ID
  `twitch_id` UNSIGNED BIGINT NOT NULL,

  -- The display name used in chat
  `name` VARCHAR(32) NOT NULL,

  -- Used for the user's canonical stream URL: <strims.gg/stream_path>
  `stream_path` VARCHAR(255) NOT NULL,

  -- The streaming service to use for this user's stream
  `service` VARCHAR(255) NOT NULL,

  -- This user's channel name within the above service
  `channel` VARCHAR(255) NOT NULL,

  -- The last IP address that this user logged in from.
  `last_ip` VARCHAR(255) NOT NULL,

  -- The date of last login for this user.
  `last_seen` DATETIME NOT NULL,

  -- 1 if the user prefers the chat on the left side of the screen, otherwise 0
  `left_chat` TINYINT(1) DEFAULT 0,

  -- 1 if the user is banned, otherwise 0
  `is_banned` TINYINT(1) NOT NULL DEFAULT 0,

  -- The reason for this user's ban, if any
  `ban_reason` VARCHAR(255),

  -- When this user was created
  `created_at` DATETIME NOT NULL,

  -- The last time this user was updated.
  `updated_at` DATETIME NOT NULL,

  -- 1 if the user is an admin, otherwise 0
  `is_admin` TINYINT(1) DEFAULT 0,

  UNIQUE (`id`),
  UNIQUE (`twitch_id`),
  UNIQUE (`stream_path`),
  UNIQUE (`name`)
);


