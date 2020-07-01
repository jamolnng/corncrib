#[cfg(test)]
mod tests {
  use corncrib::Database;
  #[test]
  fn set_get_key_value() -> Result<(), String> {
    let mut db = Database::new("empty")?;
    let key = "simplestring";
    let value = "some highly valuable value";
    db.insert(key, value)?;
    let val: String = match db.get(key) {
      Some(v) => v,
      None => return Err(String::from("Failed to get value from DB")),
    };
    assert_eq!(val, String::from(value));
    db.destroy()?;
    Ok(())
  }
}
