use tokio::time::Instant;

const BASE_URL: &str = "http://localhost:3000";

#[tokio::main]
async fn main() {
    {
        let now = Instant::now();
        reqwest::get(format!("{BASE_URL}/health")).await.unwrap();
        println!("reqwest: {:.3?}", now.elapsed());
    };
    {
        let now = Instant::now();
        ureq::get(format!("{BASE_URL}/health").as_str())
            .call()
            .unwrap();
        println!("ureq: {:.3?}", now.elapsed());
    };
}
