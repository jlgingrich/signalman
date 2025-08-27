{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    nativeBuildInputs = with pkgs.buildPackages; [
        dbus
        qrencode
        signal-cli
        systemdMinimal
        toybox
        curl
    ];

    # Configure custom prompt
    shellHook = ''
        PROMPT_DIRTRIM=2
        PS1='\[\e[38;5;39m\][nix-shell\[\e[0m\]:\[\e[38;5;51m\]\w\[\e[38;5;39m\]]\[\e[0m\]\$ '
        clear
    '';
}
