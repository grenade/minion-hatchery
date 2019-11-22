#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;
extern crate yaml_rust;

mod mh_config;
mod mh_log;


fn main() -> Result<(), Box<dyn std::error::Error>> {
  mh_log::init_logger()?;
  let config = mh_config::read_config("config.yml")?;
  info!("taskcluster queue api: {:?}", config["taskcluster"]["api"]["queue"]);
  trace!("main() end");
  Ok(())
}