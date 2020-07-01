use corncrib::Database;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    show_help();
    return;
  }
  let dbname = match args.iter().position(|s| s == "-n") {
    Some(v) => {
      if args.len() <= v {
        println!("You must specify a database name with -n <name>");
        show_help();
        return;
      }
      &args[v + 1]
    }
    None => {
      println!("You must specify a database name with -n <name>");
      show_help();
      return;
    }
  };
  if let Some(_) = args.iter().find(|&s| *s == "-c") {
    match Database::new(dbname) {
      Ok(_) => println!("Created db {}", dbname),
      Err(_) => println!("Failed to create db {}", dbname),
    }
    return;
  }
  if let Some(_) = args.iter().find(|&s| *s == "-d") {
    let mut db = match Database::read(dbname.as_str()) {
      Ok(db) => db,
      Err(_) => {
        println!("Failed to read db {}", dbname);
        return;
      }
    };
    match db.destroy() {
      Ok(_) => println!("Destroyed db {}", dbname),
      Err(_) => println!("Failed to destroy db {}", dbname),
    }
    return;
  }
  if let Some(_) = args.iter().find(|&s| *s == "-s") {
    let mut db = match Database::read(dbname.as_str()) {
      Ok(db) => db,
      Err(_) => {
        println!("Failed to read db {}", dbname);
        return;
      }
    };
    let key = match args.iter().position(|s| s == "-k") {
      Some(v) => {
        if args.len() <= v {
          println!("You must specify a key to set with -k <name>");
          show_help();
          return;
        }
        &args[v + 1]
      }
      None => {
        println!("You must specify a key to set with -k <name>");
        show_help();
        return;
      }
    };
    let value = match args.iter().position(|s| s == "-v") {
      Some(v) => {
        if args.len() <= v {
          println!("You must specify a value to set with -v <value>");
          show_help();
          return;
        }
        &args[v + 1]
      }
      None => {
        println!("You must specify a value to set with -v <value>");
        show_help();
        return;
      }
    };
    match db.insert(key, value) {
      Ok(_) => println!("Inserted key {}, value {} into db {}", key, value, dbname),
      Err(_) => println!(
        "Failed to insert key {}, value {} into db {}",
        key, value, dbname
      ),
    }
    return;
  }
  if let Some(_) = args.iter().find(|&s| *s == "-g") {
    let db = match Database::read(dbname.as_str()) {
      Ok(db) => db,
      Err(_) => {
        println!("Failed to read db {}", dbname);
        return;
      }
    };
    let key = match args.iter().position(|s| s == "-k") {
      Some(v) => {
        if args.len() <= v {
          println!("You must specify a key to set with -k <name>");
          show_help();
          return;
        }
        &args[v + 1]
      }
      None => {
        println!("You must specify a key to set with -k <name>");
        show_help();
        return;
      }
    };
    match db.get::<&String, String>(key) {
      Some(value) => println!("Got value {} from key {} in db {}", value, key, dbname),
      None => println!("Failed to get value from key {} in db {}", key, dbname),
    }
    return;
  }
}

fn show_help() {}
