use crate::error::AppError;
use crate::profile::Profile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Formatter;

/// Represents a user config.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// Collection of config profiles.
    profiles: HashMap<String, Profile>,
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.profiles {
            writeln!(f, "Profile: {key}")?;
            writeln!(f, "{value}")?;
        }
        Ok(())
    }
}

impl Config {
    pub fn from_json(json: &str) -> Result<Self, AppError> {
        Ok(serde_json::from_str::<Self>(json)?)
    }

    /// Adds or updates a new profile.
    #[inline]
    pub fn insert(&mut self, profile_name: &str, profile: &Profile) -> Option<Profile> {
        self.profiles.insert(profile_name.into(), profile.clone())
    }

    /// Deletes a profile.
    #[inline]
    pub fn remove(&mut self, profile_name: &str) -> Option<Profile> {
        self.profiles.remove(profile_name)
    }

    /// Gets a specific profile.
    #[inline]
    pub fn get(&self, profile_name: &str) -> Option<&Profile> {
        self.profiles.get(profile_name)
    }

    /// Creates a JSON string.
    #[inline]
    pub fn to_json(&self) -> Result<String, AppError> {
        serde_json::to_string(self).map_err(AppError::Json)
    }
}

#[allow(clippy::indexing_slicing, clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_json() {
        let mut config = Config::default();
        assert_eq!(config.to_json().unwrap(), r#"{"profiles":{}}"#);

        config.insert("test", &Profile::new("abc", "def"));
        assert_eq!(
            config.to_json().unwrap(),
            r#"{"profiles":{"test":{"name":"abc","email":"def"}}}"#
        );
    }

    #[test]
    fn test_from_json() {
        let config = Config::from_json(r#"{"profiles":{"test":{"name":"abc","email":"def"}}}"#).unwrap();
        assert_eq!(config.profiles.len(), 1);
        assert_eq!(config.profiles["test"].name, "abc");
        assert_eq!(config.profiles["test"].email, "def");

        let config = Config::from_json(r#"{"profiles":{}}"#).unwrap();
        assert_eq!(config.profiles.len(), 0);

        let config = Config::from_json("invalid");
        assert!(config.is_err());
    }

    #[test]
    fn test_insert() {
        let mut config = Config::default();
        config.insert("test", &Profile::new("abc", "def"));

        assert_eq!(config.profiles.len(), 1);
        assert_eq!(config.profiles["test"].name, "abc");
        assert_eq!(config.profiles["test"].email, "def");
    }

    #[test]
    fn test_remove() {
        let mut config = Config::default();
        config.insert("test", &Profile::new("abc", "def"));

        config.remove("test");
        assert_eq!(config.profiles.len(), 0);

        config.remove("test");
        assert_eq!(config.profiles.len(), 0);
    }

    #[test]
    fn test_get() {
        let mut config = Config::default();
        config.insert("test", &Profile::new("abc", "def"));

        let profile = config.get("test");
        assert_eq!(profile.unwrap().name, "abc");
        assert_eq!(profile.unwrap().email, "def");
    }
}
