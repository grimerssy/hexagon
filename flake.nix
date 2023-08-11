{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, naersk, rust-overlay, flake-utils }:
    let
      forEachSystem = system: rec {
        # ci/cd
        app = naersk'.buildPackage {
          inherit buildInputs nativeBuildInputs;
          src = ./.;
        };
        scripts = {
          test = ''
            nix develop --command cargo test
          '';
          lint = ''
            nix develop --command cargo clippy -- -D warnings && \
            nix develop --command cargo fmt --all --check
          '';
        };
        # dev shell
        dev = {
          packages = with pkgs; [
            rust-toolchain
            cargo-edit
            cargo-nextest
            postgresql
            sqlx-cli
          ];
          scripts = {
            setup = ''
              pg_ctl init -o "-U $PGUSER" -o '--auth=trust'
              echo "port = $PGPORT" >> "$PGDATA/postgresql.conf"
            '';
            up = ''
              pg_ctl start -l "$PGLOG"
            '';
            down = ''
              pg_ctl stop 
            '';
          };
          envVarDefaults = {
            DEVSHELL_DIR = "$PWD/.devshell";
            PGDATA = "$DEVSHELL_DIR/postgresql";
            PGLOG = "$PGDATA/logfile";
            PGPORT = "5432";
            PGUSER = "postgres";
          };
        };
        # dependencies
        buildInputs = with pkgs; [ openssl ];
        nativeBuildInputs = with pkgs;
          [ pkg-config ] ++ lib.optionals stdenv.isDarwin
          (with darwin.apple_sdk.frameworks; [
            CoreServices
            Security
            SystemConfiguration
          ]);
        # boilerplate
        pkgs = import nixpkgs { inherit system overlays; };
        overlays = [
          (import rust-overlay)
          (self: super: {
            rust-toolchain = super.rust-bin.stable.latest.default;
          })
        ];
        packages = { default = app; } // (toBinScripts scripts);
        devShells.default = pkgs.mkShellNoCC {
          inherit buildInputs nativeBuildInputs;
          packages = dev.packages
            ++ builtins.attrValues (toBinScripts dev.scripts);
          shellHook = ''
            ${setEnvVarsIfUnset dev.envVarDefaults}
          '';
        };
        naersk' = pkgs.callPackage naersk rec {
          cargo = rustc;
          rustc = pkgs.rust-toolchain;
        };
        toBinScripts = scripts:
          (builtins.mapAttrs (name: text: (pkgs.writeShellScriptBin name text))
            scripts);
        setEnvVarsIfUnset = set:
          builtins.concatStringsSep "\n" (builtins.attrValues (builtins.mapAttrs
            (name: value: ''export ${name}=''${${name}:="${value}"}'') set));
      };
    in flake-utils.lib.eachDefaultSystem forEachSystem;
}
