use db::Conn as DbConn;
use rocket_contrib::json::Json;
use models::lists::{List, NewList};
use serde_json::Value;

#[get("/lists", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let lists = List::all(&conn);

    Json(json!({
        "status": 200,
        "result": lists,
    }))
}

#[post("/lists", format = "application/json", data = "<new_list>")]
pub fn new(conn: DbConn, new_list: Json<NewList>) -> Json<Value> {
    Json(json!({
        "status": List::insert(new_list.into_inner(), &conn),
        "result": List::all(&conn).first(),
    }))
}

#[get("/list/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result = List::show(id, &conn);
    let status = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put("/list/<id>", format = "application/json", data = "<list>")]
pub fn update(conn: DbConn, id: i32, list: Json<NewList>) -> Json<Value> {
    let status = if List::update_by_id(id, &conn, list.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/list/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status = if List::delete_by_id(id, &conn) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))

}

#[get("/lists/user/<user_id>", format = "application/json")]
pub fn author(user_id: i32, conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": List::all_by_user(user_id, &conn),
    }))
}


//#[catch(404)]
//fn not_found() -> Json<Value> {
//    Json(json!({
//        "status": "error",
//        "reason": "Resource was not found"
//    }))
//}
//needs to be reworked for current version of rocket.
