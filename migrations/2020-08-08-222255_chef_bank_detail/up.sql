-- Your SQL goes here
CREATE TABLE smor_chef_bank_detail (
  id SERIAL PRIMARY KEY,
  user_id VARCHAR(255) NOT NULL,
  bank_name VARCHAR(255) NOT NULL,
  account_number VARCHAR(255) NOT NULL,
  created_at VARCHAR(255) NOT NULL,
  update_at VARCHAR(255) NOT NULL
);