use rbatis::rbatis::Rbatis;
use rbatis::db::DBPoolOptions;
use crate::config::config::ApplicationConfig;
use std::time;
///实例化 rbatis orm 连接池
pub async fn init_rbatis(app_config: &ApplicationConfig) -> Rbatis {
    let rbatis = Rbatis::new();
    if app_config.debug().eq(&false) && rbatis.is_debug_mode() {
        panic!(r#"已使用release模式，但是rbatis仍使用debug模式！请删除 Cargo.toml 中 rbatis的配置 features = ["debug_mode"]"#);
    }
    //连接数据库
    println!("rbatis link database ({})...", app_config.mysql_url().clone());
    let mut opt = DBPoolOptions::new();
    opt.max_connections = *app_config.mysql_max_connect();
    opt.max_lifetime = Some(time::Duration::from_secs(*app_config.mysql_max_lifetime()));
    rbatis.link_opt(&app_config.mysql_url(),opt).await.expect("rbatis link database fail!");
    
    println!("rbatis link database success!");
    return rbatis;
}
