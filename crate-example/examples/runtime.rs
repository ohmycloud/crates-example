use tokio;

// https://rust-book.junmajinlong.com
fn main() {
    // create a runtime with single thread
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    // create a runtime with multi threads
    let rt = tokio::runtime::Runtime::new().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(30));
    // create runtime with thread pool

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8) // 8 worker
        .enable_io() // enable async io in runtime
        .enable_time() // enable async timer in runtime
        .build() // build the runtime
        .unwrap();
    std::thread::sleep(std::time::Duration::from_secs(30));
}
