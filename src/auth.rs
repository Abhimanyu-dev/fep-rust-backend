use poem::Result;
use poem_openapi::{
    payload::{Json, PlainText},
    Object, OpenApi,
};

pub struct AuthAPI;

#[derive(Object)]
struct GenOTP {
    email: String,
}

#[derive(Object)]
struct CreateUser {
    id: u32,
    name: String,
    email: String,
    role_id: u32,
    otp: u32,
}

#[derive(Object)]
struct ChangeUser {
    id: u32,
    name: Option<String>,
}
#[derive(Object)]
struct Login {
    email: String,
    password: String,
}

#[OpenApi]
impl AuthAPI {
    #[oai(path = "/auth/otp", method = "post")]
    async fn send_otp(&self, email: Json<GenOTP>) -> Result<PlainText<&'static str>> {
        println!("Sent to {}", email.email);
        Ok(PlainText("otp generated"))
    }
    #[oai(path = "/auth/user/create", method = "post")]
    async fn crate_user(&self, user: Json<CreateUser>) -> Result<PlainText<&'static str>> {
        todo!()
    }
    #[oai(path = "/auth/user/login", method = "post")]
    async fn login_user(&self, user: Json<Login>) -> Result<PlainText<&'static str>> {
        todo!()
    }

    #[oai(path = "/auth/user/:id", method = "put")]
    async fn delete_user(&self, user: Json<ChangeUser>) -> Result<PlainText<&'static str>> {
        todo!()
    }
}
