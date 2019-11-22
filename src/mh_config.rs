use std::{
  fs,
  io::Read
};

pub fn read_config(path: &str) -> Result<yaml_rust::Yaml, Box<dyn std::error::Error>> {
  trace!("mh_config::read_config(path: '{}') begin", path);
  let mut file = fs::File::open(path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let config = &yaml_rust::YamlLoader::load_from_str(&contents).unwrap()[0];
  trace!("mh_config::read_config(path: '{}') end", path);
  Ok(config.clone())
}