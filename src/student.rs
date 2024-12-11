use crate::STATE;
use poem::{error::Error, http::StatusCode, Result};
use poem_openapi::{
    payload::{Json, PlainText},
    Object, OpenApi,
};
use sqlx;

pub struct StudentAPI;

#[derive(Object)]
struct Student {
    email: String,
    cpi: f64,
    branch: String,
    batch: u32,
    rollno: u32,
}

#[derive(Object)]
struct UpdateStudent {
    cpi: Option<f64>,
    branch: Option<String>,
    batch: Option<u32>,
    rollno: Option<u32>,
}

#[OpenApi]
impl StudentAPI {
    #[oai(path = "/student/new", method = "post")]
    async fn create_student(&self, user: Json<Student>) -> Result<PlainText<&'static str>> {
        let st = match STATE.get() {
            Some(val) => val,
            None => {
                return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
            }
        };
        sqlx::query!(
            "insert into students(email,cpi,branch,batch,roll_no) values($1,$2,$3,$4,$5)",
            user.email,
            user.cpi,
            user.branch,
            user.batch as i32,
            user.rollno as i32,
        )
        .execute(&st.pool)
        .await
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(PlainText("User Created"))
    }
    #[oai(path = "/student/:id", method = "put")]
    async fn update_student(&self, user: Json<UpdateStudent>) -> Result<PlainText<&'static str>> {
        todo!()
    }
}
