use crate::model::user::UserInfo;
use crate::model::user::{SignInDTO,UserRegDTO};
pub struct UserService {}

impl Default for UserService {
    fn default() -> Self {
        UserService {}
    }
}

impl UserService {
    pub async fn sign_in(&self, arg: &SignInDTO) -> Result<SignInVO> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let user: Option<UserInfo> = rb.fetch_by_wrapper(rb.new_wrapper().eq(UserInfo::user_name(), &arg.username())).await?;
        let user = user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", arg.username().clone().unwrap_or_default())))?;
        if user.status.eq(&Some(0)) {
            return Err(Error::from("账户被禁用!"));
        }
        let mut error = None;
        if !PasswordEncoder::verify(
            user.passcode.as_ref().ok_or_else(|| Error::from("错误的用户数据，密码为空!"))?,
            arg.password().as_ref().ok_or_else(|| Error::from("密码不能为空"))?,
        ) {
            error = Some(Error::from("密码不正确!"));
        }
        if error.is_some() {
            return Err(error.unwrap());
        }
        let sign_in_vo = self.get_user_info(&user).await?;
        return Ok(sign_in_vo);
    }

    pub async fn get_user_info_by_token(&self, token: &JWTToken) -> Result<SignInVO> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let user: Option<UserInfo> = rb.fetch_by_wrapper(rb.new_wrapper().eq(UserInfo::id(), &token.id())).await?;
        let user = user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", token.username())))?;
        return self.get_user_info(&user).await;
    }

    pub async fn get_user_info(&self, user: &UserInfo) -> Result<SignInVO> {
        let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
        //去除密码，增加安全性
        let mut user = user.clone();
        user.password = None;
        let agency_code = user.agency_code.clone();
        let user_id = user.id.clone().ok_or_else(|| Error::from("错误的用户数据，id为空!"))?;
        let mut sign_vo = SignInVO::default();
        sign_vo.set_user(Some(user.clone().into()));
        //提前查找所有权限，避免在各个函数方法中重复查找
        let mut jwt_token = JWTToken::default();
        jwt_token.set_id(user_id);
        jwt_token.set_super_admin(user.super_admin.clone().unwrap_or_default());
        jwt_token.set_from("admin".to_string());
        jwt_token.set_username(user.username.clone().unwrap_or_default());
        jwt_token.set_agency_code(agency_code.clone().unwrap_or_default());
        jwt_token.set_exp(DateTimeNative::now().timestamp_millis() as usize);
        sign_vo.set_access_token(jwt_token.create_token(cassie_config.jwt_secret())?);
        return Ok(sign_vo);
    }

    //根据id删除用户
    pub async fn delete_user(&self, id: String) {
        let user_info = self.get(id.clone()).await.unwrap();
        self.del(&id).await;
    }
    //保存用户
    pub async fn save_info(&self, arg: UserRegDTO) {
        let password = PasswordEncoder::encode(&arg.passcode().clone().unwrap().as_str());

        let mut entity: UserInfo = UserInfo{
            passcode : password,
            ..arg
        };
        /*保存到数据库*/
        let uid = if let Some(id) = entity.id {
            self.update_by_id(id.to_string(), &entity).await;
            id
        } else {
            let id = self.save(&mut entity).await;
            id.unwrap()
        };
    }
}
