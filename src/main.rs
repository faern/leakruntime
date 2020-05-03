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

// When using `cargo expand` on the crate these two main methods fall out:

#[cfg(not(feature = "tokio"))]
fn _expanded_main_asyncstd() {
    #[cfg(not(feature = "tokio"))]
    async fn main() {
        {
            async_std::task::sleep(std::time::Duration::from_millis(1)).await;
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
                tokio::time::delay_for(std::time::Duration::from_millis(1)).await;

                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["Hello from tokio\n"],
                        &match () {
                            () => [],
                        },
                    ));
                };
            }
        })
}
