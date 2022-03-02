use std::fs::OpenOptions;
use std::io::Write;

pub fn log(content: String, filename: &str) {
  println!("{}", content);

  if !filename.is_empty() {
    let mut file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(filename)
      .unwrap();

    if let Err(e) = writeln!(file, "{}", content) {
      eprintln!("Couldn't write to file: {}", e);
    }
  }
}
