

#[tokio::main]
async fn main() {
    //初始化上环境下文
    init_context().await;
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let server = format!("{}:{}", cassie_config.server().host(), cassie_config.server().port());
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers(Any).max_age(Duration::from_secs(60) * 10);
    //绑定端口 初始化 路由
    let app = Router::new()
        .nest(
            "/admin",
            admin::routers()
                .layer(layer_fn(|inner| EventMiddleware { inner })) //第二执行的
                .layer(extractor_middleware::<auth_admin::Auth>()), //最先执行的
        )
        .nest("/api", api::routers().layer(extractor_middleware::<auth_api::Auth>()))
        .layer(cors)
        .fallback(fallback.into_service());
    // 启动服务
    Server::bind(&server.parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}
