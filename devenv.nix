{ pkgs, lib, ... }:
let
  wasm-bindgen-cli = pkgs.buildWasmBindgenCli rec {
    src = pkgs.fetchCrate {
      pname = "wasm-bindgen-cli";
      version = "0.2.120";
      hash = "sha256-Dkkx8Bhfk+y/jEz9Fzwytmv2N3Gj/7ST+5MlPRzzetU=";
    };
    cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
      inherit src;
      inherit (src) pname version;
      hash = "sha256-5Zu/Sh9aBMxB+KGC1MHWJAQ8PuE40M6lsenkpFEwJ6A=";
    };
  };
in
{
  packages = with pkgs; [
    git
    jujutsu

    dioxus-cli
    openssl
    tailwindcss_4
    wasm-bindgen-cli
    twiggy
    binaryen

    oha
    vegeta
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
