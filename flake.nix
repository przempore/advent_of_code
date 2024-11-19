{
  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    # systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs = { nixpkgs.follows = "nixpkgs"; };
  };


  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, devenv, /* systems, */ ... } @ inputs:
    let
      supportedSystems = [ "x86_64-linux" ];
      # forEachSystem = nixpkgs.lib.genAttrs (import systems);
      forEachSystem = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      packages = forEachSystem (system: {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
        devenv-test = self.devShells.${system}.default.config.test;
      });

      # devShells = forEachSystem
      devShells = forEachSystem
        (system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
          in
          {
            default = devenv.lib.mkShell {
              inherit inputs pkgs;
              modules = [
                {
                  # https://devenv.sh/reference/options/
                  packages = [ pkgs.hello ];

                  enterShell = ''
                    hello
                  '';

                  processes.hello.exec = "hello";
                }
              ];
            };

            rust = devenv.lib.mkShell {
              inherit inputs pkgs;
              modules = [
                {
                  # https://devenv.sh/reference/options/
                  packages = [];

                  languages.rust = {
                    enable = true;
                    channel = "nightly";
                  };

                  enterShell = ''
                    cargo version
                  '';

                  # processes.hello.exec = "hello";
                }
              ];
            };

            cpp = devenv.lib.mkShell {
              inherit inputs pkgs;
              modules = [
                {
                  # https://devenv.sh/reference/options/
                  packages = [
                    # pkgs.stdenv.cc.cc.lib
                    pkgs.gcc
                    pkgs.ninja
                    pkgs.clang-tools
                  ];

                  enterShell = ''
                    gcc --version
                    echo "ninja version $(ninja --version)"
                  '';
                }
              ];
            };
          });
    };
}
