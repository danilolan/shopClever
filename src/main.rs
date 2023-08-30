mod modules;
mod routes;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    routes::routes(&mut app);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
