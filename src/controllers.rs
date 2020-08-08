
use diesel::prelude::*;
use diesel::PgConnection;
use crate::models::*;
// use bcrypt::{DEFAULT_COST, hash,verify};
use rocket_contrib::json::{JsonValue};
//  use crate::auth::*;
use crate::schema;

pub fn all_chef_bank_details(con:PgConnection) ->  JsonValue {
    use schema::smor_chef_bank_detail::dsl::*;
    let results = smor_chef_bank_detail.load::<ChefBankDetail>(&con)
    .expect("Error loading all chef bank detail");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}
pub fn create_chef_bank_detail(con:PgConnection,detail:New_ChefBankDetail)-> JsonValue {
    use schema::smor_chef_bank_detail;
    let results = diesel::insert_into(smor_chef_bank_detail::table)
                                                .values(detail)
                                                .get_result::<ChefBankDetail>(&con)
                                                .expect("Error creating new chef bank detail");
    return json!({
                "status": true,
                "data":results
            })  
}
pub fn update_chef_bank_detail(con:PgConnection,detail:Update_ChefBankDetail) -> JsonValue {
    use schema::smor_chef_bank_detail::dsl::*;

    let results = diesel::update(smor_chef_bank_detail.filter(user_id.eq(detail.user_id)))
                                                .set((
                                                    bank_name.eq(&detail.bank_name),
                                                    account_number.eq(&detail.account_number),
                                                    update_at.eq(&detail.update_at),
                                                ))
                                                .get_result::<ChefBankDetail>(&con)
                                                .expect("Error updating chef bank detail");
    return json!({
                "status": true,
                "data":results
            })
}
pub fn delete_chef_bank_detail(con:PgConnection,uid:String) -> JsonValue {
    use schema::smor_chef_bank_detail::dsl::*;
    diesel::delete(smor_chef_bank_detail.filter(user_id.eq(uid)))
    .execute(&con)
        .expect("Error deleting chef bank detail");
    return json!({
            "status": true,
            "data":"Chef bank detail deleted successfully"
        })
}
pub fn get_chef_bank_detail(con:PgConnection,uid:String)->JsonValue{
    use schema::smor_chef_bank_detail::dsl::*;
    let results = smor_chef_bank_detail.filter(user_id.eq(&uid))
    .load::<ChefBankDetail>(&con)
    .expect("Error loading chef bank detail");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}
pub fn all_posts(con:PgConnection) ->  JsonValue {
    use schema::smor_how_to::dsl::*;
    let results = smor_how_to.load::<Post>(&con)
    .expect("Error loading all posts");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}
pub fn create_post(con:PgConnection,post:New_Post)-> JsonValue {
    use schema::smor_how_to;
    let results = diesel::insert_into(smor_how_to::table)
                                                .values(post)
                                                .get_result::<Post>(&con)
                                                .expect("Error creating new post");
    return json!({
                "status": true,
                "data":results
            })  
}
pub fn update_post(con:PgConnection,post:Update_Post) -> JsonValue {
    use schema::smor_how_to::dsl::*;

    let results = diesel::update(smor_how_to.filter(post_id.eq(post.post_id)))
                                                .set((
                                                    author.eq(&post.author),
                                                    title_header.eq(&post.title_header),
                                                    body.eq(&post.body),
                                                    featured_image.eq(&post.featured_image),
                                                    update_at.eq(&post.update_at),
                                                ))
                                                .get_result::<Post>(&con)
                                                .expect("Error updating post");
    return json!({
                "status": true,
                "data":results
            })
}
pub fn delete_post(con:PgConnection,pid:String) -> JsonValue {
    use schema::smor_how_to::dsl::*;
    diesel::delete(smor_how_to.filter(post_id.eq(pid)))
    .execute(&con)
        .expect("Error deleting post");
    return json!({
            "status": true,
            "data":"Post deleted successfully"
        })
}
pub fn get_user_post(con:PgConnection,uid:String)->JsonValue{
    use schema::smor_how_to::dsl::*;
    let results = smor_how_to.filter(user_id.eq(&uid))
    .load::<Post>(&con)
    .expect("Error loading all user how to");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}
