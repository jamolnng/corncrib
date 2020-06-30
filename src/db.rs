#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Error {
  kind: ErrorKind,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
  IO(std::io::ErrorKind),
  DoesNotExist,
  DirectoryError,
  InvalidEntry,
  Other,
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Serialize {
  fn serialize(&self) -> String;
  fn deserialize(item: &String) -> Option<Self>
  where
    Self: Sized,
  {
    None
  }
}

#[derive(Debug)]
pub struct Database {
  directory: PathBuf,
  kv: HashMap<String, String>,
}

impl Error {
  pub fn kind(&self) -> &ErrorKind {
    &self.kind
  }
}

impl From<std::io::Error> for Error {
  fn from(error: std::io::Error) -> Self {
    Error {
      kind: ErrorKind::IO(error.kind()),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self.kind)
  }
}

impl From<Error> for String {
  fn from(error: Error) -> Self {
    error.to_string()
  }
}

impl Database {
  pub fn new(name: &str) -> Result<Database> {
    let mut directory = PathBuf::from(".db");
    directory.push(name);
    std::fs::create_dir_all(directory.as_path())?;
    Ok(Database {
      directory: directory.canonicalize()?,
      kv: HashMap::new(),
    })
  }

  pub fn destroy(&mut self) -> Result<()> {
    self.kv.clear();
    std::fs::remove_dir_all(&self.directory)?;
    Ok(())
  }

  pub fn read(path: &Path) -> Result<Database> {
    let mut kv: HashMap<String, String> = HashMap::new();
    let directory = PathBuf::from(path);
    if !directory.exists() {
      return Err(Error {
        kind: ErrorKind::DoesNotExist,
      });
    }
    match directory.read_dir() {
      Ok(dir) => {
        for try_file in dir {
          let file: std::fs::DirEntry = try_file?;
          let fpath = file.path();
          if !fpath.is_file() {
            continue;
          }
          if let Some(ext) = fpath.extension().and_then(std::ffi::OsStr::to_str) {
            if ext == "kv" {
              if let Some(fname) = fpath.file_stem().and_then(std::ffi::OsStr::to_str) {
                kv.insert(String::from(fname), std::fs::read_to_string(fpath)?);
              }
            }
          }
        }
      }
      Err(err) => {
        return Err(Error {
          kind: ErrorKind::DirectoryError,
        })
      }
    }
    Ok(Database {
      directory: directory,
      kv: kv,
    })
  }

  pub fn directory(&self) -> &PathBuf {
    &self.directory
  }

  pub fn insert<K: Serialize, V: Serialize>(&mut self, key: K, value: V) -> Result<()> {
    let k = key.serialize();
    let v: String = value.serialize();
    let mut kpath = PathBuf::from(self.directory.as_path());
    kpath.push(&k);
    kpath.set_extension("kv");
    match OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(kpath)
    {
      Ok(mut file) => {
        file.write_all(v.as_bytes())?;
      }
      Err(err) => {
        return Err(Error {
          kind: ErrorKind::IO(err.kind()),
        })
      }
    }
    self.kv.insert(k, v);
    Ok(())
  }

  pub fn remove<K: Serialize, V: Serialize>(&mut self, key: K) -> Result<()> {
    let k = key.serialize();
    let mut kpath = PathBuf::from(self.directory.as_path());
    kpath.push(k);
    kpath.set_extension("kv");
    std::fs::remove_file(kpath)?;
    Ok(())
  }

  pub fn contains_key<K: Serialize>(&self, key: K) -> bool {
    self.kv.contains_key(&key.serialize())
  }

  pub fn get<K: Serialize, V: Serialize>(&self, key: K) -> Option<V> {
    self.kv.get(&key.serialize()).and_then(V::deserialize)
  }
}

impl Serialize for String {
  fn serialize(&self) -> String {
    self.clone()
  }
  fn deserialize(item: &String) -> Option<Self> {
    Some(item.clone())
  }
}

impl Serialize for &str {
  fn serialize(&self) -> String {
    String::from(*self)
  }
}
