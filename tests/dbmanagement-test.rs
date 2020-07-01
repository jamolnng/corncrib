#[cfg(test)]
mod tests {
  use corncrib::Database;
  #[test]
  fn create_empty_db() -> Result<(), String> {
    let mut db = Database::new("empty")?;
    assert!(db.directory().exists());
    let dir = std::fs::read_dir(db.directory()).expect("Failed to read dir");
    assert_eq!(dir.count(), 0);
    db.destroy()?;
    assert!(!db.directory().exists());
    Ok(())
  }

  #[test]
  fn load_db() -> Result<(), String> {
    Database::new("empty")?;
    let mut db = Database::read("empty")?;
    assert!(db.directory().exists());
    let dir = std::fs::read_dir(db.directory()).expect("Failed to read dir");
    assert_eq!(dir.count(), 0);
    db.destroy()?;
    assert!(!db.directory().exists());
    Ok(())
  }
}
