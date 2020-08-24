# gtk-layer-shell
Safe wrapper for gtk-layer-shell, generated from .gir file

## Usage
The wrapper works just like described in gtk-layer-shell.h, except that you can use Rust types instead of pointers and such. Unfortunately I am struggling to auto-generate the docs.
Examples can be found in the `examples/` directory. To run an example, execute:

```bash
$ cargo run --example example
```

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
There should not have been any errors, just some warnings about unused stuff.

## Why are you not using Rust 2018?
Gir currently has some issues with Rust 2018, which is why the 2015 version is used to generate the bindings (see this [issue](https://github.com/gtk-rs/gir/issues/746)). No worries, you can still use the generated bindings with Rust 2018.

## TODO
- Auto-generate the documentation

## Contributing
Pull requests are very welcome :)

## License
[MIT](https://choosealicense.com/licenses/mit/)
