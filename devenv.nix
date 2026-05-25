{ pkgs, lib, ... }:

let
  bevyLibs = with pkgs; [
    udev
    alsa-lib
    vulkan-loader
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    libxkbcommon
    wayland
  ];
in
{
  languages.rust = {
    enable = true;
    channel = "stable";
    mold.enable = true;
  };

  packages = with pkgs; [
    pkg-config
    clang
  ] ++ bevyLibs;

  env.LD_LIBRARY_PATH = lib.makeLibraryPath bevyLibs;
}
