{...}: {
  perSystem = {
    pkgs,
    config,
    ...
  }: let
    # TODO: change this to your crate's name
    crateName = "purpur";
  in {
    # declare projects
    # TODO: change this to your crate's path
    nci.projects.${crateName}.path = ./.;
    # configure crates
    nci.crates.${crateName} = {};
  };
}
