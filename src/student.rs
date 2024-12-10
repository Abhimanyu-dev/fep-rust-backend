use poem::Result;
use poem_openapi::{
    payload::{Json, PlainText},
    Object, OpenApi,
};

pub struct StudentAPI;

#[derive(Object)]
struct Student {
    email: String,
    cpi: f32,
    branch: String,
    batch: u32,
    rollno: u32,
}

#[derive(Object)]
struct UpdateStudent {
    cpi: Option<f32>,
    branch: Option<String>,
    batch: Option<u32>,
    rollno: Option<u32>,
}

#[OpenApi]
impl StudentAPI {
    #[oai(path = "/student/new", method = "post")]
    async fn create_student(&self, user: Json<Student>) -> Result<PlainText<&'static str>> {
        todo!()
    }
    #[oai(path = "/student/:id", method = "put")]
    async fn update_student(&self, user: Json<UpdateStudent>) -> Result<PlainText<&'static str>> {
        todo!()
    }
}
