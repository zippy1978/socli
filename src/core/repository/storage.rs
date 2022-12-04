use std::{fs, path::PathBuf};

use async_trait::async_trait;
use dirs::home_dir;
use serde_json::{from_str, to_string_pretty, Value};

use super::error::RepoError;

#[async_trait]
pub trait StorageRepo {
    async fn get_collection(&self, name: &str) -> Result<Option<Value>, RepoError>;

    async fn set_collection(&self, name: &str, data: &Value) -> Result<(), RepoError>;
}

pub struct StorageRepoImpl {
    root_dir: PathBuf,
}

impl StorageRepoImpl {
    pub fn new() -> Self {
        // Create root folder
        let mut root_dir = home_dir().expect("failed to retrieve user home directory");
        root_dir.push(".socli");
        root_dir.push("storage");
        fs::create_dir_all(&root_dir).expect("failed to create storage root directory");

        Self { root_dir }
    }

    fn collection_path(&self, name: &str) -> PathBuf {
        let mut path = self.root_dir.clone();
        path.push(format!("{}.json", name));
        path
    }
}

#[async_trait]
impl StorageRepo for StorageRepoImpl {
    async fn get_collection(&self, name: &str) -> Result<Option<Value>, RepoError> {
        // Get colleciton path
        let path = self.collection_path(name);

        // Check if file exists
        if !path.exists() {
            return Ok(None);
        }

        // Read file content
        let json = match fs::read_to_string(path) {
            Ok(res) => res,
            Err(err) => return Err(RepoError::Read(err.to_string())),
        };

        // Parse and return
        match from_str(&json) {
            Ok(v) => Ok(Some(v)),
            Err(err) => Err(RepoError::Read(err.to_string())),
        }
    }

    async fn set_collection(&self, name: &str, data: &Value) -> Result<(), RepoError> {
        // Get colleciton path
        let path = self.collection_path(name);

        // Serialize content as string
        let json = match to_string_pretty(&data) {
            Ok(res) => res,
            Err(err) => return Err(RepoError::Write(err.to_string())),
        };

        match fs::write(path, json) {
            Ok(_) => (),
            Err(err) => return Err(RepoError::Write(err.to_string())),
        }

        Ok(())
    }
}
