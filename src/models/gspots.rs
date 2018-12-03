use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::gspots;
use schema::gspots::dsl::gspots as all_gspots;

#[derive(Serialize, Queryable,QueryableByName,Debug, Clone)]
#[table_name = "gspots"]
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

    pub fn insert(gspot: NewGSpot, conn: &PgConnection) -> Result<i32, i32> {

        if GSpot::get_by_google_id(gspot.google_place_id.to_string(), conn).is_empty() {
            let inserted_id = diesel::insert_into(gspots::table)
                .values(&gspot)
                .returning(gspots::id)
                .get_results(conn);
            let inserted_id = match inserted_id{
                Ok(inserted_id) => inserted_id,
                Err(error) => {
                    panic!("There was a problem  {:?}", error)
                },
            };

            return Ok(inserted_id[0]);
        }

        println!("Google Place already on the DB");
        return Err(0);

    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if GSpot::show(id, conn).is_empty() {
            return false;
        };
        diesel::delete(all_gspots.find(id)).execute(conn).is_ok()
    }

    pub fn get_city( lat: f32,lng: f32, conn: &PgConnection) -> Vec<GSpot>{
        let near : Vec<GSpot> = diesel::sql_query("SELECT * FROM gspots WHERE earth_box(ll_to_earth($1, $2), 10000)  @> ll_to_earth(gspots.lat, gspots.lng) AND gspots.primary_type = 'locality' AND gspots.secondary_type = 'political'")
            .bind::<diesel::sql_types::Float, _>(lat)
            .bind::<diesel::sql_types::Float, _>(lng)
            .load(conn)
            .expect("sou nabo como a merda");

        return near;
    }


    pub fn get_near(lat: f32, lng: f32, conn: &PgConnection) -> Vec<GSpot> {
        let near : Vec<GSpot> = diesel::sql_query("SELECT * FROM gspots WHERE earth_box(ll_to_earth($1,$2), 20) @> ll_to_earth(gspots.lat, gspots.lng)")
            .bind::<diesel::sql_types::Float, _>(lat)
            .bind::<diesel::sql_types::Float, _>(lng)
            .load(conn)
            .expect("sou nabo como a merda");

        return near;
    }


    pub fn get_by_type(lat: f32, lng: f32, t: &str, conn: &PgConnection) -> Vec<GSpot> {
        println!("Que Piça {}", t);
        let near : Vec<GSpot> = diesel::sql_query("SELECT * FROM gspots WHERE earth_box(ll_to_earth($1, $2), 4000)  @> ll_to_earth(gspots.lat, gspots.lng) AND ( gspots.primary_type = $3 OR gspots.secondary_type = $3)")
            .bind::<diesel::sql_types::Float, _>(lat)
            .bind::<diesel::sql_types::Float, _>(lng)
            .bind::<diesel::sql_types::Text, _>(t)
            .load(conn)
            .expect("sou nabo como a merda");

        return near;
    }

    pub fn change_spot(lat_n: f32, lng_n: f32, id_i: i32, conn: &PgConnection) -> Vec<GSpot> {
        let current = &GSpot::show(id_i, conn)[0];

        let GSpot {
            id,
            name,
            address,
            google_place_id,
            lat,
            lng,
            primary_type,
            secondary_type,
        } = current;

        let near : Vec<GSpot> = diesel::sql_query("SELECT * FROM gspots WHERE earth_box(ll_to_earth($1, $2), 5000)  @> ll_to_earth(gspots.lat, gspots.lng) AND gspots.primary_type = $3 AND gspots.secondary_type = $4 AND gspots.id != $5")
            .bind::<diesel::sql_types::Float, _>(lat_n)
            .bind::<diesel::sql_types::Float, _>(lng_n)
            .bind::<diesel::sql_types::Text, _>(primary_type)
            .bind::<diesel::sql_types::Text, _>(secondary_type)
            .bind::<diesel::sql_types::Integer, _>(id)
            .load(conn)
            .expect("sou nabo como a merda");

        return near;
    }
}
