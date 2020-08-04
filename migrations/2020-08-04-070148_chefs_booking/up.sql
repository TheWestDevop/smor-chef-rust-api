-- Your SQL goes here
CREATE TABLE smor_chefs_bookings (
  id SERIAL PRIMARY KEY,
  booking_id VARCHAR(255) NOT NULL,
  user_id VARCHAR(255) NOT NULL,
  chef_id VARCHAR(255) NOT NULL,
  dish VARCHAR(255) NOT NULL,
  dish_cost VARCHAR(255) NOT NULL,
  dish_time_frame VARCHAR(255) NOT NULL,  
  more_detail TEXT NOT NULL,
  booking_status INT NOT NUll DEFAULT 0,
  booking_location VARCHAR(255) NOT NULL,   
  created_at VARCHAR(255) NOT NULL,
  update_at VARCHAR(255) NOT NULL
);