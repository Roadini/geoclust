use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use schema::gspots;
use schema::gspots::dsl::gspots as all_gspots;

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct GSpot {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub google_place_id: String,
    pub lat: f32,
    pub lng: f32,
    pub primary_type: String,
    pub secondary_type: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "gspots"]
pub struct NewGSpot {
    pub name: String,
    pub address: String,
    pub google_place_id: String,
    pub lat: f32,
    pub lng: f32,
    pub primary_type: String,
    pub secondary_type: String,
}

impl GSpot {
    pub fn show(id: i32, conn: &PgConnection) -> Vec<GSpot> {
        all_gspots
            .find(id)
            .load::<GSpot>(conn)
            .expect("Error loading List")
    }

    pub fn get_by_google_id(g_id: String, conn: &PgConnection) -> Vec<GSpot> {
        all_gspots
            .filter(gspots::google_place_id.eq(g_id))
            .load::<GSpot>(conn)
            .expect("Error loading lists by User")
    }

    pub fn all(conn: &PgConnection) -> Vec<GSpot> {
        all_gspots
            .order(gspots::id.desc())
            .load::<GSpot>(conn)
            .expect("error loading the lists")
    }

    //pub fn update_by_id(id: i32, conn: &PgConnection, list: NewList) -> bool {
    //    use schema::lists::dsl::{user_id as u, list_name as n};
    //    let NewList {
    //        user_id,
    //        list_name,
    //    } = list;

    //    diesel::update(all_lists.find(id))
    //        .set((u.eq(user_id), n.eq(list_name)))
    //        .get_result::<List>(conn)
    //        .is_ok()
    //}

    pub fn insert(gspot: NewGSpot, conn: &PgConnection) -> Vec<i32> {
        let inserted_id = diesel::insert_into(gspots::table)
            .values(&gspot)
            .returning(gspots::id)
            .get_results(conn);

        let inserted_id = match inserted_id {
            Ok(inserted_id) => inserted_id,
            Err(error) => panic!("There was a problem  {:?}", error),
        };

        return inserted_id;
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if GSpot::show(id, conn).is_empty() {
            return false;
        };
        diesel::delete(all_gspots.find(id)).execute(conn).is_ok()
    }

    // TODO ALL by Primary Type and Location

    //pub fn all_by_user(user_id: i32, conn: &PgConnection) -> Vec<List> {
    //    all_lists
    //        .filter(lists::user_id.eq(user_id))
    //        .load::<List>(conn)
    //        .expect("Error loading lists by User")
    //}
}
