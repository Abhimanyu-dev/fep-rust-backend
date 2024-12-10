use poem::Result;
use poem_openapi::{
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
    async fn create_prof(&self, user: Json<Professor>) -> Result<PlainText<&'static str>> {
        todo!()
    }
    #[oai(path = "/professor/:id", method = "put")]
    async fn update_prof(&self, user: Json<Updateprofessor>) -> Result<PlainText<&'static str>> {
        todo!()
    }
}
