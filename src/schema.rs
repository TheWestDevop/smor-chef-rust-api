table! {
    smor_chefs_bookings (id) {
        id -> Int4,
        booking_id -> Varchar,
        user_id -> Varchar,
        chef_id -> Varchar,
        dish -> Varchar,
        dish_cost -> Varchar,
        dish_time_frame -> Varchar,
        more_detail -> Text,
        booking_status -> Int4,
        booking_location -> Varchar,
        created_at -> Varchar,
        update_at -> Varchar,
    }
}

table! {
    smor_how_to (id) {
        id -> Int4,
        post_id -> Varchar,
        user_id -> Varchar,
        author -> Varchar,
        title_header -> Varchar,
        body -> Text,
        featured_image -> Varchar,
        likes -> Int4,
        views -> Int4,
        created_at -> Varchar,
        update_at -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    smor_chefs_bookings,
    smor_how_to,
);
