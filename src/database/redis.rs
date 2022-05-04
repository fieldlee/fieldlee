use log::{error, info};
use redis::Connection;
use crate::config::config::ApplicationConfig;
use crate::model::error::{Error,Result};
///Redis缓存服务
pub struct RedisService {
    pub client: redis::Client,
}

impl RedisService {
    pub fn new(url: &str) -> Self {
        info!("conncect redis ({})...", url);
        let client = redis::Client::open(url).unwrap();
        info!("conncect redis success!");
        Self { client }
    }

    pub async fn get_conn(&self) -> Result<Connection> {
        let conn = self.client.get_connection();
        if conn.is_err() {
            let err = format!("RedisService connect fail:{}", conn.err().unwrap());
            error!("{}", err);
            return Err(Error::from(err));
        }
        return Ok(conn.unwrap());
    }
}

pub async fn init_redis(app_config: &ApplicationConfig) -> RedisService {
    let redis = RedisService::new(app_config.redis_url());
    //连接redis
    println!("redis link database success!");
    return redis;
}