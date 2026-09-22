use serde::{Deserialize, Serialize};

/// Represents a profile in the config.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// Corresponds to `user.name`.
    pub name: String,

    /// Corresponds to `user.email`.
    pub email: String,

    /// Corresponds to `user.signingKey`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_key: Option<String>,

    /// Corresponds to `user.sshCommand`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_command: Option<String>,

    /// Corresponds to `gpg.format`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpg_format: Option<String>,
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Email: {}", self.email)?;
        if let Some(signing_key) = &self.signing_key {
            writeln!(f, "Signing Key: {signing_key}")?;
        }
        if let Some(ssh_command) = &self.ssh_command {
            writeln!(f, "SSH Command: {ssh_command}")?;
        }
        if let Some(gpg_format) = &self.gpg_format {
            writeln!(f, "GPG Format: {gpg_format}")?;
        }
        Ok(())
    }
}

impl Profile {
    /// Creates a new [`Profile`].
    #[inline]
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
            signing_key: None,
            ssh_command: None,
            gpg_format: None,
        }
    }

    /// Sets `user.signingKey`.
    #[inline]
    pub fn with_signing_key(mut self, signing_key: Option<impl Into<String>>) -> Self {
        self.signing_key = signing_key.map(Into::into);
        self
    }

    /// Sets `user.sshCommand`.
    #[inline]
    pub fn with_ssh_command(mut self, ssh_command: Option<impl Into<String>>) -> Self {
        self.ssh_command = ssh_command.map(Into::into);
        self
    }

    /// Sets `gpg.format`.
    #[inline]
    pub fn with_gpg_format(mut self, gpg_format: Option<impl Into<String>>) -> Self {
        self.gpg_format = gpg_format.map(Into::into);
        self
    }

    /// Gets the git config mappings.
    pub fn git_mappings(&self) -> impl IntoIterator<Item = GitFieldMap<'_>> {
        [
            GitFieldMap::new("user", None, "name", Some(self.name.as_str())),
            GitFieldMap::new("user", None, "email", Some(self.email.as_str())),
            GitFieldMap::new("user", None, "signingKey", self.signing_key.as_deref()),
            GitFieldMap::new("core", None, "sshCommand", self.ssh_command.as_deref()),
            GitFieldMap::new("gpg", None, "format", self.gpg_format.as_deref()),
        ]
    }
}

/// Represents a mapping from [`Profile`] to an entry in the git config.
#[derive(Clone, Debug)]
pub struct GitFieldMap<'a> {
    pub section: &'a str,
    pub subsection: Option<&'a str>,
    pub key: &'a str,
    pub value: Option<&'a [u8]>,
}

impl<'a> GitFieldMap<'a> {
    /// Creates a new [`GitFieldMapping`].
    #[inline]
    pub fn new(section: &'a str, subsection: Option<&'a str>, key: &'a str, value: Option<&'a str>) -> Self {
        Self {
            section,
            subsection,
            key,
            value: value.map(str::as_bytes),
        }
    }
}

#[cfg(test)]
mod profile_tests {
    use super::*;

    #[test]
    fn test_new() {
        let profile = Profile::new("abc", "def");
        assert_eq!(profile.name, "abc");
        assert_eq!(profile.email, "def");
        assert_eq!(profile.signing_key, None);
        assert_eq!(profile.ssh_command, None);
    }

    #[test]
    fn test_with_signing_key() {
        let profile = Profile::new("a", "b").with_signing_key(Some("c"));
        assert_eq!(profile.signing_key.as_deref(), Some("c"));
    }

    #[test]
    fn test_with_ssh_command() {
        let profile = Profile::new("a", "b").with_ssh_command(Some("c"));
        assert_eq!(profile.ssh_command.as_deref(), Some("c"));
    }

    #[test]
    fn test_with_gpg_format() {
        let profile = Profile::new("a", "b").with_gpg_format(Some("c"));
        assert_eq!(profile.gpg_format.as_deref(), Some("c"));
    }
}

#[cfg(test)]
mod git_field_map_tests {
    use super::*;

    #[test]
    fn test_new() {
        let map = GitFieldMap::new("user", None, "name", None);
        assert_eq!(map.section, "user");
        assert_eq!(map.subsection, None);
        assert_eq!(map.key, "name");
        assert_eq!(map.value, None);

        let map = GitFieldMap::new("a", Some("b"), "c", Some("d"));
        assert_eq!(map.section, "a");
        assert_eq!(map.subsection, Some("b"));
        assert_eq!(map.key, "c");
        assert_eq!(map.value, Some(b"d".as_slice()));
    }
}
