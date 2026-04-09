{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        naersk-lib = pkgs.callPackage naersk {};

        antlrJar = pkgs.fetchurl {
          url = "https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar";
          hash = "sha256-0j17AAb3R3JD0thcVGMrqhkypeBViODCVI2+PdafRjc=";
        };

        antlr4-rust-cli = pkgs.writeShellScriptBin "antlr4rust" ''
          exec ${pkgs.jre}/bin/java -jar ${antlrJar} "$@"
        '';
      in {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [
            jre
          ];
          ANTLR_JAR = antlrJar;
        };
        devShell = with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustc
              rustfmt
              rust-analyzer
              pre-commit
              rustPackages.clippy

              jre
              antlr
              antlr4-rust-cli
            ];

            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            ANTLR_JAR = antlrJar;
          };
      }
    );
}
