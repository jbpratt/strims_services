CREATE TABLE IF NOT EXISTS `streams` (
  -- Stream ID
  `id` INTEGER PRIMARY KEY,

  -- The streaming service to use for this stream
  `service` VARCHAR(255) NOT NULL,

  -- This user's channel name within the above service
  `channel` VARCHAR(255) NOT NULL,

  -- Used for the user's canonical stream URL: <strims.gg/username>
  `path` VARCHAR(255) REFERENCES `users` (`stream_path`) ON DELETE SET NULL ON UPDATE CASCADE,

  -- Used by an admin to hide a stream from the front page
  `hidden` TINYINT(1) DEFAULT 0,

  -- Used by an admin to set a stream as afk
  `afk` TINYINT(1) DEFAULT 0,

  -- Used by an admin to promote a stream on the front page
  `promoted` TINYINT(1) DEFAULT 0,

  -- Title returned by the streaming service
  `title` VARCHAR(255) NOT NULL,

  -- Thumbnail image URL provided by the streaming service
  `thumbnail` VARCHAR(255),

  -- 1 if reported online by the streaming service
  `live` TINYINT(1) DEFAULT 0,

  -- Number of viewers reported by the streaming service
  `viewers` INTEGER DEFAULT 0,

  -- When this stream was first accessed
  `created_at` DATETIME NOT NULL,

  -- The last time this stream was updated
  `updated_at` DATETIME NOT NULL,

  UNIQUE (`id`),
  UNIQUE (`channel`, `service`, `path`)
);
