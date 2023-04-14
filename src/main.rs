use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    oxido::run();
}
