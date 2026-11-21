import logging
import os
from pathlib import Path

from .nix import nix_env_set, nix_env_switch_profile


class Kontor:
    def __init__(self) -> None:
        self.home_dir = get_home_dir()
        self.profile_dir = get_xdg_state_home(self.home_dir) / "kontor" / "profile"
        self.config_dir = self.home_dir / "kontor"

    def switch(self) -> None:
        logging.info("building user environment")
        nix_env_set(
            profile=self.profile_dir,
            file=self.config_dir,
            attr="user-env",
        )

        # ensure user profile link and nix.conf like are setup
        nix_env_switch_profile(self.profile_dir)
        symlink(
            link=self.home_dir / ".config" / "nix" / "nix.conf",
            target=self.profile_dir / "etc" / "nix" / "nix.conf",
        )


def symlink(link: Path, target: Path) -> None:
    """Create a symbolic link from `link` to `target`.

    This function will create all parent directories of `link`.
    """
    link.parent.mkdir(parents=True, exist_ok=True)

    try:
        os.symlink(src=target, dst=link, target_is_directory=False)
    except FileExistsError as err:
        existing_link_target = link.readlink()
        if existing_link_target != target:
            msg = "symbolic link already exists but points to a different target"
            raise RuntimeError(msg) from err


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
