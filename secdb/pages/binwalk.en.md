[![](https://img.shields.io/badge/GitHub-%23121011?logo=github)](https://github.com/ReFirmLabs/binwalk)
[![](https://img.shields.io/badge/Install%20on%20Arch%20via%20extra-black?logo=archlinux)](https://archlinux.org/packages/extra/x86_64/binwalk/)
[![](https://img.shields.io/badge/Install%20with%20Cargo-black?logo=rust)](https://crates.io/crates/binwalk)

Binwalk allows you to see the hidden embedded files in a file. It works by trying to detect files
using their signatures and then trying to parse the detected formats at every position in a file.

Imagine a file that is comprised of a JPEG file and a GIF file.

```bash
binwalk example.jpg.gif
```

The resulting output will be the following:

```plain
----------------------------------------------------------------------------
DECIMAL   HEXADECIMAL   DESCRIPTION                                         
----------------------------------------------------------------------------
0         0x0           JPEG image, total size: 25303 bytes                 
25303     0x62D7        GIF image, 400x400 pixels, total size: 1001718 bytes
----------------------------------------------------------------------------
```

You can extract the embedded files with:

```bash
binwalk -e example.jpg.gif
```

The files will be put into the `extractions` directory:

```plain
------------------------------------------------------------------
[#] Extraction of jpeg data at offset 0x0 declined
[+] Extraction of gif data at offset 0x62D7 completed successfully
------------------------------------------------------------------

extractions
├── example.jpg.gif (symlink)
└── example.jpg.gif.extracted
    └── 62D7
        └── image.gif
```
