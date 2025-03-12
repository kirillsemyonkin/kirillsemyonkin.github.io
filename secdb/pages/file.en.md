The `file` command is a tool made to determine the kind of file a given file is. It is a built-in
command in many Unix-like OSes. It works by reading the first few bytes of a file (magic number) and
then matching it against a database of known file signatures.

```bash
$ file example.jpg
example.jpg: JPEG image data, JFIF standard 1.01, ...
```
