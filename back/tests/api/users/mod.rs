use camion::core::users::Role;

use super::{TestApp, auth::login::login, insert_test_user};

pub mod get_user_data;
pub mod search_users;
pub mod change_user_role;

pub async fn create_user_and_login_with_username(
    app: &TestApp,
    username: &str,
    email: &str,
    password: &str,
    role: &Role,
) -> (i32, String) {
    let id = insert_test_user(username, email, password, role, &app.db_conn_pool).await;
    let (_, body) = login(app, username, password).await;
    (id, body["token"].as_str().unwrap().to_owned())
}