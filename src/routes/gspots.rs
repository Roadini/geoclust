extern crate reqwest;

use dotenv::dotenv;
use std::env;
use db::Conn as DbConn;
use rocket_contrib::json::Json;
use models::gspots::{GSpot};
use models::google_responses::{Candidate, GoogleResponse};
use serde_json::Value;

#[get("/gspots", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let gspots = GSpot::all(&conn);

    Json(json!({
        "status": 200,
        "result": gspots,
    }))
}

#[get("/gspots?<lat>&<lng>", format = "application/json")]
pub fn get_possible_gspots( lat: f32, lng: f32, conn: DbConn) -> Json<Value> {

    // TODO: Make this right!
    let gspots = GSpot::all(&conn);

    Json(json!({
        "status": 200,
        "result": gspots,
    }))
}

#[get("/gspots/populate?<lat>&<lng>", format = "application/json")]
pub fn populate_by_coord(lat: f32, lng: f32, conn: DbConn) -> Json<Value> {
    println!("Coordenadas. Lat: {} Lng: {}",lat, lng);
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("set DATABASE_URL");


    let request_url = format!("https://maps.googleapis.com/maps/api/place/findplacefromtext/json?input=*\
            &inputtype=textquery\
            &fields=formatted_address,name,place_id,types\
            &locationbias=circle:50@{},{}\
            &key={}",lat, lng,api_key);


    let mut response = reqwest::get(&request_url)
        .expect("Que treta");

    let gr: GoogleResponse = response.json::<GoogleResponse>()
        .expect("Upsi Dupsi");

    for x in &gr.candidates {
        let mut candidate_name = x.name.as_str();
        println!("Adding token Candidate ... {}", candidate_name);
        GSpot::insert(Candidate::to_gspot(x), &conn);
    }

    Json(json!({
        "status": 200,
        "result": format!("{:#?}",gr.candidates[0].name),
    }))
}


#[get("/gspots/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result = GSpot::show(id, &conn);
    let status = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[delete("/gspots/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status = if GSpot::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

// MAGIC STUFF
#[get("/magic", format = "application/json")]
pub fn get_route_suggestion(conn: DbConn) -> Json<Value> {
    let gspots = GSpot::all(&conn);

    Json(json!({
        "status": 200,
        "result": gspots,
    }))
}

#[get("/magic?<type_of_place>", format = "application/json")]
pub fn change_route_suggestion(type_of_place: String, conn: DbConn) -> Json<Value> {
    println!("Tipo de lugar para alterar {}",type_of_place);
    let gspots = GSpot::all(&conn);

    Json(json!({
        "status": 200,
        "result": gspots.get(0),
    }))
}

//#[catch(404)]
//fn not_found() -> Json<Value> {
//    Json(json!({
//        "status": "error",
//        "reason": "Resource was not found"
//    }))
//}
