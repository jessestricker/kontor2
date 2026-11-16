use std::{env, os::unix::fs::symlink, path::PathBuf};

use anyhow::{Context, Error, anyhow};
use log::info;

use crate::{
    nix::{NixBuild, NixEnvSet, NixEnvSwitchProfile},
    sys::{ensure_parent_exists, xdg_state_home},
};

#[derive(Debug)]
pub struct Kontor {
    home_dir: PathBuf,
    /// The directory containing the configuration.
    config_dir: PathBuf,
    /// The Nix profile containing the built configuration.
    profile_dir: PathBuf,
}

impl Kontor {
    pub fn new() -> Self {
        let home_dir = env::home_dir().expect("current user should have a home dir");

        let config_dir = home_dir.join("kontor");
        let profile_dir = xdg_state_home(&home_dir).join("kontor/profile");

        Self {
            home_dir,
            config_dir,
            profile_dir,
        }
    }

    pub fn switch(&self) -> Result<(), Error> {
        let temp_dir = tempfile::tempdir()?;
        let config_result_link = temp_dir.path().join("config-result");

        // 1. Build the configuration derivation.
        info!("building configuration");
        NixBuild::new()
            .path(&self.config_dir)
            .attr("config".as_ref())
            .out_link(&config_result_link)
            .execute()
            .context(anyhow!("failed to build configuration"))?;

        // 2. Set Kontor profile to contain built configuration.
        info!("setting kontor profile");
        ensure_parent_exists(&self.profile_dir)?;
        NixEnvSet::new(config_result_link.as_ref())
            .profile(&self.profile_dir)
            .execute()?;

        // 3. Switch Nix's user profile link.
        info!("switching user profile link");
        NixEnvSwitchProfile::new(&self.profile_dir.join("user-env")).execute()?;

        // 4. Switch Nix's user channel link.
        info!("switching user channel link");
        let user_channel_link = self.home_dir.join(".nix-defexpr/channels");
        ensure_parent_exists(&user_channel_link).context(anyhow!(
            "failed to create parent directory of user channel link"
        ))?;
        symlink(self.profile_dir.join("channels"), user_channel_link)?;

        Ok(())
    }
}
