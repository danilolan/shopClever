use tide::Request;
use tide::Server;

use crate::modules::user::*;

pub fn routes(app: &mut Server<()>) {
    app.at("/").get(alive);

    //user
    app.at("/user").get(User::get_user);
}

async fn alive(mut _req: Request<()>) -> tide::Result {
    Ok(format!("I'm alive :D").into())
}
