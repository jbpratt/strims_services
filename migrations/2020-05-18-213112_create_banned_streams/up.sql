CREATE TABLE IF NOT EXISTS `banned_streams` (
  -- Channel for the stream ("jbpratt")
  `channel` VARCHAR(255) NOT NULL NOT NULL,

  -- Service for the stream ("twitch")
  `service` VARCHAR(255) NOT NULL NOT NULL,

  -- Context for the ban
  `reason` VARCHAR(255),

  -- When this stream ban was first set
  `created_at` DATETIME NOT NULL,

  -- The last time this stream ban was updated
  `updated_at` DATETIME NOT NULL,

  UNIQUE (`channel`, `service`),
  PRIMARY KEY (`channel`, `service`)
);
