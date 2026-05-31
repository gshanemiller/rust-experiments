use log::{LevelFilter, Record, Level, Metadata};

pub struct Logger;

impl Logger {
  pub fn new() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);
  }
}

static LOGGER: Logger = Logger;

impl log::Log for Logger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= Level::Trace
  }

  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      println!("{} - {} - {}", record.level(), record.target(), record.args());
    }
  }

  fn flush(&self) {}
}
