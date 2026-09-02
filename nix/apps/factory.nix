{
  writeShellApplication,
  git,
  openspec,
}:

writeShellApplication {
  name = "factory";
  runtimeInputs = [
    git
    openspec
  ];
  text = ''
    repository_root="$(git rev-parse --show-toplevel)"
    exec "$repository_root/scripts/factory" "$@"
  '';
}
