table! {
    app_map (id) {
        id -> Int8,
        app_id -> Varchar,
        third_app_id -> Varchar,
        third_channel -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
        login_redirect_url -> Varchar,
        login_success_redirect_url -> Varchar,
        app_private_key -> Varchar,
        app_public_key -> Varchar,
        payed_redirect_url -> Nullable<Varchar>,
        notify_url -> Nullable<Varchar>,
        qr_pay_model -> Int2,
        app_secret -> Nullable<Varchar>,
        app_private_key_pkcs1 -> Varchar,
        app_public_key_pkcs1 -> Varchar,
        alipay_public_key -> Varchar,
    }
}

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
    order_items (id) {
        order_id -> Varchar,
        iap_product_id -> Int8,
        quantity -> Int4,
        price -> Numeric,
        created_time -> Int8,
        updated_time -> Int8,
        id -> Int8,
    }
}

table! {
    orders (id) {
        id -> Int8,
        user_id -> Int8,
        total_price -> Numeric,
        order_status -> Int4,
        third_app_id -> Varchar,
        app_id -> Varchar,
        pay_channel -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
        qr_pay_model -> Int2,
        subject -> Varchar,
        product_code -> Varchar,
        order_id -> Varchar,
        seller_id -> Varchar,
        deleted -> Int2,
        paied_amount -> Int8,
    }
}

table! {
    payments (id) {
        id -> Int8,
        payment_id -> Varchar,
        order_id -> Varchar,
        amount -> Numeric,
        status -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
    }
}

table! {
    sms_config (id) {
        id -> Int8,
        access_key_id -> Varchar,
        access_key_secret -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        server_region -> Nullable<Varchar>,
        sign_name -> Varchar,
        app_id -> Varchar,
    }
}

table! {
    sms_log (id) {
        id -> Int8,
        service -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        text -> Nullable<Varchar>,
        template_code -> Varchar,
        phone -> Varchar,
        received_at -> Nullable<Varchar>,
        request_id -> Nullable<Varchar>,
        biz_id -> Nullable<Varchar>,
    }
}

table! {
    sms_template (id) {
        id -> Int8,
        biz_code -> Varchar,
        sms_code -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        app_id -> Varchar,
    }
}

table! {
    user_sub (id) {
        id -> Int8,
        app_id -> Varchar,
        product_id -> Int4,
        iap_product_id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        user_id -> Int8,
        sub_start_time -> Int8,
        sub_end_time -> Int8,
        enabled -> Int2,
        order_id -> Varchar,
        sub_start -> Timestamptz,
        sub_end -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int8,
        nickname -> Varchar,
        avatar_url -> Varchar,
        phone -> Varchar,
        updated_time -> Int8,
        created_time -> Int8,
        salt -> Varchar,
        pwd -> Varchar,
        sex -> Int4,
        level_type -> Varchar,
        phone_region -> Varchar,
        country_code -> Varchar,
        user_status -> Int4,
        last_login_time -> Nullable<Int8>,
        first_login_time -> Nullable<Int8>,
        app_id -> Varchar,
        register_time -> Int8,
        apple_iap_product_id -> Nullable<Varchar>,
        auto_renew_product_expire_time_ms -> Nullable<Int8>,
        is_guest -> Int4,
        product_id -> Int4,
        register_ip -> Varchar,
        reg_ip -> Nullable<Inet>,
    }
}

allow_tables_to_appear_in_same_query!(
    app_map,
    apps,
    iap_product,
    oauth2_refresh_token,
    order_items,
    orders,
    payments,
    sms_config,
    sms_log,
    sms_template,
    user_sub,
    users,
);
