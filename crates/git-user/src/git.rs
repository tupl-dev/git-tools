use crate::error::AppError;
use crate::profile::Profile;
use gix::config::{AsBStr, File, Source};
use std::fs;
use std::path::{Path, PathBuf};

/// Updates a git config file using a [`Profile`].
pub fn update_config(path: impl AsRef<Path>, profile: &Profile) -> Result<(), AppError> {
    let config_path = get_config_path(path)?;
    let mut config = get_config_file(&config_path)?;
    for map in profile.git_mappings() {
        if let Some(value) = map.value {
            config
                .set_raw_value_by(map.section, map.subsection.map(AsBStr::as_bstr), map.key, value)
                .map_err(|e| AppError::Git(e.to_string()))?;
        } else if let Ok(mut section) = config.section_mut(map.section, None) {
            section.remove(map.key);
        }
    }
    fs::write(&config_path, config.to_string()).map_err(|e| AppError::File(e.to_string()))
}

/// Gets the path to `.git/config`.
#[inline]
fn get_config_path(path: impl AsRef<Path>) -> Result<PathBuf, AppError> {
    gix::discover(path)
        .map(|repo| repo.git_dir().join("config"))
        .map_err(|e| AppError::Git(e.to_string()))
}

/// Gets the `.git/config` file.
#[inline]
fn get_config_file(path: impl AsRef<Path>) -> Result<File, AppError> {
    File::from_path_no_includes(path.as_ref().to_path_buf(), Source::Local).map_err(|e| AppError::Git(e.to_string()))
}
