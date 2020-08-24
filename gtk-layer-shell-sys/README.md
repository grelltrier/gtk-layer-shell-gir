# gtk-layer-shell-sys
Unsafe FFI bindings for gtk-layer-shell, generated from .gir file

## Usage
The wrapper works just like described in gtk-layer-shell.h, except that you can use Rust types instead of pointers and such. Unfortunately I am struggling to auto-generate the docs.

## Generate the wrapper
Generating the bindings yourself is not necessary to be able to use it. If you want to do it anyways, just clone the repository and the submodule "gir-files", open up a terminal in it and run
```bash
gir
```
After this you can run
```bash
cargo build
```
There should not have been any errors, just some warnings about unused stuff.

## TODO
- Auto-generate the documentation

## Contributing
Pull requests are very welcome :)

## License
[MIT](https://choosealicense.com/licenses/mit/)
