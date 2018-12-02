use db::Conn as DbConn;
use models::visits::{NewVisit, Visit};
use rocket_contrib::json::Json;
use serde_json::Value;

#[post("/visits", format = "application/json", data = "<new_visit>")]
pub fn new(conn: DbConn, new_visit: Json<NewVisit>) -> Json<Value> {
    Json(json!({
        "status": Visit::insert(new_visit.into_inner(), &conn),
    }))
}

#[get("/visits/list/<list_id>", format = "application/json")]
pub fn show_by_list(conn: DbConn, list_id: i32) -> Json<Value> {
    let result = Visit::show_visits_by_list(list_id, &conn);
    let status = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result,
    }))
}

#[delete("/visits/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status = if Visit::delete_by_id(id, &conn) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
    }))
}
