use tide::{Request, Result};
// use tide::prelude::*;

async fn greet(req: Request<()>) -> Result<String> {
    let name = req.param("name").unwrap_or("world");
    Ok(format!("Hello, {}!", name))
}
#[async_std::main]
async fn main() -> tide::Result<()> {
let mut app = tide::new();
app.at("/hello").get(greet);
app.at("/hello/:name").get(greet);
app.listen("127.0.0.1:8090").await?;
Ok(())
}
// #[derive(Debug, Deserialize)]
// struct Animal {
//     name: String,
//     legs: u8,
// }

// #[derive(Debug, Deserialize)]
// struct User {
//     name: String,
// }

// #[async_std::main]
// async fn main() -> tide::Result<()> {
//     let mut app = tide::new();
//     app.at("/orders/shoes").post(order_shoes);
//     app.at("/hello").get(say_hello);
//     app.listen("127.0.0.1:8090").await?;
//     Ok(())
// }

// async fn order_shoes(mut req: Request<()>) -> tide::Result {
//     let Animal { name, legs } = req.body_json().await?;
//     Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
// }

// async fn say_hello( req: Request<()>) -> tide::Result {
//    let user:User = req.query()?;
//    Ok(format!("Hello, {}!", user.name).into())
// }