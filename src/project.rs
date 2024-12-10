use poem::{web::Multipart, Result};
use poem_openapi::{
    payload::{Json, PlainText},
    types::multipart::Upload,
    Multipart, Object, OpenApi,
};

pub struct ProjectAPI;
#[derive(Object)]
struct Project {
    id: u32,
    offered_by: String,
    desc: String,
    img: Option<String>,
    files: Option<Vec<String>>,
}

#[derive(Object)]
struct NewProject {
    offered_by: String,
    desc: String,
}

#[derive(Object)]
struct UpdateProject {
    id: u32,
    offered_by: Option<String>,
    desc: Option<String>,
}

#[derive(Multipart)]
struct UploadImage {
    file: Upload,
}
#[derive(Multipart)]
struct UploadFiles {
    files: Vec<Upload>,
}

#[OpenApi]
impl ProjectAPI {
    #[oai(path = "/project/:id", method = "get")]
    async fn get_project(&self) -> Result<Json<Project>> {
        todo!()
    }
    #[oai(path = "/project/all", method = "get")]
    async fn get_projects(&self) -> Result<Json<Vec<Project>>> {
        todo!()
    }

    #[oai(path = "/project/new", method = "post")]
    async fn create_proj(&self, user: Json<NewProject>) -> Result<PlainText<&'static str>> {
        todo!()
    }
    #[oai(path = "/project/:id", method = "put")]
    async fn update_proj(&self, user: Json<UpdateProject>) -> Result<PlainText<&'static str>> {
        todo!()
    }
    #[oai(path = "/project/:id/img", method = "put")]
    async fn update_img(&self, user: UploadImage) -> Result<PlainText<&'static str>> {
        todo!()
    }
    #[oai(path = "/project/:id/files", method = "post")]
    async fn upload_files(&self, user: UploadFiles) -> Result<PlainText<&'static str>> {
        todo!()
    }
    #[oai(path = "/project/:id", method = "delete")]
    async fn delete_proj(&self) -> Result<PlainText<&'static str>> {
        todo!()
    }
}
