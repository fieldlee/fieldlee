use crate::APPLICATION_CONTEXT;
use crate::config::config::ApplicationConfig;
//初始化配置信息
pub async fn init_config() {
    let yml_data = include_str!("../../application.yaml");
    let config = ApplicationConfig::new(yml_data);
    // let mut list = match config.api_white_list_api().clone() {
    //     Some(e) => e,
    //     None => Vec::new(),
    // };
    // /*添加需要登录但是不需要权限的路由
    //  * 如果有额外的可以在application.yml中添加
    //  * admin_auth_list_api
    //  *  - XXXXXX
    //  *  - XXXXX
    //  * */
    // list.push("/user/info".to_string());
    // list.push("/dict/type/all".to_string());
    // list.push("/menu/nav".to_string());
    // config.set_admin_auth_list_api(Some(list));
    APPLICATION_CONTEXT.set::<ApplicationConfig>(config);
}