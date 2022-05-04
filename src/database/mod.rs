pub mod mysql;
pub mod redis;
use rbatis::rbatis::Rbatis;
use crate::APPLICATION_CONTEXT;
use crate::config::config::ApplicationConfig;
use self::redis::RedisService;

pub async fn init_database(){
    let app_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let mysql =  mysql::init_rbatis(app_config).await;
    let redis =  redis::init_redis(app_config).await;
    APPLICATION_CONTEXT.set::<Rbatis>(mysql);
    APPLICATION_CONTEXT.set::<RedisService>(redis);
}