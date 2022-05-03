#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

#[macro_use]
extern crate getset;

pub mod config;
pub mod app_log;
pub mod init;
pub mod model;

use crate::config::config::ApplicationConfig;
use crate::init::init_config::init_config;
use log::info;
use state::Container;
use crate::app_log::app_log::init_log;
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
    // init_database().await;
    info!("DataBase init complete");
    //第三步初始化所有的 服务类
    // init_service().await;
    info!("ServiceContext init complete");
    let app_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    info!(" - Local:   http://{}:{}", app_config.server().host().replace("0.0.0.0", "127.0.0.1"), app_config.server().port());
}
