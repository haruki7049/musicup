{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    flake-compat.url = "github:edolstra/flake-compat";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
      ];

      perSystem =
        { pkgs, lib, ... }:
        let
          musicup = pkgs.stdenv.mkDerivation {
            name = "musicup";
            src = lib.cleanSource ./.;

            nativeBuildInputs = [
              pkgs.scons
            ];

            buildPhase = ''
              scons
            '';

            installPhase = ''
              mkdir -p $out/bin
              cp .build/musicup $out/bin
            '';
          };
        in
        {
          treefmt = {
            projectRootFile = "flake.nix";
            programs.nixfmt.enable = true;
            programs.clang-format.enable = true;
            programs.ruff-format.enable = true;
            programs.ruff-check.enable = true;

            settings.formatter.ruff-format.includes = [
              "SConstruct"
              "SCsub"
            ];
            settings.formatter.ruff-check.includes = [
              "SConstruct"
              "SCsub"
            ];
          };

          packages = {
            inherit musicup;
            default = musicup;
          };

          devShells.default = pkgs.mkShell {
            nativeBuildInputs = [
              pkgs.nil
              pkgs.ruff
              pkgs.scons
            ];
          };
        };
    };
}
