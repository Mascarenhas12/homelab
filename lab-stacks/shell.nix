let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-25.11";
  pkgs = import nixpkgs { config = {}; overlays = []; };
  packageList = with pkgs; [
    pulumi
    pulumiPackages.pulumi-python
    neovim
    lazygit
    go-task
    tree
    uv
  ];

in

pkgs.mkShellNoCC {
  packages = packageList;
  GREETING = ''
    This environment is set up for developing a Pulumi Stacks
    for the HomeLab using Python.
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
    
    echo "Installing latest python..."
    uv python install
    uv add mypy
    pulumi install

    echo "Current directory structure:"
    tree -L 2
  '';
}

