use crate::{auth::jwt::*, utils::*};
use poem::{error::Error, http::StatusCode, Result};
use poem_openapi::{
    param::Header,
    payload::{Json, PlainText},
    Object, OpenApi,
};

pub struct ProfessorAPI;

#[derive(Object)]
struct Professor {
    email: String,
    institute: String,
}

#[derive(Object)]
struct Updateprofessor {
    institute: Option<String>,
}

#[OpenApi]
impl ProfessorAPI {
    #[oai(path = "/professor/new", method = "post")]
    async fn create_prof(
        &self,
        Header(Authorization): Header<String>,
        user: Json<Professor>,
    ) -> Result<PlainText<&'static str>> {
        let st = get_state()?;
        let creds = decode_token(&Authorization, st.jwt_secret_key)?;
        validate_creds(&Authorization, None, Some(2), st.jwt_secret_key)?;
        sqlx::query!(
            "insert into professors(email,institute) values($1,$2)",
            creds.email,
            user.institute
        )
        .execute(&st.pool)
        .await
        .map_err(|_| Error::from(StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(PlainText("Created Professor"))
    }
    #[oai(path = "/professor", method = "put")]
    async fn update_prof(
        &self,
        Header(Authorization): Header<String>,
        user: Json<Updateprofessor>,
    ) -> Result<PlainText<&'static str>> {
        let st = get_state()?;
        let creds = decode_token(&Authorization, st.jwt_secret_key)?;
        validate_creds(&Authorization, None, Some(2), st.jwt_secret_key)?;
        sqlx::query!(
            "update professors set institute = $1 where email = $2",
            user.institute,
            creds.email
        )
        .execute(&st.pool)
        .await
        .map_err(|_| Error::from(StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(PlainText("Updated Professor"))
    }
}
