{
  description = "zipline file uploader";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { nixpkgs, ... }:
  let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f { inherit system; pkgs = nixpkgs.legacyPackages.${system}; });
    in
  {
    devShells = forAllSystems ({pkgs, ...}: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
        ];

        buildInputs = with pkgs; [
          file.dev
        ];

        env.RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
      };
    });
  };
}
