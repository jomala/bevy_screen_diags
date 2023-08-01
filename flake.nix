{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix.url = "github:nix-community/fenix";
  };

  outputs = {self, nixpkgs, flake-utils, fenix, naersk,  ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
    let
      target = "x86_64-unknown-linux-gnu";
      toolchain = with fenix.packages.${system}; combine [
        latest.cargo
        latest.rustc
        targets.${target}.latest.rust-std
      ];
      pkgs = import nixpkgs {
        overlays = [
          (_: super: let pkgs = fenix.inputs.nixpkgs.legacyPackages.${super.system}; in fenix.overlays.default pkgs pkgs)
        ];
        inherit system;
      };
      extraInputs = if target == "x86_64-pc-windows-gnu" then
        with pkgs; [pkgsCross.mingwW64.windows.mingw_w64_pthreads pkgsCross.mingwW64.windows.pthreads]
      else 
        [];
      buildInputs = with pkgs; [
        rust-analyzer-nightly
        cargo-expand
        pkgsCross.mingwW64.buildPackages.gcc
        glibc_multi
        udev alsa-lib vulkan-loader
        xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
        libxkbcommon wayland
        gperftools
      ] ++ extraInputs;
      src = ./.;
      copySources = [
        "graphical_alien_swarm_proc_macros"
        "gas_asset_loader"
      ];
      manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
    in {
      packages.default = self.packages.${system}.buildGame;

      packages.buildGame = (naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        }).buildPackage rec {
        singleStep = true;
        pname = manifest.name;
        version = manifest.version;
        gameName = "${pname}-${version}";
        gitAllRefs = true;

        inherit src copySources buildInputs;
        
        nativeBuildInputs = with pkgs; [
          toolchain
          pkg-config
        ];

        preBuild = ''
          export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-C link-args=''$(echo $NIX_LDFLAGS | tr ' ' '\n' | grep -- '^-L' | tr '\n' ' ')"
          export NIX_LDFLAGS=
        '';

        LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath buildInputs;
        CARGO_BUILD_TARGET = target;
      };

      devShells.${system}.default = pkgs.mkShell {
        gitAllRefs = true;
        inherit src copySources buildInputs;
        nativeBuildInputs = with pkgs; [ toolchain ];

        LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath buildInputs;
        CARGO_BUILD_TARGET = target;
      };
  });
}