{
  description = "comboios-rs: Portuguese train boards (Rust API, MCP server, SvelteKit UI)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "aarch64-darwin"
        "x86_64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages =
            with pkgs;
            [
              # Rust (edition 2024 needs >= 1.85)
              cargo
              rustc
              clippy
              rustfmt
              rust-analyzer

              # UI (comboios-ui uses bun; the Docker image builds on Node 22)
              bun
              nodejs_22

              just
            ]
            # reqwest uses native-tls, which links OpenSSL on Linux
            ++ lib.optionals stdenv.isLinux [
              pkg-config
              openssl
            ];

          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);
    };
}
