let
    nixpkgs = import tools/nixpkgs;
in
    nixpkgs.mkShell {
        nativeBuildInputs = [
            nixpkgs.postgresql_14
            nixpkgs.rustChannel.rust
            nixpkgs.util-linux
        ];
    }
