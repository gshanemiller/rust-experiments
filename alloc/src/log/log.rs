use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError, Record, Level, Metadata};

pub struct DefaultLogger;

impl log::Log for DefaultLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= Level::Info
  }

  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      println!("{}.{}: {}", record.level(), record.target(), record.args());
    }
  }

  fn flush(&self) {
  }
}

pub fn Install(&self: DefaultLogger) {
  log::set_logger(self).map(|()| log::set_max_level(LevelFilter::Warn))
}
