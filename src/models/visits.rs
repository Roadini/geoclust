use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::visits;
use schema::visits::dsl::visits as all_visits;

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Visit {
    pub id: i32,
    pub list_id: i32,
    pub internal_id_place: i32,
    pub review: String,
    // TODO: Add Created @
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "visits"]
pub struct NewVisit {
    pub list_id: i32,
    pub internal_id_place: i32,
    pub review: String,
}

impl Visit {
    pub fn show_visits_by_list(list_id: i32, conn: &PgConnection) -> Vec<Visit> {
        all_visits
            .filter(visits::list_id.eq(list_id))
            .load::<Visit>(conn)
            .expect("Error loading List")
    }

    pub fn insert(visit: NewVisit, conn: &PgConnection) -> bool {
        diesel::insert_into(visits::table)
            .values(&visit)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(all_visits.find(id)).execute(conn).is_ok()
    }
}
