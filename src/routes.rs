use crate::controllers::*;
use crate::models::*;
use crate::auth::*;

use rocket_contrib::json::{JsonValue};
use rocket::request::Form;


#[get("/",)]
pub fn index() -> JsonValue {
    json!({
        "status":true,
        "message":"Welcome smorfarm chef services"
    })
}

#[get("/all/booking",)]
pub fn bookings(_auth:NormalAdminApiKey) -> JsonValue { 
    let connect = establish_connection();
   return all_bookings(connect);
    
}

#[get("/all/user/<uid>/booking",)]
pub fn user_booking(uid:String,_auth:UserApiKey) -> JsonValue { 
    let connect = establish_connection();
   return get_user_booking(connect,uid);
    
}

#[get("/all/chef/<uid>/booking")]
pub fn chef_booking(uid:String,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
   return get_chef_booking(connect,uid);
}

#[post("/create/booking",data="<data>")]
pub fn book_chef(data:Form<NewBooking>,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
    let booking = New_Booking::new(
        data.user_id.to_string(),
        data.chef_id.to_string(),
        data.dish.to_string(),
        data.dish_cost.to_string(),
        data.dish_time_frame.to_string(),
        data.more_detail.to_string(),
        data.booking_location.to_string()
    );
   return create_chef_booking(connect,booking);
}

#[put("/update/booking",data="<data>")]
pub fn update_booking(data:Form<UpdateBooking>,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
    let booking = Update_Booking::new(
        data.id,
        data.booking_id.to_string(),
        data.user_id.to_string(),
        data.chef_id.to_string(),
        data.dish.to_string(),
        data.dish_cost.to_string(),
        data.dish_time_frame.to_string(),
        data.more_detail.to_string()
    );
   return update_chef_booking(connect,booking);
}

#[put("/update/booking/<booking_id>/<status>")]
pub fn update_booking_status(booking_id:String,status:i32,_auth:NormalAdminApiKey) -> JsonValue {
    let connect = establish_connection();
   return update_chef_booking_status(connect,booking_id,status);
}

#[delete("/delete/booking/<booking_id>")]
pub fn delete_booking(booking_id:String,_auth:NormalAdminApiKey) -> JsonValue {
    let connect = establish_connection();
   return delete_chef_booking(connect,booking_id);
}

#[get("/all/chefs/bank/detail",)]
pub fn bank_details(_auth:NormalAdminApiKey) -> JsonValue { 
    let connect = establish_connection();
   return all_chef_bank_details(connect);
    
}

#[get("/get/user/<uid>/bank/detail")]
pub fn chef_bank_detail(uid:String,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
   return get_chef_bank_detail(connect,uid);
}

#[post("/create/chef/bank/detail",data="<data>")]
pub fn create_bank_detail(data:Form<NewChefBankDetail>,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
    let new_detail = New_ChefBankDetail::new(
        data.user_id.to_string(),
        data.bank_name.to_string(),
        data.account_number.to_string()
    );
   return create_chef_bank_detail(connect,new_detail);
}

#[put("/update/chef/bank/detail",data="<data>")]
pub fn update_bank_detail(data:Form<UpdateChefBankDetail>,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
    let detail = Update_ChefBankDetail::new(
        data.id,
        data.user_id.to_string(),
        data.bank_name.to_string(),
        data.account_number.to_string(),
    );
   return update_chef_bank_detail(connect,detail);
}

#[delete("/delete/bank/detail/<user_id>")]
pub fn delete_bank_detail(user_id:String,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
   return delete_chef_bank_detail(connect,user_id);
}

#[get("/all/posts",)]
pub fn posts(_auth:UserApiKey) -> JsonValue { 
    let connect = establish_connection();
   return all_posts(connect);
    
}

#[get("/get/user/<uid>/post")]
pub fn user_posts(uid:String,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
   return get_user_post(connect,uid);
}

#[get("/like/post/<pid>")]
pub fn user_like_post(pid:String,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
   return like_post(connect,pid);
}

#[get("/post/<pid>/viewed")]
pub fn user_viewed_post(pid:String,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
   return post_viewed(connect,pid);
}

#[post("/create/post",data="<data>")]
pub fn post(data:Form<NewPost>,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
    let new_post = New_Post::new(
        data.user_id.to_string(),
        data.author.to_string(),
        data.title_header.to_string(),
        data.body.to_string(),
        data.featured_image.to_string()
    );
   return create_post(connect,new_post);
}

#[put("/update/post",data="<data>")]
pub fn update_user_post(data:Form<UpdatePost>,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
    let post = Update_Post::new(
        data.id,
        data.post_id.to_string(),
        data.author.to_string(),
        data.title_header.to_string(),
        data.body.to_string(),
        data.featured_image.to_string()
    );
   return update_post(connect,post);
}

#[delete("/delete/post/<post_id>")]
pub fn delete_user_post(post_id:String,_auth:UserApiKey) -> JsonValue {
    let connect = establish_connection();
   return delete_post(connect,post_id);
}






#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": false,
        "message": "Nothing found."
    })
}
#[catch(401)]
pub fn not_authorised() -> JsonValue {
    json!({
        "status": false,
        "message": "The request requires authentication."
    })
}
#[catch(403)]
pub fn forbidden() -> JsonValue {
    json!({
        "status": false,
        "message": "Whoops! Looks like you are forbidden from accessing this service, contact admin."
    })
}
#[catch(203)]
pub fn not_authoritative() -> JsonValue {
    json!({
        "status": false,
        "message": "non-authoritative token given."
    })
}
#[catch(500)]
pub fn server_error() -> JsonValue {
    json!({
        "status": false,
        "message": "Whoops! Looks like we messed up."
    })
}
#[catch(400)]
pub fn bad_request() -> JsonValue {
    json!({
        "status": false,
        "message": "Whoops! Looks like you send a bad request."
    })
}
#[catch(422)]
pub fn unprocessable_entity() -> JsonValue {
    json!({
        "status": false,
        "message": "Whoops! Looks like you send a bad request."
    })
}