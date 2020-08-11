#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate uuid;
extern crate chrono;
extern crate jwt;
extern crate hmac;
extern crate sha2;
extern crate bcrypt;

mod schema;

mod  models;
mod  auth;
mod  controllers;
mod  routes;

fn main() {
    models::establish_connection();
    rocket::ignite()
    .mount("/api/v1/user", routes![
        routes::index,
        routes::book_chef,
        routes::update_booking,
        routes::user_booking,
        routes::chef_booking,
        routes::chef_bank_detail,
        routes::create_bank_detail,
        routes::update_bank_detail,
        routes::delete_bank_detail,
        routes::post,
        routes::posts,
        routes::update_user_post,
        routes::user_like_post,
        routes::user_posts,
        routes::user_viewed_post,
        routes::delete_user_post,
        ])
    .mount("/api/v1/admin", routes![
        routes::index,
        routes::posts,  
        routes::delete_booking,
        routes::update_booking_status,
        routes::bookings,
        routes::bank_details
        ])
    .register(
        catchers![
        routes::not_found,
        routes::server_error,
        routes::bad_request,
        routes::unprocessable_entity,
        routes::not_authorised,
        routes::not_authoritative,
        routes::forbidden
        ]
    )
    .launch();
}
