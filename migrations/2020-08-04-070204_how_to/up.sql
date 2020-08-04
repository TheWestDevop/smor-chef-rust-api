-- Your SQL goes here
CREATE TABLE smor_how_to (
  id SERIAL PRIMARY KEY,
  post_id VARCHAR(255) NOT NULL,
  user_id VARCHAR(255) NOT NULL,
  author VARCHAR(255) NOT NULL,
  title_header VARCHAR(255) NOT NULL,
  body TEXT NOT NULL,
  featured_image VARCHAR(255) NOT NULL,
  likes INT NOT NUll DEFAULT 0,
  views INT NOT NUll DEFAULT 0,   
  created_at VARCHAR(255) NOT NULL,
  update_at VARCHAR(255) NOT NULL
);