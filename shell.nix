let
  # Rolling updates, not deterministic.
  pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};
in pkgs.mkShell {
    buildInputs = with pkgs; [
        cargo
        rustc
    ];
}
