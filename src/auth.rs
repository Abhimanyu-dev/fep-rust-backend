use crate::STATE;
use fred::prelude::KeysInterface;
use poem::{http::StatusCode, Error, Result};
use poem_openapi::{
    payload::{Json, PlainText},
    Object, OpenApi,
};
use rand::Rng;

pub struct AuthAPI;

#[derive(Object)]
struct GenOTP {
    email: String,
}

#[derive(Object)]
struct CreateUser {
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

fn generate_otp() -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1000..=9999)
}

async fn verify_otp(email: &str, otp: &str) -> Result<(), Error> {
    let st = match STATE.get() {
        Some(val) => val,
        None => {
            return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
        }
    };
    let real_otp: Option<String> = st
        .redis
        .get(email)
        .await
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;
    let real_otp = match real_otp {
        Some(val) => val,
        None => return Err(Error::from_status(StatusCode::UNAUTHORIZED)),
    };
    match real_otp == otp.to_string() {
        true => Ok(()),
        false => Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

#[OpenApi]
impl AuthAPI {
    #[oai(path = "/auth/otp", method = "post")]
    async fn send_otp(&self, email: Json<GenOTP>) -> Result<PlainText<&'static str>> {
        let otp = generate_otp();
        let st = STATE.get().unwrap();
        let _: () = st
            .redis
            .set(
                &email.email,
                &otp.to_string(),
                Some(fred::types::Expiration::EX(60 * 5)),
                None,
                false,
            )
            .await
            .unwrap();
        println!("otp for {} is {}", &email.email, otp);
        Ok(PlainText("Otp Sent"))
    }
    #[oai(path = "/auth/user/create", method = "post")]
    async fn crate_user(&self, user: Json<CreateUser>) -> Result<PlainText<&'static str>> {
        verify_otp(&user.email, &user.otp.to_string()).await?;
        Ok(PlainText("User Created"))
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
