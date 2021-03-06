extern crate reqwest;

use dotenv::dotenv;
use std::env;
use db::Conn as DbConn;
use rocket_contrib::json::Json;
use models::gspots::{GSpot};
use models::google_responses::{Candidate, GoogleResponse};
use serde_json::Value;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;

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

    let city = GSpot::get_city(lat, lng, &conn);
    let gspots = GSpot::get_near(lat, lng, &conn);

    Json(json!({
        "status": 200,
        "address": city[0].address,
        "result": gspots,
    }))
}

#[get("/gspots/populate?<lat>&<lng>", format = "application/json")]
pub fn populate_by_coord(lat: f32, lng: f32, conn: DbConn) -> Json<Value> {

    let viking_names: HashSet<&'static str> =
        [   "amusement_park", "aquarium", "art_gallery", "bakery",
            "bar","book_store","bowling_alley","cafe","casino",
            "church","city_hall","library","liquor_store", "meal_delivery",
            "meal_takeaway", "museum", "night_club", "restaurant", "shopping_mall",
            "spa", "stadium", "zoo", "point_of_interest", "establishment", "food", "locality","political"].iter().cloned().collect();

    println!("Coordenadas. Lat: {} Lng: {}",lat, lng);

    dotenv().ok();
    let api_key = env::var("API_KEY").expect("set DATABASE_URL");


    let request_url = format!("https://maps.googleapis.com/maps/api/place/findplacefromtext/json?input=*\
            &inputtype=textquery\
            &fields=formatted_address,geometry,id,name,permanently_closed,photos,place_id,types\
            &locationbias=circle:50@{},{}\
            &key={}",lat, lng,api_key);

    let mut response = reqwest::get(&request_url)
        .expect("Que treta");

    let gr: GoogleResponse = response.json::<GoogleResponse>()
        .expect("Upsi Dupsi");

    'outer : for x in &gr.candidates {
        let mut candidate_name = x.name.as_str();
        println!("{}",candidate_name);
        let mut types = &x.types;

        for x in types {
            println!("{}", x);
            if !viking_names.contains(x.as_str()) {
                println!("We dont care about this place");
                continue 'outer;
            }
        }

        let mut inserted_id : Result<i32, i32>;
        println!("Adding token Candidate ... {}", candidate_name);
        inserted_id = GSpot::insert(Candidate::to_gspot(x), &conn);

        if( inserted_id.is_ok()){
            populate_photos_by_coord(Candidate::get_photo_id(x), inserted_id.unwrap());
        }
    };

    Json(json!({
        "status": 200,
    }))
}

pub fn populate_photos_by_coord(photo_id: String, place_internal_id: i32) {

    if(photo_id == "error"){
        return;
    }

    println!("photo_id : {} place_internal_id: {}", photo_id, place_internal_id);

    dotenv().ok();
    let api_key = env::var("API_KEY").expect("set DATABASE_URL");

    let request_url = format!("https://maps.googleapis.com/maps/api/place/photo? \
            &photoreference={}\
            &maxwidth=400\
            &key={}",photo_id, api_key);

    let mut response = reqwest::get(&request_url)
        .expect("Que treta");

    // TODO:: FIX
    //let mut extension = "jpeg".to_string();
    //if response.status().is_success() {

    //    let len = response.headers()["CONTENT_TYPE"].to_str().unwrap();

    //    println!("{}",len);

    //    let splits :Vec<&str> = len.split("/").collect();

    //    extension = splits[1].to_string();
    //}

    let file_name = format!("public/{}.{}", place_internal_id.to_string(), "jpeg");

    let path = Path::new(&file_name);

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create File"),

        Ok(file) => file,
    };

    let mut buffer = Vec::new();

    match response.read_to_end(&mut buffer){
        Err(why) => panic!("Something went Wrong Reading Response"),

        Ok(_) => println!("Successfully read Response"),
    };

    match file.write_all(&buffer) {
        Err(why) => panic!("Couldn't write to File"),

        Ok(_) => println!("Successfully wrote to File"),
    };
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
#[get("/magic?<lat>&<lng>&<user>", format = "application/json")]
pub fn get_route_suggestion(lat: f32, lng: f32, user:i32, conn: DbConn) -> Json<Value> {

    let mut vec = Vec::new();

    // NOT VERY NICE BUT I DON'T WANT THE GRAB AND GO TO SHOW
    let restaurant = GSpot::get_by_type(lat, lng, "restaurant",&conn).remove(1);
    vec.push(restaurant);

    let mut museums = GSpot::get_by_type(lat, lng, "museum",&conn);
    vec.push(museums.remove(0));
    vec.push(museums.remove(1));

    let cafe = GSpot::get_by_type(lat, lng, "cafe",&conn).remove(1);
    vec.push(cafe);

    Json(json!({
        "status": 200,
        "result": vec,
    }))
}

#[get("/magic/change?<lat>&<lng>&<place_id>", format = "application/json")]
pub fn change_route_suggestion(lat: f32, lng: f32 , place_id: i32, conn: DbConn) -> Json<Value> {
    let gspots = GSpot::change_spot(lat, lng, place_id, &conn);

    Json(json!({
        "status": 200,
        "result": gspots,
    }))
}
