-- PRAGMA foreign_keys=off;

-- BEGIN TRANSACTION;

DROP TABLE IF EXISTS _posts_old;

ALTER TABLE posts RENAME TO _posts_old;

CREATE TABLE posts
( 
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  published INTEGER NOT NULL
);

INSERT INTO posts (id, title, body, published)
  SELECT id, title, body, published
  FROM _posts_old;


DROP TABLE IF EXISTS _posts_old;

-- COMMIT;

-- PRAGMA foreign_keys=on;