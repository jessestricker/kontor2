import subprocess
from pathlib import Path


def nix_env_set(
    profile: Path,
    file: Path,
    attr: str,
) -> None:
    """Run `nix-env --set`.

    See https://nix.dev/manual/nix/2.36/command-ref/nix-env/set.html.
    """
    args = ["nix-env", "--set"]
    args += ["--profile", profile]
    args += ["--file", file]
    args += ["--attr", attr]
    subprocess.run(args, check=True)


def nix_env_switch_profile(path: Path) -> None:
    """Run `nix-env --switch-profile`.

    See https://nix.dev/manual/nix/2.36/command-ref/nix-env/switch-profile.html.
    """
    args = ["nix-env", "--switch-profile"]
    args += [path]
    subprocess.run(args, check=True)
