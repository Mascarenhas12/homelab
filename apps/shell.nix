let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-25.11";
  pkgs = import nixpkgs { config = {}; overlays = []; };
  packageList = with pkgs; [
    elixir_1_19
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
    This environment is set up for developing a CLI
    for the HomeLab using Rust.
  '';


  shellHook = ''
    echo
    echo "$GREETING"
    echo
    echo "Available packages:"
    for pkg in ${pkgs.lib.concatStringsSep " " (map (p: p.pname or p.name) packageList)}; do
      echo "  - $pkg"
    done

    mix archive.install hex phx_new

    echo "Current directory structure:"
    tree -L 2
  '';
}

