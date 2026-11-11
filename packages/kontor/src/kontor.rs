use std::{env, path::PathBuf};

use anyhow::{Context, Error, anyhow};
use log::info;

use crate::nix::NixBuild;

#[derive(Debug)]
pub struct Kontor {
    home_dir: PathBuf,
    kontor_dir: PathBuf,
}

impl Kontor {
    const KONTOR_DIR_NAME: &str = "kontor";
    const USER_ENV_ATTR: &str = "user-env";

    pub fn new() -> Self {
        let home_dir = env::home_dir().expect("current user should have a home dir");
        let kontor_dir = home_dir.join(Self::KONTOR_DIR_NAME);
        Self {
            home_dir,
            kontor_dir,
        }
    }

    pub fn switch(&self) -> Result<(), Error> {
        info!("building user environment");
        NixBuild::default()
            .path(&self.kontor_dir)
            .attr(Self::USER_ENV_ATTR.as_ref())
            .run()
            .context(anyhow!("failed to build user environment"))?;

        Ok(())
    }
}
