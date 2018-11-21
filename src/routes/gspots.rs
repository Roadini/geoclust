use db::Conn as DbConn;
use rocket_contrib::json::Json;
use models::gspots::{GSpot, NewGSpot};
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
pub fn get_by_coord(lat: f32, lng: f32, conn: DbConn) -> Json<Value> {

    let gspots = format!("Coordenadas. Lat: {} Lng: {}",lat, lng);

    Json(json!({
        "status": 200,
        "result": gspots,
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

//#[catch(404)]
//fn not_found() -> Json<Value> {
//    Json(json!({
//        "status": "error",
//        "reason": "Resource was not found"
//    }))
//}
