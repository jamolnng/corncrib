#[cfg(test)]
mod tests {
  extern crate corncrib;

  use corncrib::Database;
  #[test]
  fn create_empty_db() -> Result<(), String> {
    let mut db = Database::new("empty")?;
    assert!(db.directory().exists());
    let dir = match std::fs::read_dir(db.directory()) {
      Ok(t) => t,
      Err(err) => panic!("Failed to read dir {}", err),
    };
    assert_eq!(dir.count(), 0);
    db.destroy()?;
    assert!(!db.directory().exists());
    Ok(())
  }

  #[test]
  fn load_db() -> Result<(), String> {
    let db = Database::new("empty")?;
    let mut db2 = Database::read(db.directory())?;
    assert!(db2.directory().exists());
    let dir = match std::fs::read_dir(db2.directory()) {
      Ok(t) => t,
      Err(err) => panic!("Failed to read dir {}", err),
    };
    assert_eq!(dir.count(), 0);
    db2.destroy()?;
    assert!(!db2.directory().exists());
    Ok(())
  }
}
