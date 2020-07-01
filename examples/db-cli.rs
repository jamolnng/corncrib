use corncrib::Database;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    return;
  }
  let check_len = |n| {
    if args.len() <= n {
      None
    } else {
      Some(&args[n + 1])
    }
  };
  let dbname = args
    .iter()
    .position(|s| s == "-n")
    .and_then(check_len)
    .expect("You must specify a database name with -n <name>");
  if let Some(_) = args.iter().find(|&s| *s == "-c") {
    Database::new(dbname).expect("Failed to create db");
    println!("Created db {}", dbname);
    return;
  }
  if let Some(_) = args.iter().find(|&s| *s == "-d") {
    let mut db = Database::read(dbname.as_str()).expect("Failed to read db");
    db.destroy().expect("Failed to destroy db");
    println!("Destroyed db {}", dbname);
    return;
  }
  if let Some(_) = args.iter().find(|&s| *s == "-s") {
    let mut db = Database::read(dbname.as_str()).expect("Failed to read db");
    let key = args
      .iter()
      .position(|s| s == "-k")
      .and_then(check_len)
      .expect("You must specify a key to set with -k <key>");
    let value = args
      .iter()
      .position(|s| s == "-v")
      .and_then(check_len)
      .expect("You must specify a value to set with -v <value>");
    db.insert(key, value).expect("Failed to insert key into db");
    println!("Inserted \"{}\": \"{}\" into db {}", key, value, dbname);
    return;
  }
  if let Some(_) = args.iter().find(|&s| *s == "-g") {
    let db = Database::read(dbname.as_str()).expect("Failed to read db");
    let key = args
      .iter()
      .position(|s| s == "-k")
      .and_then(check_len)
      .expect("You must specify a key to set with -k <key>");
    let value: String = db.get(key).expect("Failed to get value from key in db");
    println!(
      "Got value \"{}\" associated with key \"{}\" from db {}",
      value, key, dbname
    );
    return;
  }
}
