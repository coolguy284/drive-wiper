# https://nixos-and-flakes.thiscute.world/development/intro
{
  description = "Rust Nix compilation test.";
  
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  
  outputs =
    { self, nixpkgs, ... }:
      let
        buildAllPlatformsNativeSystem = "x86_64-linux";
        
        lib = nixpkgs.lib;
        
        # "import nixpkgs { inherit system; }" from https://www.google.com/search?q=nixos+attribute+rustplatform+missing+on+nixpkgs+flake+input
        makePlatformPkgs =
          system:
            (import nixpkgs { system = buildAllPlatformsNativeSystem; }).pkgs;
        
        buildAllPlatformsNativePkgs = makePlatformPkgs buildAllPlatformsNativeSystem;
        
        # Allowed system values from:
        #   https://github.com/NixOS/nixpkgs/issues/55785
        #   https://github.com/NixOS/nix/blob/4ae6e849015fb269414c1544e815aba75085cf7b/release.nix#L4
        #   Allowed values: [ "x86_64-linux" "i686-linux" "x86_64-darwin" "aarch64-linux" ]
        forAllPlatforms =
          makePlatformCfg:
            builtins.listToAttrs
              (
                map
                (
                  platform:
                    {
                      name = platform;
                      value = makePlatformCfg platform;
                    }
                )
                [ "x86_64-linux" "i686-linux" "x86_64-darwin" "aarch64-linux" ]
              );
        
        cargoFileContents = lib.importTOML ./Cargo.toml;
        
        makeRustPackageParams =
          extraConfig: finalAttrs: {
            pname = cargoFileContents.package.name;
            version = cargoFileContents.package.version;
            
            src = ./.;
            
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          } // extraConfig;
        
        customPkgs = {
          # https://www.google.com/search?q=nix+derivation+that+copies+other+derivations+into+itself
          # "toString" from https://www.google.com/search?q=nix+convert+derivation+to+a+store+path+string
          copyJoin =
            { pkgsToCombine, pkgSubdir ? "", copySymlinkContents ? false, ... } @ derivationParams:
              buildAllPlatformsNativePkgs.stdenv.mkDerivation (
                (removeAttrs derivationParams [ "pkgsToCombine" ])
                // {
                  folderNames = builtins.attrNames pkgsToCombine;
                  folderPkgs = map (pkg: toString pkg) (builtins.attrValues pkgsToCombine);
                  inherit pkgSubdir;
                  
                  dontUnpack = true;
                  
                  installPhase = ''
                    # https://stackoverflow.com/questions/9293887/how-to-read-a-space-delimited-string-into-an-array-in-bash/9294015#9294015
                    folderNamesArr=($folderNames)
                    folderPkgsArr=($folderPkgs)
                    
                    mkdir -p $out
                    
                    # https://stackoverflow.com/questions/1445452/shell-script-for-loop-syntax/1445471#1445471
                    # https://stackoverflow.com/questions/1886374/how-to-find-the-length-of-an-array-in-shell/1886483#1886483
                    for i in ''$(seq 0 ''$((''${#folderNamesArr[@]} - 1))); do
                      # "-L" needed to copy the content of any symbolic links that end up in build output
                      # (if that behavior is chosen by argument to this function)
                      cp -r${if copySymlinkContents then " -L" else ""} ''${folderPkgsArr[i]}$pkgSubdir $out/''${folderNamesArr[i]}
                    done
                  '';
                }
              );
        };
        
        outPkgs = {
          # "pkgs.pkgsCross" from https://www.google.com/search?q=rustplatform+buildrustpackage+specify+output+platform
          # "pkgs.pkgsCross.mingwW64" from https://www.google.com/search?q=nixos+pkgscross+rust+x86_64-pc-windows-gnu
          x86_64-linux = buildAllPlatformsNativePkgs.rustPlatform.buildRustPackage (makeRustPackageParams {});
          x86-linux = buildAllPlatformsNativePkgs.pkgsCross.gnu32.rustPlatform.buildRustPackage (makeRustPackageParams {});
          x86_64-windows = buildAllPlatformsNativePkgs.pkgsCross.mingwW64.rustPlatform.buildRustPackage (makeRustPackageParams {});
          x86-windows = buildAllPlatformsNativePkgs.pkgsCross.mingw32.rustPlatform.buildRustPackage (makeRustPackageParams {
            # https://discourse.nixos.org/t/trying-to-cross-compile-to-i686-pc-windows-gnu/76237/18
            env.RUSTFLAGS =
              let
                dylibs = [
                  buildAllPlatformsNativePkgs.pkgsCross.mingw32.windows.pthreads
                  buildAllPlatformsNativePkgs.pkgsCross.mingw32.windows.mcfgthreads
                ];
                searchDirs = map (a: "-L ${a}/lib") dylibs;
              in
              builtins.concatStringsSep
              " "
              (
                [
                  "-C" "link-arg=-lmcfgthread"
                ]
                ++ searchDirs
              );
          });
        };
        
        makeNativeOutPkgs =
          system: params:
            (makePlatformPkgs system)
              .pkgs.rustPlatform.buildRustPackage (makeRustPackageParams params);
      in
        {
          packages = forAllPlatforms (
            buildSystem:
              {
                # "pkgs.symlinkJoin" from https://www.google.com/search?q=nix+combine+multiple+derivations+into+one+big+output
                build-all-platforms =
                  if buildSystem == buildAllPlatformsNativeSystem then
                    customPkgs.copyJoin {
                      name = "build";
                      pkgsToCombine = outPkgs;
                      pkgSubdir = "/bin";
                      copySymlinkContents = true;
                    }
                  else
                    throw "full cross platform build intended for system ${buildAllPlatformsNativeSystem} but this flake output is for system ${buildSystem}";
                
                default = makeNativeOutPkgs buildSystem {};
                
                #test = makeNativeOutPkgs buildSystem {
                #  buildType = "test";
                #};
              }
          );
          
          apps = forAllPlatforms (
            buildSystem:
              {
                default = {
                  type = "app";
                  program = "${makeNativeOutPkgs buildSystem {}}/bin/${cargoFileContents.package.name}";
                };
              }
          );
          
          #check = forAllPlatforms (
          #  buildSystem:
          #    let
          #      pkgs = makePlatformPkgs buildSystem;
          #    in
          #      {
          #        default =
          #          makeNativeOutPkgs buildSystem {
          #            buildType = "test";
          #          };
          #      }
          #);
          
          # https://nixos-and-flakes.thiscute.world/development/intro
          devShells = forAllPlatforms (
            buildSystem:
              let
                pkgs = makePlatformPkgs buildSystem;
              in
                {
                  default =
                    pkgs.mkShell {
                      # The following two methods of getting "rustc" and "cargo" (packages or inputsFrom)
                      # seem roughly equivalent, so packages is chosen as it doesnt depend on choice of
                      # build outputs
                      
                      packages = builtins.attrValues pkgs.rustPlatform.rust;
                      
                      #inputsFrom = [
                      #  outPkgs.x86_64-linux
                      #];
                      
                      shellHook = ''
                        echo "Entering Rust development environment for platform "'"'"${buildSystem}"'"'
                      '';
                    };
                }
          );
        };
}
