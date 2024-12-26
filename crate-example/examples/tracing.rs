use tokio::time::{sleep, Duration, Instant};

// We use the instrument macro from the tracing crate to instrument our three sleep functions
#[tracing::instrument]
async fn sleep_1s() {
    sleep(Duration::from_secs(1)).await;
}

#[tracing::instrument]
async fn sleep2s() {
    sleep(Duration::from_secs(2)).await;
}

#[tracing::instrument]
async fn sleep_3s() {
    sleep(Duration::from_secs(3)).await;
}

#[tokio::main]
async fn main() {
    // We have to initialize the console subscriber in our main function to emit the traces
    console_subscriber::init();

    loop {
        // We'll fire and forget sleep 1 and sleep 2 and then block on sleep 3.
        tokio::spawn(sleep_1s());
        tokio::spawn(sleep2s());
        // Here, we block on sleep 3 until 3 seconds have elapsed and then repeat the process forever.
        sleep_3s().await;
    }
}

#[tokio::test]
async fn sleep_test() {
    let start_time = Instant::now();
    sleep(Duration::from_secs(1)).await;
    let end_time = Instant::now();

    let seconds = end_time
        .checked_duration_since(start_time)
        .unwrap()
        .as_secs();
    assert_eq!(seconds, 1);
}