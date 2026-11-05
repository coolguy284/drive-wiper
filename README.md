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
