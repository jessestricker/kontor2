# Kontor

Declarative Nix user environment and channels.

## How it works

- The `kontor` CLI sets up a Nix profile at `$XDG_STATE_HOME/kontor/profile`.
  - This contains the user environment directory `user-env`, which has the usual
    `bin`, `lib`, `share`, etc.
  - The profile also contains the channels directory `channels`, which is
    structured in a way that Nix' `builtins.findFile` can use it for lookup
    paths.
- To be able to run programs without typing the full path, the _Nix user profile
  link_ is setup to point to the `user-env` directory from the Kontor profile.
  - This link is located at `~/.nix-profile`.
  - The `bin` subdirectory of this link is part of the `PATH` environment
    variable.
- To be able to use the Nix channels as a lookup path (like `<nixpkgs>`), the
  _Nix user channel link_ is setup to point to the `channels` directory from the
  Kontor profile.
  - This link is located at `~/.nix-defexpr/channels`.
  - It is used as the default value for the `builtins.nixPath` expression.
