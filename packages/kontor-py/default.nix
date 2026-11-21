{
  lib,
  installShellFiles,
  python314Packages,
}:
let
  pyprojectToml = lib.importTOML ./pyproject.toml;
in
python314Packages.buildPythonApplication {
  pname = "kontor";
  version = pyprojectToml.project.version;
  pyproject = true;

  src = lib.sources.sourceByGlobs ./. [
    "**/*.py"
    "pyproject.toml"
  ];

  nativeBuildInputs = [ installShellFiles ];
  build-system = with python314Packages; [ hatchling ];
  dependencies = with python314Packages; [ click ];

  postInstall = ''
    installShellCompletion --cmd kontor \
      --bash <(_KONTOR_COMPLETE=bash_source $out/bin/kontor)
  '';
}
