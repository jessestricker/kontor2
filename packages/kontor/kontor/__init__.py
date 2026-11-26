import logging
import os
from pathlib import Path

import renameat2

from .nix import nix_env_set, nix_env_switch_profile


class Kontor:
    def __init__(self) -> None:
        self.home_dir = get_home_dir()
        self.profile_dir = get_xdg_state_home(self.home_dir) / "kontor" / "profile"
        self.config_dir = self.home_dir / "kontor"
        self.kontor_home_dir = self.config_dir / "home"

    def link(self, home_file: Path) -> None:
        relative_name = home_file.relative_to(self.home_dir, walk_up=False)
        kontor_file = self.kontor_home_dir / relative_name

        kontor_file.parent.mkdir(parents=True, exist_ok=True)
        renameat2.rename(home_file, kontor_file, replace=False)
        home_file.symlink_to(kontor_file)

    def switch(self) -> None:
        logging.info("building user environment")
        nix_env_set(
            profile=self.profile_dir,
            file=self.config_dir,
            attr="user-env",
        )

        # ensure user profile link and nix.conf like are setup
        nix_env_switch_profile(self.profile_dir)
        ensure_symlink_with_parents(
            link=self.home_dir / ".config" / "nix" / "nix.conf",
            target=self.profile_dir / "etc" / "nix" / "nix.conf",
        )


def ensure_symlink_with_parents(link: Path, target: Path) -> None:
    """Create a symbolic link from `link` to `target`.

    This function will create all parent directories of `link`.
    This function is idempotent.
    """
    link.parent.mkdir(parents=True, exist_ok=True)
    ensure_symlink(link, target)


def ensure_symlink(link: Path, target: Path) -> None:
    """Create a symbolic link from `link` to `target`.

    This function is idempotent.
    """
    try:
        link.symlink_to(target, target_is_directory=False)
    except FileExistsError as err:
        existing_link_target = link.readlink()
        if existing_link_target != target:
            msg = "Symbolic link already exists, but with a different target."
            raise FileExistsError(msg) from err


def get_xdg_state_home(home_dir: Path) -> Path:
    state_home = os.environ.get("XDG_STATE_HOME")
    if state_home is None or state_home == "":
        return home_dir / ".local" / "state"
    return Path(state_home)


def get_home_dir() -> Path:
    home_dir = os.environ.get("HOME")
    if home_dir is None or home_dir == "":
        msg = 'environment variable "HOME" is undefined or empty'
        raise RuntimeError(msg)
    return Path(home_dir)
