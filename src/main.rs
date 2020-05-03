use std::time::Duration;

const DELAY: Duration = Duration::from_millis(1);

#[cfg(not(feature = "tokio"))]
#[async_std::main]
async fn main() {
    async_std::task::sleep(DELAY).await;
}

#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() {
    tokio::time::delay_for(DELAY).await;
}

// When using `cargo expand` on the crate these two main methods fall out:

#[cfg(not(feature = "tokio"))]
fn _expanded_main_asyncstd() {
    #[cfg(not(feature = "tokio"))]
    async fn main() {
        {
            async_std::task::sleep(DELAY).await;
        }
    }
    async_std::task::block_on(async { main().await })
}

#[cfg(feature = "tokio")]
fn _expanded_main_tokio() {
    tokio::runtime::Builder::new()
        .basic_scheduler()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            {
                tokio::time::delay_for(DELAY).await;
            }
        })
}
