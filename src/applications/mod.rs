use crate::{auth::jwt::*, utils::*};
use poem::{
    http::StatusCode,
    web::{Path, Query},
    Result,
};
use poem_openapi::{
    param::Header,
    payload::{Json, PlainText},
    types::multipart::Upload,
    Multipart, Object, OpenApi,
};
use search::{build_search_query, Filter};
use sqlx::Row;
mod search;

pub struct ApplicationsAPI;
#[derive(Object)]
struct Application {
    id: i32,
    project_id: i32,
    student: String,
    note: String,
    file: String,
    status: String,
}

#[derive(Multipart)]
struct NewApplication {
    project_id: i32,
    note: String,
    file: Upload,
}

#[derive(Object)]
struct UpdateApp {
    id: i32,
    note: Option<String>,
}
#[derive(Object)]
struct UpdateStatus {
    id: i32,
    status: String,
}

#[OpenApi]
impl ApplicationsAPI {
    #[oai(path = "/applications/:id", method = "get")]
    async fn get_app(&self, Path(id): Path<i32>) -> Result<Json<Application>> {
        let st = get_state()?;
        let application =
            sqlx::query_as!(Application, "select * from applications where id = $1", id)
                .fetch_one(&st.pool)
                .await
                .map_err(|_| StatusCode::NOT_FOUND)?;
        Ok(Json(application))
    }

    #[oai(path = "/applications/ids", method = "get")]
    async fn get_app_ids(&self, Query(query): Query<Filter>) -> Result<Json<Vec<i32>>> {
        let st = get_state()?;
        let sql_query = build_search_query(query);
        println!("{}", &sql_query);

        match sqlx::query(&sql_query).fetch_all(&st.pool).await {
            Ok(val) => Ok(Json(
                val.into_iter().map(|row| row.get("apps_id")).collect(),
            )),
            Err(err) => {
                eprintln!("{:?}", err);
                Err(StatusCode::EXPECTATION_FAILED.into())
            }
        }
    }

    #[oai(path = "/project/application/apply", method = "post")]
    async fn create_app(
        &self,
        Header(Authorization): Header<String>,
        new_app: NewApplication,
    ) -> Result<PlainText<&'static str>> {
        let st = get_state()?;
        let creds = decode_token(&Authorization, st.jwt_secret_key)?;
        validate_creds(&Authorization, None, Some(1), st.jwt_secret_key)?;
        let name = random_string(10) + new_app.file.file_name().unwrap();
        write_file(&name, &new_app.file.into_vec().await.unwrap()).await?;
        sqlx::query!(
            "insert into applications(project_id,student,note,file) values ($1,$2,$3,$4)",
            new_app.project_id,
            creds.email,
            new_app.note,
            name,
        )
        .execute(&st.pool)
        .await
        .map_err(|_| StatusCode::PRECONDITION_FAILED)?;

        Ok(PlainText("New Application Created"))
    }
    #[oai(path = "/application/:id", method = "put")]
    async fn update_app(
        &self,
        Header(Authorization): Header<String>,
        updates: Json<UpdateApp>,
    ) -> Result<PlainText<&'static str>> {
        let st = get_state()?;
        let creds = decode_token(&Authorization, st.jwt_secret_key)?;
        validate_creds(&Authorization, None, Some(1), st.jwt_secret_key)?;
        sqlx::query!(
            "update applications set note = $1 where id = $2 and student = $3",
            updates.note.clone().unwrap(),
            updates.id,
            creds.email
        )
        .execute(&st.pool)
        .await
        .map_err(|_| StatusCode::PRECONDITION_FAILED)?;

        Ok(PlainText("Application Updated"))
    }

    #[oai(path = "/application/status", method = "put")]
    async fn update_status(
        &self,
        Header(Authorization): Header<String>,
        updates: Json<UpdateStatus>,
    ) -> Result<PlainText<&'static str>> {
        let st = get_state()?;
        let creds = decode_token(&Authorization, st.jwt_secret_key)?;
        let record  = sqlx::query!(
            "select offered_by from applications inner join projects on applications.project_id = projects.id"
        )
        .fetch_one(&st.pool)
        .await
        .map_err(|_| StatusCode::PRECONDITION_FAILED)?;

        validate_creds(
            &Authorization,
            Some(&record.offered_by),
            Some(2),
            st.jwt_secret_key,
        )?;

        sqlx::query!(
            "update applications set status = $1 where id = $2",
            &updates.status,
            updates.id,
        )
        .execute(&st.pool)
        .await
        .map_err(|_| StatusCode::PRECONDITION_FAILED)?;

        Ok(PlainText("Application Status Updated"))
    }
    #[oai(path = "/applications/:id", method = "delete")]
    async fn delete_app(
        &self,
        Header(Authorization): Header<String>,
        id: Path<i32>,
    ) -> Result<PlainText<&'static str>> {
        let st = get_state()?;
        let creds = decode_token(&Authorization, st.jwt_secret_key)?;
        validate_creds(&Authorization, None, Some(1), st.jwt_secret_key)?;
        sqlx::query!(
            "delete from applications where id = $1 and student = $2",
            *id,
            creds.email
        )
        .execute(&st.pool)
        .await
        .map_err(|_| StatusCode::PRECONDITION_FAILED)?;

        Ok(PlainText("Application Deleted"))
    }
}
