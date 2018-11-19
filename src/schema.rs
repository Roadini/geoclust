table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        published -> Bool,
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
    lists,
    spatial_ref_sys,
    visits,
);
