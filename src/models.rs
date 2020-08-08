use dotenv::dotenv;
use std::env;
use uuid::Uuid;
use diesel::prelude::*;
use chrono::prelude::*;
use diesel::PgConnection;
use serde::{Serialize, Deserialize};

use crate::schema::*;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Error loading database. \n Database url is required!!! .");
    let db_connection = PgConnection::establish(&database_url).expect("error connecting to database.");
    return db_connection  
}


#[derive(Queryable,Serialize, Deserialize,Debug)]
pub struct Post{
    pub id: i32,
    pub post_id: String,
    pub user_id: String,
    pub author: String,
    pub title_header: String,
    pub body: String,
    pub featured_image: String,
    pub likes:i32,
    pub views:i32,
    pub created_at: String,
    pub update_at: String,
}
#[derive(Queryable,Serialize, Deserialize,Debug)]
pub struct Booking{
    pub id: i32,
    pub booking_id: String,
    pub user_id: String,
    pub chef_id: String,
    pub dish: String,
    pub dish_cost: String,
    pub dish_time_frame: String,
    pub more_detail: String,
    pub booking_status:i32,
    pub booking_location: String,
    pub created_at: String,
    pub update_at: String,
}
#[derive(Queryable,Serialize, Deserialize,Debug)]
pub struct ChefBankDetail{
    pub id: i32,
    pub user_id: String,
    pub bank_name: String,
    pub account_number: String,
    pub created_at: String,
    pub update_at: String,
}

#[derive(Insertable,Debug)]
#[table_name="smor_how_to"]
pub struct New_Post{
    pub post_id: String,
    pub user_id: String,
    pub author: String,
    pub title_header: String,
    pub body: String,
    pub featured_image: String,
    pub created_at: String,
    pub update_at: String,
}
impl New_Post {
    pub fn new(user_id: String,author: String,title_header: String,body: String,featured_image: String) -> New_Post {
        let created_at = Local::now().to_string();
         let update_at = Local::now().to_string();
         let post_id =  Uuid::new_v5(
             &Uuid::NAMESPACE_OID,
             format!("{}-{}-{}",user_id,title_header,created_at).to_string().as_bytes()
         ).to_string();
         New_Post{
            post_id,
            user_id,
            author,
            title_header,
            body,
            featured_image,
            created_at,
            update_at
         }
    }
}

#[derive(Insertable,Debug)]
#[table_name="smor_chefs_bookings"]
pub struct New_Booking{
    pub booking_id: String,
    pub user_id: String,
    pub chef_id: String,
    pub dish: String,
    pub dish_cost: String,
    pub dish_time_frame: String,
    pub more_detail: String,
    pub booking_location: String,
    pub created_at: String,
    pub update_at: String,
}
impl New_Booking {
    pub fn new(user_id: String,chef_id: String,dish: String,dish_cost: String,dish_time_frame: String,more_detail: String,booking_location: String) -> New_Booking {
         let created_at = Local::now().to_string();
         let update_at = Local::now().to_string();
         let booking_id =  Uuid::new_v5(
             &Uuid::NAMESPACE_OID,
             format!("{}-{}-{}",user_id,chef_id,created_at).to_string().as_bytes()
         ).to_string();
        New_Booking {
            booking_id,
            user_id,
            chef_id,
            dish,
            dish_cost,
            dish_time_frame,
            more_detail,
            booking_location,
            created_at,
            update_at
        }
    }
}
#[derive(Insertable,Debug)]
#[table_name="smor_chef_bank_detail"]
pub struct New_ChefBankDetail{
    pub user_id: String,
    pub bank_name: String,
    pub account_number: String,
    pub created_at: String,
    pub update_at: String,
}
impl New_ChefBankDetail {
    pub fn new(user_id: String,bank_name: String,account_number: String,) -> New_ChefBankDetail {
        let created_at = Local::now().to_string();
        let update_at = Local::now().to_string();
        
        New_ChefBankDetail{
            user_id,
            bank_name,
            account_number,
            created_at,
            update_at
         }
    }
}
#[derive(Identifiable,Debug)]
#[table_name="smor_how_to"]
pub struct Update_Post{
    pub id:i32,
    pub post_id: String,
    pub author: String,
    pub title_header: String,
    pub body: String,
    pub featured_image: String,
    pub update_at: String,
}
impl Update_Post {
    pub fn new(id:i32,post_id: String,author: String,title_header: String,body: String,featured_image: String,) -> Update_Post {
         let update_at = Local::now().to_string();
        Update_Post {
            id,
            post_id,
            author,
            title_header,
            body,
            featured_image,
            update_at
        } 
    }
}

#[derive(Identifiable,Debug)]
#[table_name="smor_chefs_bookings"]
pub struct Update_Booking{
    pub id:i32,
    pub booking_id: String,
    pub user_id: String,
    pub chef_id: String,
    pub dish: String,
    pub dish_cost: String,
    pub dish_time_frame: String,
    pub more_detail: String,
    pub update_at: String,
}
impl Update_Booking {
    pub fn new(id:i32,booking_id: String,user_id: String,chef_id: String,dish: String,dish_cost: String,dish_time_frame: String,more_detail: String) -> Update_Booking {
        let update_at = Local::now().to_string();
        Update_Booking{
            id,
            booking_id,
            user_id,
            chef_id,
            dish,
            dish_cost,
            dish_time_frame,
            more_detail,
            update_at
        }
    }
}

#[derive(Identifiable,Debug)]
#[table_name="smor_chef_bank_detail"]
pub struct Update_ChefBankDetail{
    pub id:i32,
    pub user_id: String,
    pub bank_name: String,
    pub account_number: String,
    pub update_at: String,
}
impl Update_ChefBankDetail {
    pub fn new(id:i32,user_id:String,bank_name: String,account_number: String) -> Update_ChefBankDetail {
         let update_at = Local::now().to_string();
        Update_ChefBankDetail {
            id,
            user_id,
            bank_name,
            account_number,
            update_at
        } 
    }
}

#[derive(FromForm,Debug)]
 pub struct NewPost{
    pub user_id: String,
    pub author: String,
    pub title_header: String,
    pub body: String,
    pub featured_image: String,
}
#[derive(FromForm,Debug)]
 pub struct NewBooking{
    pub user_id: String,
    pub chef_id: String,
    pub dish: String,
    pub dish_cost: String,
    pub dish_time_frame: String,
    pub more_detail: String,
    pub booking_location: String,
}
#[derive(FromForm,Debug)]
 pub struct NewChefBankDetail{
    pub user_id: String,
    pub bank_name: String,
    pub account_number: String,
}
#[derive(FromForm,Debug)]
 pub struct UpdatePost{
    pub id:i32,
    pub post_id: String,
    pub author: String,
    pub title_header: String,
    pub body: String,
    pub featured_image: String,
}
#[derive(FromForm,Debug)]
 pub struct UpdateBooking{
    pub id:i32,
    pub booking_id: String,
    pub user_id: String,
    pub chef_id: String,
    pub dish: String,
    pub dish_cost: String,
    pub dish_time_frame: String,
    pub more_detail: String,
}
#[derive(FromForm,Debug)]
 pub struct UpdateChefBankDetail{
    pub id:i32,
    pub user_id: String,
    pub bank_name: String,
    pub account_number: String,
}