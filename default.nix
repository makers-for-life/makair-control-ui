let
  pkgs = import ./nix/nixpkgs.nix;
  generatedBuild = pkgs.callPackage ./Cargo.nix {
    defaultCrateOverrides = pkgs.defaultCrateOverrides // {
      makair-control = attrs: {
        buildInputs = [pkgs.xorg.libxcb];
        # propagatedBuildInputs = [ pkgs.xorg.libX11];
      };
      expat-sys = attrs: {
        buildInputs = [pkgs.cmake]; 
      };
      freetype = attrs: {
        rename = "servo_freetype";
      };
      servo-fontconfig = attrs: {
        crateName = "servo_fontconfig";
        rename = "servo_fontconfig";
      };
      servo-fontconfig-sys = attrs: {
        crateName = "servo_fontconfig-sys";
        rename = "servo_fontconfig-sys";
        buildInputs = [pkgs.cmake pkgs.pkg-config pkgs.freetype pkgs.expat];
      };
      servo-freetype-sys = attrs: {
        crateName = "servo_freetype-sys";
        rename = "servo_freetype-sys";
        buildInputs = [pkgs.cmake]; 
      };
      wayland-client = attrs: {
        patches = if attrs.version == "0.21.13" then
        [ ./nix/wayland-client-0.12.13-force-native_lib.patch ]
        else
        [];
      };
      wayland-protocols = attrs: {
        patches = if attrs.version == "0.23.6" then
        [ ./nix/wayland-protocols-0.23.6-0001-patch.patch 
        ./nix/wayland-protocols-0.23.6-0002-patch.patch
      ]
      else
      [ ./nix/wayland-protocols-0.21.13-0001-patch.patch ];
    };
  };
};
in generatedBuild.rootCrate.build
