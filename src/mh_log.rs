use chrono::Utc;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
  Builder::new()
    .format(|buf, record| {
      writeln!(buf,
        "{} [{}] - {}",
        Utc::now().format("%Y-%m-%dT%H:%M:%SZ"),
        record.level(),
        record.args()
      )
    })
    .filter(None, LevelFilter::Trace)
    .init();
  trace!("mh_log::init_logger() end");
  Ok(())
}