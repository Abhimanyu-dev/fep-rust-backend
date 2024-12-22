use dotenv::dotenv;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
use state::State;
use tokio::sync::OnceCell;

mod applications;
mod auth;
mod professor;
mod project;
mod state;
mod student;

struct Api;

static STATE: OnceCell<State> = OnceCell::const_new();

#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    if let Err(err) = STATE.set(State::create().await) {
        eprintln!("{}", err);
    }

    let api_service = OpenApiService::new(
        (
            Api,
            auth::AuthAPI,
            student::StudentAPI,
            professor::ProfessorAPI,
            project::ProjectAPI,
            applications::ApplicationsAPI,
        ),
        "Hello World",
        "1.0",
    )
    .server("http://localhost:3000");

    let ui = api_service.openapi_explorer();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    println!("Starting server!");
    let _ = Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await;
}
