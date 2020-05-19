CREATE TABLE IF NOT EXISTS `banned_ip_ranges` (
  -- The start of the banned IP range
  `start` VARCHAR(39),

  -- The end of the banned IP range
  `end` VARCHAR(39),

  -- A note for additional context
  `note` VARCHAR(255),

  -- When this IP range was first set
  `created_at` DATETIME NOT NULL,

  -- The last time this IP range was updated
  `updated_at` DATETIME NOT NULL,

  PRIMARY KEY (`start`, `end`),
  UNIQUE (`start`, `end`)
);
