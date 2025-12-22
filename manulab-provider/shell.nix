let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-25.11";
  pkgs = import nixpkgs { config = {}; overlays = []; };
  packageList = with pkgs; [
    rustc
    cargo
    pulumi
    protobuf
    neovim
    lazygit
    go-task
    which
    tree
  ];

in

pkgs.mkShellNoCC {
  packages = packageList;
  GREETING = ''
    This environment is set up for developing a Pulumi Provider
    for the HomeLab using Rust.
  '';

  # TODO: fix this in shellHooks as complains about %
  # export PROTOC_INCLUDE=${PROTOC%/bin/protoc}/include

  shellHook = ''
    echo
    echo "$GREETING"
    echo
    echo "Available packages:"
    for pkg in ${pkgs.lib.concatStringsSep " " (map (p: p.pname or p.name) packageList)}; do
      echo "  - $pkg"
    done

    export PROTOC=$(which protoc)

    echo "Current directory structure:"
    tree -L 2
  '';
}

