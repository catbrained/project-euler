{
  description = "My solutions for Project Euler";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, rust-overlay, flake-utils, advisory-db, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = craneLib.cleanCargoSource ./.;

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = [
            # Add additional build inputs here
          ];

          # Additional environment variables can be set directly
          # MY_CUSTOM_VAR = "some value";
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        individualCrateArgs = commonArgs // {
          inherit cargoArtifacts;
          inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
          doCheck = false;
        };

        fileSetForCrate = crate: lib.fileset.toSource {
          root = ./.;
          fileset = lib.fileset.unions [
            ./Cargo.toml
            ./Cargo.lock
            (craneLib.fileset.commonCargoSources crate)
          ];
        };

        # Build the individual workspace crates.
        project-euler = craneLib.buildPackage (individualCrateArgs // {
          src = fileSetForCrate ./crates/project-euler;
        });
        project-euler-macros = craneLib.buildPackage (individualCrateArgs // {
          src = fileSetForCrate ./crates/project-euler-macros;
        });
        project-euler-util = craneLib.buildPackage (individualCrateArgs // {
          src = fileSetForCrate ./crates/project-euler-util;
          nativeBuildInputs = [
            pkgs.gnuplot # for Criterion
          ];
        });
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit project-euler project-euler-macros project-euler-util;

          # Run clippy (and deny all warnings) on the crate source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          project-euler-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          project-euler-doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          # Check formatting
          project-euler-fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Audit dependencies
          project-euler-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Audit licenses
          project-euler-deny = craneLib.cargoDeny {
            inherit src;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `project-euler` if you do not want
          # the tests to run twice
          project-euler-nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
            cargoNextestPartitionsExtraArgs = "--no-tests=pass";
          });

          # Code coverage with cargo-tarpaulin
          project-euler-tarpaulin = craneLib.cargoTarpaulin (commonArgs // {
            inherit cargoArtifacts;
          });
        };

        packages = {
          inherit project-euler project-euler-macros project-euler-util;
        };

        apps = {
          project-euler = flake-utils.lib.mkApp {
            drv = project-euler;
          };
          project-euler-macros = flake-utils.lib.mkApp {
            drv = project-euler-macros;
          };
          project-euler-util = flake-utils.lib.mkApp {
            drv = project-euler-util;
          };
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = [
            pkgs.cargo-expand
            pkgs.cargo-mutants
          ];
        };
      });
}
