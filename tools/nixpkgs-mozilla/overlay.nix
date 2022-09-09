let
    tarball = fetchTarball (import ./pinned.nix);
in
    import tarball
