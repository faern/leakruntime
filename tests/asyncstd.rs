#[cfg(not(feature = "tokio"))]
#[async_std::test]
async fn just_sleep() {
    async_std::task::sleep(std::time::Duration::from_millis(1)).await;
}
