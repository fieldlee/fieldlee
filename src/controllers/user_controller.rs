
pub async fn login(Json(sign): Json<SignInDTO>) -> impl IntoResponse {
    let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    let sys_auth_service = APPLICATION_CONTEXT.get::<SysAuthService>();
    if let Err(e) = sign.validate() {
        return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
    }
    if let Ok(code) = cache_service.get_string(&format!("_captch:uuid_{}", &sign.uuid().clone().unwrap())).await {
        if !code.eq(&sign.vcode().clone().unwrap()) {
            return RespVO::<()>::from_error(&Error::E("验证码错误".to_string())).resp_json();
        }
    }
    let vo = sys_auth_service.sign_in(&sign).await;
    return RespVO::from_result(&vo).resp_json();
}