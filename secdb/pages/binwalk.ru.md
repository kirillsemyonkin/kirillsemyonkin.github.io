[![](https://img.shields.io/badge/GitHub-%23121011?logo=github)](https://github.com/ReFirmLabs/binwalk)
[![](https://img.shields.io/badge/Установить%20на%20Arch%20из%20extra-black?logo=archlinux)](https://archlinux.org/packages/extra/x86_64/binwalk/)
[![](https://img.shields.io/badge/Установить%20через%20Cargo-black?logo=rust)](https://crates.io/crates/binwalk)

Binwalk позволяет вам увидеть скрытые вложенные файлы. Он работает, пытаясь обнаружить вложенные
файлы по их сигнатурам, затем пытаясь разобрать обнаруженные форматы - и так проходя по всему
исходному файлу. Вы можете мыслить о нем как о более мощной версии
[команды `file`](file).

Представьте файл, состоящий из JPEG-файла и GIF-файла.

```bash
binwalk example.jpg.gif
```

В результате будет следующий вывод:

```plain
----------------------------------------------------------------------------
DECIMAL   HEXADECIMAL   DESCRIPTION                                         
----------------------------------------------------------------------------
0         0x0           JPEG image, total size: 25303 bytes                 
25303     0x62D7        GIF image, 400x400 pixels, total size: 1001718 bytes
----------------------------------------------------------------------------
```

Вы можете извлечь вложенные файлы с помощью:

```bash
binwalk -e example.jpg.gif
```

Полученные файлы будут расположены в директории `extractions`:

```plain
------------------------------------------------------------------
[#] Extraction of jpeg data at offset 0x0 declined
[+] Extraction of gif data at offset 0x62D7 completed successfully
------------------------------------------------------------------
```

```tree
extractions
├── example.jpg.gif -> ../example.jpg.gif
└── example.jpg.gif.extracted
    └── 62D7
        └── image.gif
```
