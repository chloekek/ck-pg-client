let
    tarball = fetchTarball (import ./pinned.nix);
    overlays = [
        (import ../nixpkgs-mozilla/overlay.nix)
        (import ../rust/overlay.nix)
    ];
in
    import tarball { inherit overlays; }
