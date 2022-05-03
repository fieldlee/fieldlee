use fieldlee::{APPLICATION_CONTEXT,init_context};
use fieldlee::config::config::ApplicationConfig;
use fieldlee::model::model::RespVO;
use log::warn;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use axum::{handler::Handler, http::Uri, response::IntoResponse, Router, Server};


async fn fallback(uri: Uri) -> impl IntoResponse {
    let msg = format!("资源不存在：{}", uri);
    warn!("{}", msg.clone());
    RespVO::<String> {
        code: Some(-1),
        msg: Some(msg),
        data: None,
    }
    .resp_json()
}

#[tokio::main]
async fn main() {
    //初始化上环境下文
    init_context().await;
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let server = format!("{}:{}", cassie_config.server().host(), cassie_config.server().port());
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers(Any).max_age(Duration::from_secs(60) * 10);
    //绑定端口 初始化 路由
    let app = Router::new()
        // .nest(
        //     "/admin",
        //     admin::routers()
        //         .layer(layer_fn(|inner| EventMiddleware { inner })) //第二执行的
        //         .layer(extractor_middleware::<auth_admin::Auth>()), //最先执行的
        // )
        // .nest("/api", api::routers().layer(extractor_middleware::<auth_api::Auth>()))
        .layer(cors)
        .fallback(fallback.into_service());
    // 启动服务
    Server::bind(&server.parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}
