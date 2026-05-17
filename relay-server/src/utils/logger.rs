pub fn setup_logger() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .init();
}
