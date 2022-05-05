use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct Number {
    value: i32,
}

fn factorielle(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * factorielle(n - 1)
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/fact/compute").post(return_facto);
    app.at("/fact/compute/:n").get(return_facto_get);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn return_facto(mut req: Request<()>) -> tide::Result {
    let Number { value } = req.body_json().await?;
    let result = factorielle(value);
    Ok(format!("Factorielle de {} = {}\n", value, result).into())
}

async fn return_facto_get(req: Request<()>) -> tide::Result {
    let n: i32 = req.param("n")?.parse().unwrap_or(0);
    let result = factorielle(n);
    Ok(format!("Factorielle de {} = {}\n", n, result).into())
}
