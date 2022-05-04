use serde::{Deserialize, Serialize};
use validator_derive::Validate;
use rbatis::DateTimeNative;

#[derive(Serialize, Deserialize, Validate, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SignInDTO {
    #[validate(required)]
    #[validate(length(min = 5, message = "账号最少5个字符"))]
    username: Option<String>,
    #[validate(required)]
    #[validate(length(min = 6, message = "密码最少6个字符"))]
    password: Option<String>,
    //验证码，可用是短信验证码，图片验证码,二维码验证码...
    #[validate(required)]
    #[validate(length(equal = 4, message = "验证码必须为4位"))]
    vcode: Option<String>,
    #[validate(required)]
    uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct UserRegDTO {
    pub id: Option<usize>,
    #[validate(required)]
    pub user_phone: Option<u8>,
    #[validate(required)]
    pub passcode: Option<String>,
    #[validate(required)]
    pub nation: Option<String>,
    #[validate(required)]
    pub user_phone_type: Option<u8>,
    #[validate(required)]
    pub gender: Option<u8>,
    #[validate(required)]
    pub user_name: Option<String>,
    #[validate(required)]
    pub user_avator: Option<String>,
    #[validate(required)]
    pub user_union: Option<String>,
    #[validate(required)]
    pub user_image: Option<String>,
    #[validate(required)]
    pub user_lvl: Option<u8>,
    #[validate(required)]
    pub user_badges: Option<String>,
    #[validate(required)]
    pub user_topics: Option<String>,
    #[validate(required)]
    pub birthday: Option<DateTimeNative>,
}



#[crud_table(table_name:user_info)]
#[derive(Clone, Debug)]
pub struct UserInfo {
    pub id: Option<usize>,
    pub user_phone: Option<u8>,
    pub passcode: Option<String>,
    pub nation: Option<String>,
    pub user_phone_type: Option<u8>,
    pub gender: Option<u8>,
    pub user_name: Option<String>,
    pub user_avator: Option<String>,
    pub user_union: Option<String>,
    pub user_image: Option<String>,
    pub user_lvl: Option<u8>,
    pub user_badges: Option<String>,
    pub user_topics: Option<String>,
    pub birthday: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
    pub created_at: Option<DateTimeNative>,
}