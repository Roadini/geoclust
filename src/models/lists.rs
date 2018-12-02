use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use schema::lists;
use schema::lists::dsl::lists as all_lists;

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct List {
    pub id: i32,
    pub user_id: i32,
    pub list_name: String,
    // TODO: Add Created @
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "lists"]
pub struct NewList {
    pub user_id: i32,
    pub list_name: String,
}

impl List {
    pub fn show(id: i32, conn: &PgConnection) -> Vec<List> {
        all_lists
            .find(id)
            .load::<List>(conn)
            .expect("Error loading List")
    }

    pub fn all(conn: &PgConnection) -> Vec<List> {
        all_lists
            .order(lists::id.desc())
            .load::<List>(conn)
            .expect("error loading the lists")
    }

    pub fn update_by_id(id: i32, conn: &PgConnection, list: NewList) -> bool {
        use schema::lists::dsl::{list_name as n, user_id as u};
        let NewList { user_id, list_name } = list;

        diesel::update(all_lists.find(id))
            .set((u.eq(user_id), n.eq(list_name)))
            .get_result::<List>(conn)
            .is_ok()
    }

    pub fn insert(list: NewList, conn: &PgConnection) -> bool {
        diesel::insert_into(lists::table)
            .values(&list)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if List::show(id, conn).is_empty() {
            return false;
        };
        diesel::delete(all_lists.find(id)).execute(conn).is_ok()
    }

    pub fn all_by_user(user_id: i32, conn: &PgConnection) -> Vec<List> {
        all_lists
            .filter(lists::user_id.eq(user_id))
            .load::<List>(conn)
            .expect("Error loading lists by User")
    }
}
