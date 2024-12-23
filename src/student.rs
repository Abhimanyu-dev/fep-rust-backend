use crate::{
    auth::jwt::{decode_token, validate_creds},
    utils::get_state,
};
use poem::{error::Error, http::StatusCode, Result};
use poem_openapi::{
    param::{Header, Path},
    payload::{Json, PlainText},
    Object, OpenApi,
};

pub struct StudentAPI;

#[derive(Object)]
struct NewStudent {
    cpi: f64,
    branch: String,
    batch: i32,
    roll_no: i32,
}

#[derive(Object)]
struct Student {
    email: String,
    cpi: f64,
    branch: String,
    batch: i32,
    roll_no: i32,
}

#[OpenApi]
impl StudentAPI {
    #[oai(path = "/student/new", method = "post")]
    async fn create_student(
        &self,
        Header(Authorization): Header<String>,
        user: Json<NewStudent>,
    ) -> Result<PlainText<&'static str>> {
        let st = get_state()?;
        let creds = decode_token(&Authorization, st.jwt_secret_key)?;
        validate_creds(&Authorization, None, Some(1), st.jwt_secret_key)?;
        sqlx::query!(
            "insert into students(email,cpi,branch,batch,roll_no) values($1,$2,$3,$4,$5)",
            creds.email,
            user.cpi,
            user.branch,
            user.batch,
            user.roll_no,
        )
        .execute(&st.pool)
        .await
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(PlainText("Student Created"))
    }
    #[oai(path = "/student/:id", method = "put")]
    async fn update_student(
        &self,
        Header(Authorization): Header<String>,
        user: Json<NewStudent>,
    ) -> Result<PlainText<&'static str>> {
        let st = get_state()?;
        let creds = decode_token(&Authorization, st.jwt_secret_key)?;
        validate_creds(&Authorization, None, Some(0), st.jwt_secret_key)?;
        sqlx::query!(
            "update students set cpi = $1,branch = $2, batch = $3, roll_no = $4 where email = $5",
            user.cpi,
            user.branch,
            user.batch,
            user.roll_no,
            creds.email,
        )
        .execute(&st.pool)
        .await
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(PlainText("Student Updated"))
    }
    #[oai(path = "/student/:email", method = "post")]
    async fn get_student(&self, Path(email): Path<String>) -> Result<PlainText<&'static str>> {
        let st = get_state()?;
        sqlx::query_as!(Student, "select * from students where email=$1", email)
            .fetch_one(&st.pool)
            .await
            .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(PlainText("Student Created"))
    }
}
