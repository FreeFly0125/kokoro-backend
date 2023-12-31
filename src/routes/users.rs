use crate::config;
use rocket_contrib::json::{Json, JsonValue};

use rocket::http::Status;

use crate::models;
use models::users::{NewUser, UpdateUser, User};

use crate::database;
use database::users as action;

#[get("/users")]
pub fn view_all_users(connection: config::Connection) -> JsonValue {
    let result = action::query_view_all_users(&connection);
    json!({ "users": result })
}

#[get("/user/<user_id>")]
pub fn find_user(user_id: i32, connection: config::Connection) -> Option<JsonValue> {
    action::query_find_user(user_id, &connection).map(|user| json!({ "users": user }))
}

#[post("/user", data = "<user_data>")]
pub fn create_user(user_data: Json<NewUser>, connection: config::Connection) -> Json<User> {
    let new_user = user_data.into_inner();
    let name = new_user.name;
    let email = new_user.email;
    let password = new_user.password;

    Json(action::query_create_user(
        &connection,
        name,
        email,
        password,
    ))
}

#[put("/user/<user_id>", data = "<user_data>")]
pub fn update_user(
    user_id: i32,
    user_data: Json<UpdateUser>,
    connection: config::Connection,
) -> String {
    let query = action::query_update_user(user_id, user_data.into_inner(), &connection);
    match query {
        true => format!("User has been succesfully updated"),
        false => format!("Failed to update user"),
    }
}

#[delete("/user/<user_id>")]
pub fn delete_user(user_id: i32, connection: config::Connection) -> String {
    let result = action::query_delete_user(user_id, &connection);
    match result {
        true => format!("User has been succesfully deleted"),
        false => format!("Failed to delete user"),
    }
}

#[delete("/users")]
pub fn delete_all_user(connection: config::Connection) -> String {
    let result = action::query_delete_all_user(&connection);
    match result {
        true => format!("All User has been succesfully deleted"),
        false => format!("Failed to delete All user"),
    }
}
