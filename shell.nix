{pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/53e81e790209e41f0c1efa9ff26ff2fd7ab35e27.tar.gz") {}}:

with pkgs;
  mkShell rec {
    nativeBuildInputs = [
      pkg-config
    ];
    buildInputs = [
      udev
      alsa-lib
      vulkan-loader
      xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
      libxkbcommon wayland # To use the wayland feature
      cargo
    ];
    LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  }
