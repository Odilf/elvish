{
  description = "TODO";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        formatter = pkgs.nixfmt-rfc-style;

        # checks = {
        #	TODO
        # };

        # packages = TODO
        
        # apps.default = flake-utils.lib.mkApp {
        #   drv = elvish; # TODO (?)
        # };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            pkg-config
            iconv
            cmake
            cups
          ] ++ lib.optionals stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            # pkgs.libiconv
            darwin.apple_sdk.frameworks.SystemConfiguration
            darwin.apple_sdk.frameworks.CoreGraphics
            darwin.apple_sdk.frameworks.AppKit
          ];

          packages = with pkgs; [
            cargo
            rustc
            rust-analyzer
            rustfmt
            bacon
            cargo-expand
          ];
        };
      }
    );
}

