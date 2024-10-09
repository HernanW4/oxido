use simple_logger::SimpleLogger;
fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .with_module_level("oxido", log::LevelFilter::Debug)
        .with_module_level("calloop", log::LevelFilter::Warn)
        .init()
        .unwrap();

    oxido::run().unwrap();
}
