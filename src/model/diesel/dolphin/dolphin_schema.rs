table! {
    apps (id) {
        id -> Int4,
        app_name -> Varchar,
        remark -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        user_count -> Int4,
        online_status -> Int4,
        online_time -> Nullable<Int8>,
        app_abbr -> Varchar,
        app_id -> Varchar,
        app_tag -> Nullable<Varchar>,
        auth_mode -> Int2,
        product_id -> Int4,
    }
}

table! {
    iap_product (id) {
        id -> Int8,
        product_id -> Int4,
        product_type -> Int4,
        online_status -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
        product_title -> Varchar,
        description -> Varchar,
        price -> Numeric,
        raw_price -> Numeric,
        currency_code -> Nullable<Varchar>,
        app_id -> Varchar,
        sort -> Int4,
        deleted -> Int4,
        amount -> Int4,
        period -> Int4,
    }
}

table! {
    oauth2_refresh_token (id) {
        id -> Int8,
        refresh_token -> Varchar,
        user_id -> Int8,
        expire_date -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        replaced_by -> Nullable<Varchar>,
        revoked_by_ip -> Nullable<Varchar>,
        revoked_date -> Nullable<Varchar>,
        device_id -> Varchar,
        app_type -> Nullable<Int4>,
        auth_mode -> Nullable<Int4>,
        app_id -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int8,
        nickname -> Varchar,
        avatar_url -> Nullable<Varchar>,
        phone -> Varchar,
        updated_time -> Int8,
        created_time -> Int8,
        salt -> Varchar,
        pwd -> Varchar,
        sex -> Nullable<Int4>,
        level_type -> Nullable<Varchar>,
        phone_region -> Nullable<Varchar>,
        country_code -> Nullable<Varchar>,
        user_status -> Int4,
        last_login_time -> Nullable<Int8>,
        first_login_time -> Nullable<Int8>,
        app_id -> Varchar,
        register_time -> Int8,
        apple_iap_product_id -> Nullable<Varchar>,
        auto_renew_product_expire_time_ms -> Nullable<Int8>,
        is_guest -> Int4,
        product_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    apps,
    iap_product,
    oauth2_refresh_token,
    users,
);
