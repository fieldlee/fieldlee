
use state::Container;
/*
整个项目上下文ApplicationContext
包括：
ApplicationConfig 配置
Rbatis  mysql orm
ServiceContext 服务上下文
CasbinService 权限服务
*/

pub static APPLICATION_CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();
/*初始化环境上下文*/
pub async fn init_context() {
    //第一步加载配置
    init_config().await;
    init_log();
    info!("ConfigContext init complete");
    //第二步初始化数据源
    init_database().await;
    info!("DataBase init complete");
    //第三步初始化所有的 服务类
    init_service().await;
    info!("ServiceContext init complete");
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    info!(" - Local:   http://{}:{}", cassie_config.server().host().replace("0.0.0.0", "127.0.0.1"), cassie_config.server().port());
}
