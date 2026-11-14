use std::{env, fs, path::PathBuf};

use anyhow::{Context, Error, anyhow};
use log::info;

use crate::{
    nix::{NixBuild, NixEnvSet, NixEnvSwitchProfile},
    sys::xdg_state_home,
};

#[derive(Debug)]
pub struct Kontor {
    /// The directory which contains the kontor configuration.
    kontor_dir: PathBuf,

    /// The kontor Nix profile directory.
    kontor_profile_dir: PathBuf,
}

impl Kontor {
    const KONTOR_DIR_NAME: &str = "kontor";
    const USER_ENV_ATTR: &str = "user-env";
    const KONTOR_PROFILE_PATH: &str = "kontor/profile";

    pub fn new() -> Self {
        let home_dir = env::home_dir().expect("current user should have a home dir");
        let kontor_dir = home_dir.join(Self::KONTOR_DIR_NAME);
        let kontor_profile_dir = xdg_state_home(&home_dir).join(Self::KONTOR_PROFILE_PATH);

        Self {
            kontor_dir,
            kontor_profile_dir,
        }
    }

    pub fn switch(&self) -> Result<(), Error> {
        let temp_dir = tempfile::tempdir()?;
        let user_env_result_link = temp_dir.path().join("result");

        // 1. Build the user environment derivation.
        info!("building user environment");
        NixBuild::new()
            .path(&self.kontor_dir)
            .attr(Self::USER_ENV_ATTR.as_ref())
            .out_link(&user_env_result_link)
            .execute()
            .context(anyhow!("failed to build user environment"))?;

        // 2. set kontor profile to contain user environment derivation
        info!("setting kontor profile");
        fs::create_dir_all(
            self.kontor_profile_dir
                .parent()
                .expect("kontor profile directory should have a parent"),
        )?;
        NixEnvSet::new(user_env_result_link.as_ref())
            .profile(&self.kontor_profile_dir)
            .execute()?;

        // 3. switch Nix's user profile link to the kontor profile
        info!("switching user profile link");
        NixEnvSwitchProfile::new(&self.kontor_profile_dir).execute()?;

        Ok(())
    }
}
