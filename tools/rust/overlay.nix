self: super:

{
    # rustChannelOf is supplied by nixpkgs-mozilla.
    rustChannel = super.rustChannelOf (import ./pinned.nix);
}
