use log::{LevelFilter, SetLoggerError, Record, Level, Metadata};

pub struct Logger;

impl Logger {
  pub fn new() -> Result<(), SetLoggerError> {
    return log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));
  }
}

static LOGGER: Logger = Logger;

impl log::Log for Logger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= Level::Info
  }

  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      println!("{} - {} - {}", record.level(), record.target(), record.args());
    }
  }

  fn flush(&self) {}
}