pub fn like_post(con:PgConnection,pid:String) -> JsonValue{
    use schema::smor_how_to::dsl::*;

    let mut old_likes = smor_how_to.select(likes).filter(post_id.eq(&pid))
                                                .load::<i32>(&con).expect("Error unable to fetch post");
    old_likes[0] += 1;

    diesel::update(smor_how_to.filter(post_id.eq(&pid)))
                                                .set(
                                                    likes.eq(old_likes[0]),
                                                )
                                                .execute(&con)
                                                .expect("Error updating post like");
    json!({
                "status": true,
                "data":"Liked"
            })
}
pub fn post_viewed(con:PgConnection,pid:String) -> JsonValue{
    use schema::smor_how_to::dsl::*;

    let mut old_views = smor_how_to.select(views).filter(post_id.eq(&pid))
                                                .load::<i32>(&con).expect("Error unable to fetch post");
    old_views[0] += 1;

    diesel::update(smor_how_to.filter(post_id.eq(&pid)))
                                                .set(
                                                    views.eq(old_views[0]),
                                                )
                                                .execute(&con)
                                                .expect("Error updating post view count");
    json!({
                "status": true,
                "data":"Viewed"
            })
}

pub fn all_bookings(con:PgConnection) ->  JsonValue {
    use schema::smor_chefs_bookings::dsl::*;
    let results = smor_chefs_bookings.load::<Booking>(&con)
    .expect("Error loading all bookings");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}
pub fn create_chef_booking(con:PgConnection,booking:New_Booking)-> JsonValue {
    use schema::smor_chefs_bookings;
    let results = diesel::insert_into(smor_chefs_bookings::table)
                                                .values(booking)
                                                .get_result::<Booking>(&con)
                                                .expect("Error creating new booking");
    return json!({
                "status": true,
                "data":results
            })  
}
pub fn update_chef_booking(con:PgConnection,booking:Update_Booking) -> JsonValue {
    use schema::smor_chefs_bookings::dsl::*;

    let results = diesel::update(smor_chefs_bookings.filter(booking_id.eq(booking.booking_id)))
                                                .set((
                                                    dish.eq(&booking.dish),
                                                    dish_cost.eq(&booking.dish_cost),
                                                    dish_time_frame.eq(&booking.dish_time_frame),
                                                    more_detail.eq(&booking.more_detail),
                                                    update_at.eq(&booking.update_at),
                                                ))
                                                .get_result::<Booking>(&con)
                                                .expect("Error updating chef booking");
    return json!({
                "status": true,
                "data":results
            })
}
pub fn update_chef_booking_status(con:PgConnection,booked_id:String,status:i32) -> JsonValue {
    use schema::smor_chefs_bookings::dsl::*;
    diesel::update(smor_chefs_bookings.filter(booking_id.eq(&booked_id)))
                                                .set(
                                                    booking_status.eq(&status),
                                                    )
                                                .execute(&con)
                                                .expect("Error updating chef booking status");
    return json!({
                "status": true,
                "data":"Booking status updated"
            })
}
pub fn delete_chef_booking(con:PgConnection,booked_id:String) -> JsonValue {
    use schema::smor_chefs_bookings::dsl::*;
    diesel::delete(smor_chefs_bookings.filter(booking_id.eq(&booked_id)))
    .execute(&con)
        .expect("Error deleting chef booking");
    return json!({
            "status": true,
            "data":"Chef booking deleted successfully"
        })
}

pub fn get_user_booking(con:PgConnection,uid:String)->JsonValue{
    use schema::smor_chefs_bookings::dsl::*;
    let results = smor_chefs_bookings.filter(user_id.eq(&uid))
    .load::<Booking>(&con)
    .expect("Error loading all user bookings");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}

pub fn get_chef_booking(con:PgConnection,cid:String)->JsonValue{
    use schema::smor_chefs_bookings::dsl::*;
    let results = smor_chefs_bookings.filter(chef_id.eq(&cid))
    .load::<Booking>(&con)
    .expect("Error loading all chef bookings");
    // print!("query result  {:?}",results);
    return json!({
        "status": true,
        "data":results
    })
}
