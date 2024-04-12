{ pkgs ? import <nixpkgs> {}, lib ? pkgs.lib }:
let 
  myBuildInputs = with pkgs; [
    vulkan-loader
  ];
in
pkgs.mkShell {
  buildInputs = myBuildInputs;

    shellHook = ''
        export LD_LIBRARY_PATH="${lib.makeLibraryPath myBuildInputs}:$LD_LIBRARY_PATH";
        export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib"
    '';
}
