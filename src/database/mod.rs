pub mod mysql;
pub mod redis;
use crate::APPLICATION_CONTEXT;
use crate::config::config::ApplicationConfig;

pub async fn init_database(){
    let app_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    mysql::init_rbatis(app_config);
    redis::init_redis(app_config);
}