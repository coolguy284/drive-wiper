# drive-wiper

`drive-wiper` is a program to fill drives with all zeros, checking to see if chunks are filled with zero first, and if so, skipping them (saves on writes on flash media that don't support the TRIM command like USB flash drives).

# Building

To build, install the Nix package manager, and run the appropriate "build-" script in the scripts folder.

Note: GitHub releases are automatically made for tags after v0.3.0, but for tag v0.3.0 and earlier, the release was manually created by renaming and zipping.

The manual releases were created like so (a slash in the output path means the zip file in the output path contains the file with the path after the slash; the zip file was created using 7-Zip 26.03 (2026-09-03), Ultra, Deflate, no storing of creation or last access time):
x86-linux/drive-wiper -> drive-wiper-linux-x86
x86_64-linux/drive-wiper -> drive-wiper-linux-x86_64
x86-windows/drive-wiper.exe -> drive-wiper-windows-x86.zip/drive-wiper.exe
x86-windows/libmcfgthread-2.dll -> drive-wiper-windows-x86.zip/libmcfgthread-2.dll
x86_64-windows/drive-wiper.exe -> drive-wiper-windows-x86_64.exe

## Installation as a Nix Flake

This GitHub repository is a nix flake. To install the latest release, add the following input to `flake.nix`:

```nix
{
  inputs = {
    ...
    rust-nix-template.url = "github:coolguy284/rust-nix-template/latest-release";
    # optionally add this as well:
    rust-nix-template.inputs.nixpkgs.follows = "nixpkgs";
  };
  
  # This readme assumes that you are passing flake inputs to the Nix system configuration. Example for a NixOS system shown below (adapted from https://drakerossman.com/blog/how-to-convert-default-nixos-to-nixos-with-flakes):
  outputs =
    { self, nixpkgs, ... }:
    
    {
      nixosConfigurations = {
        systemName =
          nixpkgs.lib.nixosSystem {
            system = "x86_64-linux";
            modules = [
              ./configuration.nix
            ];
            specialArgs = {
              # This is the key, pass "inputs" as an extra argument to each Nix module:
              inputs = self.inputs;
            };
          };
      };
    }
}
```

Then, add the input to the system packages in any Nix module's configuration like so:

```nix
{ inputs, pkgs, ... }:

{
  environment.systemPackages = with pkgs; [
    inputs.rust-nix-template.packages.${pkgs.stdenv.hostPlatform.system}
  ];
}
```
