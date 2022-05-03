pub mod api_auth;

use crate::APPLICATION_CONTEXT;
use crate::config::ApplicationConfig;

///api是否处在白名单接口中
#[cached(name = "FRONT_WHITE_LIST_API", time = 60, size = 100)]
pub fn is_white_api_list_api(path: String) -> bool {
    if path.eq("/") {
        return true;
    }
    let app_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    for x in app_config.api_white_list_api() {
        //匹配 user/:id 模式
        if key_match2(path.clone().as_str(), x) || x.contains(path.clone().as_str()) {
            return true;
        }
    }
    return false;
}