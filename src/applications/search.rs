use sea_query::*;
use serde::Deserialize;

use super::Application;

#[derive(Iden)]
enum Projects {
    Table,
    Id,
    OfferedBy,
    Title,
    Description,
    Img,
    Files,
}

#[derive(Iden)]
enum Applications {
    Table,
    Id,
    ProjectId,
    Student,
    Note,
    File,
    Status,
}

#[derive(Deserialize)]
pub struct Filter {
    id: Option<i32>,
    project_id: Option<i32>,
    student: Option<String>,
}

pub fn build_search_query(filters: Filter) -> String {
    let mut search_sql = Query::select()
        .expr_as(
            Expr::col((Applications::Table, Applications::Id)),
            Alias::new("apps_id"), // (Applications::Table, Applications::ProjectId),
                                   // (Applications::Table, Applications::Student),
                                   // (Applications::Table, Applications::Note),
                                   // (Applications::Table, Applications::File),
                                   // (Applications::Table, Applications::Status),
        )
        .from(Applications::Table)
        .join(
            JoinType::InnerJoin,
            Projects::Table,
            Expr::col((Applications::Table, Applications::ProjectId))
                .equals((Projects::Table, Projects::Id)),
        )
        .to_owned();
    if let Some(id) = filters.id {
        search_sql.and_where(Expr::col(Applications::ProjectId).eq(id));
    }
    if let Some(project_id) = filters.project_id {
        search_sql.and_where(Expr::col(Applications::ProjectId).eq(project_id));
    }
    if let Some(student) = filters.student {
        search_sql.and_where(Expr::col(Applications::Student).eq(student));
    }

    search_sql.to_string(PostgresQueryBuilder)
}
