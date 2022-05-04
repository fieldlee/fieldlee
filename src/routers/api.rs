use axum::{
    routing::{get, post},
    Router,
};
pub fn routers() -> Router {
    Router::new()
        //-------------------------------------登录服务-------------------------------------------------------
        .route("/getsmscode", get(sys_auth_resource::captcha_img))
        .route("/login", post(sys_auth_resource::login))
}
