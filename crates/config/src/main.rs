fn main() {
  let config = config::AppConfig::from_env();

  println!("{config:#?}");
}
