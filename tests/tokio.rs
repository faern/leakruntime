#[cfg(feature = "tokio")]
#[tokio::test]
async fn just_sleep() {
    tokio::time::delay_for(std::time::Duration::from_millis(1)).await;
}
