table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        published -> Bool,
    }
}

table! {
    gspots (id) {
        id -> Int4,
        name -> Varchar,
        address -> Varchar,
        google_place_id -> Varchar,
        lat -> Float4,
        lng -> Float4,
        primary_type -> Varchar,
        secondary_type -> Varchar,
    }
}

table! {
    lists (id) {
        id -> Int4,
        user_id -> Int4,
        list_name -> Varchar,
    }
}

table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

table! {
    visits (id) {
        id -> Int4,
        list_id -> Int4,
        internal_id_place -> Int4,
        review -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    books,
    gspots,
    lists,
    spatial_ref_sys,
    visits,
);
