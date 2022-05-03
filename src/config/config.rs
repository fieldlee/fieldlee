#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone, Getters, Setters)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct ServerConfig {
    ///当前服务地址
    host: String,
    port: String,
}

///服务启动配置
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone, Getters, Setters, MutGetters)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct ApplicationConfig {
    ///server name
    server_name:String,
    ///debug
    debug: bool,
    ///redis地址
    redis_url: String,
    /// 数据库地址
    database_url: String,
    ///日志目录 "target/logs/"
    log_dir: String,
    /// "100MB" 日志分割尺寸-单位KB,MB,GB
    log_temp_size: String,
    /// 日志打包格式可选“”（空-不压缩）“gzip”（gz压缩包）“zip”（zip压缩包）“lz4”（lz4压缩包（非常快））
    log_pack_compress: String,
    ///日志滚动配置   保留全部:All,按时间保留:KeepTime(Duration),按版本保留:KeepNum(i64)
    log_rolling_type: String,
    ///日志等级
    log_level: String,
    ///jwt 秘钥
    jwt_secret: String,
    ///白名单接口
    api_white_list_api: Vec<String>,
    ///权限缓存类型
    cache_type: String,
    ///重试
    login_fail_retry: u64,
    ///重试等待时间
    login_fail_retry_wait_sec: u64,
    //server 配置
    server: ServerConfig,
}

impl ApplicationConfig {
    pub fn new(yml_data: &str) -> Self {
        let config = match serde_yaml::from_str(yml_data) {
            Ok(e) => e,
            Err(e) => panic!("{}", e),
        };
        config
    }
    pub fn validate(&self) {
        if self.redis_url.is_empty() {
            panic!("请配置redis_url")
        }
        if self.database_url.is_empty() {
            panic!("请配置database_url")
        }
    }
}
