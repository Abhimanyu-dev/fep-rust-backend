use poem::{web::Multipart, Result};
use poem_openapi::{
    payload::{Json, PlainText},
    types::multipart::Upload,
    Multipart, Object, OpenApi,
};

pub struct ApplicationsAPI;
#[derive(Object)]
struct Application {
    id: u32,
    offered_by: String,
    project: u32,
    note: String,
    files: Option<String>,
    status: String,
}

#[derive(Multipart)]
struct NewApplication {
    note: String,
    file: Upload,
}

#[derive(Object)]
struct UpdateApp {
    id: u32,
    note: Option<String>,
}
#[derive(Object)]
struct UpdateStatus {
    status: Option<String>,
}

#[OpenApi]
impl ApplicationsAPI {
    #[oai(path = "/applications/:id", method = "get")]
    async fn get_project(&self) -> Result<Json<Application>> {
        todo!()
    }
    #[oai(path = "/project/:id/applications", method = "get")]
    async fn get_projects(&self) -> Result<Json<Vec<Application>>> {
        todo!()
    }

    #[oai(path = "/project/:id/application/apply", method = "post")]
    async fn create_proj(&self, user: NewApplication) -> Result<PlainText<&'static str>> {
        todo!()
    }
    #[oai(path = "/project/:id", method = "put")]
    async fn update_proj(&self, user: Json<UpdateApp>) -> Result<PlainText<&'static str>> {
        todo!()
    }

    #[oai(path = "/project/:id/status", method = "put")]
    async fn update_status(&self, user: Json<UpdateStatus>) -> Result<PlainText<&'static str>> {
        todo!()
    }
    #[oai(path = "/applications/:id", method = "delete")]
    async fn delete_proj(&self) -> Result<PlainText<&'static str>> {
        todo!()
    }
}
