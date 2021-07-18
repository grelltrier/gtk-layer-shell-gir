[![Crate](https://img.shields.io/crates/v/gtk-layer-shell.svg)](https://crates.io/crates/gtk-layer-shell)
[![dependency status](https://deps.rs/repo/github/grelltrier/gtk-layer-shell-gir/status.svg)](https://deps.rs/repo/github/grelltrier/gtk-layer-shell-gir)
![Build](https://github.com/grelltrier/gtk-layer-shell-gir/workflows/Build/badge.svg)

# gtk-layer-shell
Safe wrapper for [gtk-layer-shell](https://github.com/wmww/gtk-layer-shell), generated from .gir file

## Usage
The wrapper works just like described in gtk-layer-shell.h, except that you can use Rust types instead of pointers and such. Unfortunately I am struggling to auto-generate the docs.

## Generate the wrapper
Generating the wrapper yourself is not necessary to be able to use it. If you want to do it anyways, just clone the repository and the submodule "gir-files" with
```bash
git clone --recurse-submodules -j8 https://github.com/grelltrier/gtk-layer-shell-gir.git
cd ./gtk-layer-shell-gir
```
If you have a newer .gir file then drop it in ./gir-files and run
```bash
gir
```
After this you can run
```bash
cargo build
```
There should not have been any errors, just some warnings about unused stuff. Currently I am getting the error that the crate ffi can not be found. I manually added the following line to enums.rs and functions.rs.
```bash
use gtk_layer_shell_sys as ffi; // Manual edit
```
This however is overwritten each time gir is ran again so it is a bit hacky. Let me know how I can do this properly

## TODO
- Auto-generate the documentation

## Contributing
Pull requests are very welcome :)

## License
[MIT](https://choosealicense.com/licenses/mit/)
