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

        logging.info("switching user profile")
        nix_env_switch_profile(self.profile_dir)


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
