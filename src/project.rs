use poem::{http::StatusCode, web::Path, Result};
use poem_openapi::{
    param::Header,
    payload::{Json, PlainText},
    types::multipart::Upload,
    Multipart, Object, OpenApi,
};

use crate::{
    auth::jwt::{decode_token, validate_creds},
    utils::{get_state, random_string, write_file},
    STATE,
};

pub struct ProjectAPI;
#[derive(Object)]
struct Project {
    id: i32,
    title: String,
    offered_by: String,
    description: String,
    img: Option<String>,
    files: Option<String>,
}

#[derive(Object)]
struct NewProject {
    title: String,
    desc: String,
}

#[derive(Object)]
struct UpdateProject {
    id: i32,
    desc: Option<String>,
}

#[derive(Multipart)]
struct UploadImage {
    id: i32,
    img: Upload,
}
#[derive(Multipart)]
struct UploadFiles {
    id: i32,
    files: Vec<Upload>,
}

#[OpenApi]
impl ProjectAPI {
    #[oai(path = "/project/:id", method = "get")]
    async fn get_project(&self, id: Path<i32>) -> Result<Json<Project>> {
        let st = STATE.get().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        let project = sqlx::query_as!(Project, "select * from projects where id = $1", *id)
            .fetch_one(&st.pool)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;
        Ok(Json(project))
    }
    #[oai(path = "/project/all", method = "get")]
    async fn get_projects(&self) -> Result<Json<Vec<Project>>> {
        let st = STATE.get().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        let projects = sqlx::query_as!(Project, "select * from projects")
            .fetch_all(&st.pool)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;
        Ok(Json(projects))
    }
    #[oai(path = "/project/by", method = "get")]
    async fn get_projects_by(&self, by: Path<String>) -> Result<Json<Vec<Project>>> {
        let st = STATE.get().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        let projects =
            sqlx::query_as!(Project, "select * from projects where offered_by = $1", *by)
                .fetch_all(&st.pool)
                .await
                .map_err(|_| StatusCode::NOT_FOUND)?;
        Ok(Json(projects))
    }

    #[oai(path = "/project/new", method = "post")]
    async fn create_proj(
        &self,
        Header(Authorization): Header<String>,
        project: Json<NewProject>,
    ) -> Result<PlainText<&'static str>> {
        let st = STATE.get().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        let creds = decode_token(&Authorization, st.jwt_secret_key)?;
        validate_creds(&Authorization, None, Some(2), st.jwt_secret_key)?;
        sqlx::query!(
            "insert into projects(title,description,offered_by) values ($1,$2,$3)",
            project.title,
            project.desc,
            creds.email
        )
        .execute(&st.pool)
        .await
        .map_err(|_| StatusCode::PRECONDITION_FAILED)?;
        Ok(PlainText("Project Added"))
    }
    #[oai(path = "/project/", method = "put")]
    async fn update_proj(
        &self,
        Header(Authorization): Header<String>,
        user: Json<UpdateProject>,
    ) -> Result<PlainText<&'static str>> {
        let st = STATE.get().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        let creds = decode_token(&Authorization, st.jwt_secret_key)?;
        validate_creds(&Authorization, None, Some(2), st.jwt_secret_key)?;
        sqlx::query!(
            "update projects set description = $1 where id = $2 and offered_by = $3",
            user.desc,
            user.id,
            creds.email
        )
        .execute(&st.pool)
        .await
        .map_err(|_| StatusCode::PRECONDITION_FAILED)?;
        Ok(PlainText("Project Updated"))
    }
    #[oai(path = "/project/img", method = "put")]
    async fn update_img(
        &self,
        Header(Authorization): Header<String>,
        req: UploadImage,
    ) -> Result<PlainText<&'static str>> {
        let st = get_state()?;
        let creds = decode_token(&Authorization, st.jwt_secret_key)?;
        validate_creds(&Authorization, None, Some(2), st.jwt_secret_key)?;

        let name = random_string(10) + req.img.file_name().unwrap();
        write_file(&name, &req.img.into_vec().await.unwrap()).await?;
        sqlx::query!(
            "update projects set img= $1 where id = $2 and offered_by = $3",
            name,
            req.id,
            creds.email
        )
        .execute(&st.pool)
        .await
        .map_err(|_| StatusCode::PRECONDITION_FAILED)?;
        Ok(PlainText("Image uploaded"))
    }
    #[oai(path = "/project/files", method = "post")]
    async fn upload_files(
        &self,
        Header(Authorization): Header<String>,
        req: UploadFiles,
    ) -> Result<PlainText<&'static str>> {
        let st = get_state()?;
        let creds = decode_token(&Authorization, st.jwt_secret_key)?;
        validate_creds(&Authorization, None, Some(2), st.jwt_secret_key)?;

        let mut names: Vec<String> = vec![];
        for f in req.files {
            let name = random_string(10) + f.file_name().unwrap();
            write_file(&name, &f.into_vec().await.unwrap()).await?;
            names.push(name);
        }
        sqlx::query!(
            "update projects set files= $1 where id = $2 and offered_by = $3",
            names.join(","),
            req.id,
            creds.email
        )
        .execute(&st.pool)
        .await
        .map_err(|_| StatusCode::PRECONDITION_FAILED)?;

        Ok(PlainText("Files uploaded"))
    }
    #[oai(path = "/project", method = "delete")]
    async fn delete_proj(
        &self,
        Header(Authorization): Header<String>,
        id: Json<i32>,
    ) -> Result<PlainText<&'static str>> {
        let st = STATE.get().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        let creds = decode_token(&Authorization, st.jwt_secret_key)?;
        validate_creds(&Authorization, None, Some(2), st.jwt_secret_key)?;
        sqlx::query!(
            "delete from projects where id = $1 and offered_by = $2",
            *id,
            creds.email
        )
        .execute(&st.pool)
        .await
        .map_err(|_| StatusCode::PRECONDITION_FAILED)?;
        Ok(PlainText("Project Deleted"))
    }
}
