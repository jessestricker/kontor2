use std::{ffi::OsStr, path::Path, process::Command};

use anyhow::{Error, anyhow};
use log::debug;

/// Execute `nix-build`.
///
/// See https://nix.dev/manual/nix/2.35/command-ref/nix-build.html.
#[derive(Debug, Default)]
pub struct NixBuild<'a> {
    path: Option<&'a Path>,
    attr: Option<&'a OsStr>,
    out_link: Option<&'a Path>,
}

impl<'a> NixBuild<'a> {
    pub fn path(&mut self, path: &'a Path) -> &mut Self {
        self.path = Some(path);
        self
    }

    pub fn attr(&mut self, attr: &'a OsStr) -> &mut Self {
        self.attr = Some(attr);
        self
    }

    pub fn out_link(&mut self, out_link: &'a Path) -> &mut Self {
        self.out_link = Some(out_link);
        self
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let mut cmd = Command::new("nix-build");
        if let Some(path) = self.path {
            cmd.arg(path);
        }
        if let Some(attr) = self.attr {
            cmd.arg("--attr").arg(attr);
        }
        if let Some(out_link) = self.out_link {
            cmd.arg("--out-link").arg(out_link);
        } else {
            cmd.arg("--no-out-link");
        }

        debug!("running {:?}", cmd);
        let status = cmd.status()?;
        status
            .success()
            .ok_or_else(|| anyhow!("nix-build terminated unsuccessfully, {status}"))
    }
}
