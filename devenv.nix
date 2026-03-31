{ pkgs, lib, ... }:
let
  # TODO: Remove when 117 is released in unstable
  wasm-bindgen-cli = pkgs.buildWasmBindgenCli rec {
    src = pkgs.fetchCrate {
      pname = "wasm-bindgen-cli";
      version = "0.2.117";
      hash = "sha256-vtDQXL8FSgdutqXG7/rBUWgrYCtzdmeVQQkWkjasvZU=";
    };
    cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
      inherit src;
      inherit (src) pname version;
      hash = "sha256-eKe7uwneUYxejSbG/1hKqg6bSmtL0KQ9ojlazeqTi88=";
    };
  };
in
{
  packages = with pkgs; [
    git
    jujutsu

    dioxus-cli
    wasm-bindgen-cli

    twiggy

    tailwindcss_4
  ];

  languages = {
    rust = {
      enable = true;
      channel = "nightly";
      targets = [ "wasm32-unknown-unknown" ];
      components = [
        "rustc"
        "cargo"
        "clippy"
        "rustfmt"
        "rust-analyzer"
        "rust-src"
      ];
      lsp.enable = true;
    };

    c.enable = true;
    cplusplus.enable = true;
  };
}
