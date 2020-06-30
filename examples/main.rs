extern crate corncrib;

use corncrib::Database;

fn main() {
  if let Ok(mut database) = Database::new("test") {
    match database.insert("test", "test") {
      Err(_) => println!("Error"),
      _ => {}
    }
    println!("{:#?}", database);

    if let Ok(mut database) = Database::read(database.directory()) {
      println!("{:#?}", database);
      match database.destroy() {
        Ok(_) => {}
        Err(_) => println!("Error destroying database"),
      }
    }
  }
}
