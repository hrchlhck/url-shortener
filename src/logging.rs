pub fn init() {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_line_number(true)
        .with_file(true)
        .compact()
        .init();
}
