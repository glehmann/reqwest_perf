// use std::time::Duration;

use std::time::Duration;

use statistical::mean;
use tokio::{
    task,
    time::{sleep, Instant},
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nb_of_tasks = if args.len() > 1 {
        args[1].parse().unwrap()
    } else {
        100
    };
    println!("Running {} tasks", nb_of_tasks);

    let handles: Vec<_> = (0..nb_of_tasks)
        .into_iter()
        .map(|_| {
            task::spawn(async move {
                let now = Instant::now();
                sleep(Duration::from_millis(10)).await;
                let duration = now.elapsed();
                duration
            })
        })
        .collect();

    let durations: Vec<f32> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.unwrap().as_millis() as f32)
        .collect();
    println!("mean: {:.3?}ms", mean(&durations));
}
