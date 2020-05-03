#[cfg(not(feature = "tokio"))]
#[async_std::main]
async fn main() {
    async_std::task::sleep(std::time::Duration::from_millis(1)).await;
    println!("Hello from async-std");
}

#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() {
    tokio::time::delay_for(std::time::Duration::from_millis(1)).await;
    println!("Hello from tokio");
}
