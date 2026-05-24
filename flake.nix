{
  description = "Mnau - 2D arcade game: collect fish, avoid cars!";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default;

        nativeBuildDeps = with pkgs; [
          rustToolchain
          pkg-config
          makeWrapper
        ];

        buildDeps = with pkgs; [
          libGL
          libxkbcommon
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          xorg.libXinerama
          alsa-lib
        ];

        runtimeDeps = buildDeps;

        mnau = pkgs.rustPlatform.buildRustPackage {
          pname = "Mnau";
          version = "0.1.0";

          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = nativeBuildDeps;
          buildInputs = buildDeps;

          postInstall = ''
            mkdir -p $out/share/mnau/res
            cp -r res/* $out/share/mnau/res/
            wrapProgram $out/bin/Mnau \
              --set LD_LIBRARY_PATH ${pkgs.lib.makeLibraryPath runtimeDeps}
          '';

          meta = with pkgs.lib; {
            description = "2D arcade game - collect fish, avoid cars!";
            homepage = "https://github.com/your-org/Mnau";
            license = licenses.mit;
            platforms = platforms.linux;
          };
        };

      in {
        packages = {
          default = mnau;
          mnau = mnau;
        };

        apps.default = {
          type = "app";
          program = "${mnau}/bin/Mnau";
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = nativeBuildDeps;
          buildInputs = buildDeps;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeDeps;

          shellHook = ''
            echo "Mnau dev shell"
            echo "Run 'cargo run' to start the game"
          '';
        };
      }
    );
}