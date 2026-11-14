use std::{ffi::OsStr, path::Path, process::Command};

use anyhow::{Error, anyhow};
use log::debug;

/// Execute `nix-build`.
///
/// See https://nix.dev/manual/nix/2.35/command-ref/nix-build.html.
#[derive(Debug)]
pub struct NixBuild<'a> {
    path: Option<&'a Path>,
    attr: Option<&'a OsStr>,
    out_link: Option<&'a Path>,
}

impl<'a> NixBuild<'a> {
    pub fn new() -> Self {
        Self {
            path: None,
            attr: None,
            out_link: None,
        }
    }

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

    pub fn execute(&mut self) -> Result<(), Error> {
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

        execute(cmd, "nix-build")
    }
}

/// Execute `nix-env --set`.
///
/// See https://nix.dev/manual/nix/2.35/command-ref/nix-env/set.html.
#[derive(Debug)]
pub struct NixEnvSet<'a> {
    drvname: &'a OsStr,
    profile: Option<&'a Path>,
}

impl<'a> NixEnvSet<'a> {
    /// `drvname` is either interpreted as a path to a store derivation if it contains a `/`,
    /// or otherwise as an attribute in the default Nix expression (`~/.nix-defexpr`).
    pub fn new(drvname: &'a OsStr) -> Self {
        Self {
            drvname,
            profile: None,
        }
    }

    pub fn profile(&mut self, profile: &'a Path) -> &mut Self {
        self.profile = Some(profile);
        self
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        let mut cmd = Command::new("nix-env");
        cmd.arg("--set");

        if let Some(profile) = self.profile {
            cmd.arg("--profile").arg(profile);
        }
        cmd.arg(self.drvname);

        execute(cmd, "nix-env --set")
    }
}

/// Executes `nix-env --switch-profile`.
///
/// See https://nix.dev/manual/nix/2.35/command-ref/nix-env/switch-profile.html.
#[derive(Debug)]
pub struct NixEnvSwitchProfile<'a> {
    path: &'a Path,
}

impl<'a> NixEnvSwitchProfile<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        let mut cmd = Command::new("nix-env");
        cmd.arg("--switch-profile");

        cmd.arg(self.path);

        execute(cmd, "nix-env --switch-profile")
    }
}

fn execute(mut cmd: Command, cmd_name: &str) -> Result<(), Error> {
    debug!("executing {:?}", cmd);

    let status = cmd.status()?;
    status
        .success()
        .ok_or_else(|| anyhow!("`{cmd_name}` terminated unsuccessfully, {status}"))
}
