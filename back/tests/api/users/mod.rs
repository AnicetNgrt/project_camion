use camion::core::users::UserRole;

use super::{TestApp, auth::login::login, insert_test_user};

pub mod get_user_data;
pub mod search_users;

async fn create_user_and_login_with_username(
    app: &TestApp,
    username: &str,
    email: &str,
    password: &str,
    role: &UserRole,
) -> (i32, String) {
    let id = insert_test_user(username, email, password, role, &app.db_conn_pool).await;
    let (_, body) = login(app, username, password).await;
    (id, body["token"].as_str().unwrap().to_owned())
}