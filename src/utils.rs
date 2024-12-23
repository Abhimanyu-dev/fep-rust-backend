use crate::{state::State, STATE};
use poem::http::StatusCode;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn write_file(name: &str, data: &[u8]) -> Result<(), StatusCode> {
    let mut file = match File::create(format!("res/{name}")).await {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Cannot save file! {err}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    if let Err(err) = file.write_all(data).await {
        eprintln!("{}", err);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    Ok(())
}
pub fn random_string(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}

pub fn get_state<'a>() -> Result<&'a State, StatusCode> {
    STATE.get().ok_or(StatusCode::INTERNAL_SERVER_ERROR)
}
