use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError, Record, Level, Metadata};

struct SimpleLogger;

impl log::Log for SimpleLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= Level::Info
  }

  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      println!("{} - {}", record.level(), record.args());
    }
  }

  fn flush(&self) {}
}

struct Position {
  x: f64,
  y: f64,
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
  log::set_logger(&LOGGER)
    .map(|()| log::set_max_level(LevelFilter::Info))
}

fn main() {
  _ = init();
  let pos = Position { x: 3.234, y: -1.223 };
  error!(target: "app_events", "New position: x: {}, y: {}", pos.x, pos.y);
  error!("error log");
  warn!("warn log");
  info!("info log");
  debug!("debug log");
  trace!("trace log");
}
