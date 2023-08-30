use tide::prelude::*;
use tide::Request;

mod responses;
mod structs;

#[derive(Debug, Deserialize)]
struct GetUserBody {
    id: Option<String>,
}

pub struct User {}

impl User {
    pub async fn get_user(mut req: Request<()>) -> tide::Result {
        let GetUserBody { id } = req.body_json().await?;

        if !id.is_some() {
            return Ok(responses::Failed::get());
        }

        Ok(responses::Success::get(structs::User {
            name: "danilo".to_string(),
        }))
    }
}
