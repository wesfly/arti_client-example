{
  description = "Nix Flake for PDF Remote Access Trojan";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.hello = nixpkgs.legacyPackages.x86_64-linux.hello;
    packages.x86_64-linux.default = self.packages.x86_64-linux.hello;

    devShells.x86_64-linux.default = nixpkgs.legacyPackages.x86_64-linux.mkShell {
      buildInputs = [
        nixpkgs.legacyPackages.x86_64-linux.pkg-config
        nixpkgs.legacyPackages.x86_64-linux.openssl
        nixpkgs.legacyPackages.x86_64-linux.rustc
        nixpkgs.legacyPackages.x86_64-linux.cargo
        nixpkgs.legacyPackages.x86_64-linux.rustfmt
        nixpkgs.legacyPackages.x86_64-linux.sqlite
        nixpkgs.legacyPackages.x86_64-linux.clippy
      ];
    };
  };
}
