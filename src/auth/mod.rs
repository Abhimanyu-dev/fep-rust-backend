pub mod jwt;
use crate::STATE;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::{Duration, Utc};
use fred::prelude::*;
use jwt::{generate_token, Claims};
use poem::{http::StatusCode, web::Path, Error, Result};
use poem_openapi::{
    param::Header,
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
    password: String,
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
fn get_hash(passwd: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    match Argon2::default().hash_password(passwd.as_bytes(), &salt) {
        Ok(val) => Ok(val.to_string()),
        Err(err) => {
            eprintln!("{}", err);
            Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
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
    match real_otp == otp {
        true => Ok(()),
        false => Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

fn match_hash(passwd: &str, saved_pass: &str) -> Result<(), Error> {
    let parsed_hash = PasswordHash::new(saved_pass)
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;
    match Argon2::default()
        .verify_password(passwd.as_bytes(), &parsed_hash)
        .is_ok()
    {
        true => Ok(()),
        false => Err(Error::from_status(StatusCode::UNAUTHORIZED)),
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
        let st = match STATE.get() {
            Some(val) => val,
            None => {
                return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
            }
        };
        let hash = get_hash(&user.password)?;
        sqlx::query!(
            "insert into users(name,email,password,role_id) values ($1,$2,$3,$4)",
            user.name,
            user.email,
            hash,
            1
        )
        .execute(&st.pool)
        .await
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(PlainText("User Created"))
    }

    #[oai(path = "/auth/user/login", method = "post")]
    async fn login_user(&self, user: Json<Login>) -> Result<PlainText<String>> {
        let st = match STATE.get() {
            Some(val) => val,
            None => {
                return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
            }
        };
        let s = sqlx::query!(
            "select password,role_id from users where email = $1",
            user.email
        )
        .fetch_one(&st.pool)
        .await
        .map_err(|_| StatusCode::PRECONDITION_FAILED)?;

        let claims = Claims {
            email: user.email.clone(),
            role_id: s.role_id,
            exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
        };
        let token = generate_token(claims, st.jwt_secret_key)?;

        match_hash(&user.password, &s.password)?;
        Ok(PlainText(token))
    }

    #[oai(path = "/auth/user/:id", method = "delete")]
    async fn delete_user(
        &self,
        Header(Authorization): Header<String>,
        id: Path<i32>,
    ) -> Result<PlainText<&'static str>> {
        todo!()
    }
}
