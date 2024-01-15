use tokio::time::Instant;

const URL: &str = "http://localhost:3000/health";

#[tokio::main]
async fn main() {
    {
        let now = Instant::now();
        reqwest::get(URL).await.unwrap();
        println!("reqwest: {:.3?}", now.elapsed());
    };
    {
        let now = Instant::now();
        ureq::get(URL).call().unwrap();
        println!("ureq: {:.3?}", now.elapsed());
    };
}
