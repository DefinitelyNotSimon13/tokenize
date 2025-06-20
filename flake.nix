{
  description = "Flake for direnv";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.systems.url = "github:nix-systems/default";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages = {
          default = pkgs.callPackage ./dist/package.nix { };
        };

        devShells.default = pkgs.mkShell {
          # Uncomment relevant sections!

          # Packages e.g. used in cli
          packages = with pkgs; [
            # rustc
            cargo
            bacon
            rust-analyzer
            cargo-deb
            rustfmt
          ];

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;

          # Dependencies used during runtime
          # pkgs of hosts architecture, e.g. added to "$NIX_LD_FLAGS"
          # buildInputs = with pkgs; [];

          # Dependencies used during compiletime
          # pkgs of buildPlatform's architecture, added to "$PATH"
          # nativeBuildInputs = with pkgs; [];

          shellHook = ''
            echo -e "\n\e[1;32mUsing flake development environment!\e[0m\n"
          '';
        };
      }
    );

}
