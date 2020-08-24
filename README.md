# gtk-layer-shell-gir
Safe wrapper for gtk-layer-shell, generated from .gir file

## Usage
The wrapper works just like described in gtk-layer-shell.h, except that you can use Rust types instead of pointers and such. Unfortunately I am struggling to auto-generate the docs.

## Generate the wrapper
Generating the wrapper yourself is not necessary to be able to use it. If you want to do it anyways, just clone the repository and the submodule "gir-files", open una terminal in it and r
```bash
gir
```
unfortunately you need to change the functions.rs file in the /src/auto folder. Please change
```rust
use Edge;
use Layer;
```
to
```rust
use crate::Edge;
use crate::Layer;
```
I am trying to solve this issue. If you have an idea, a PR is always welcome :). After this you can run
```bash
cargo build
```
There should not have been any errors, just some warnings about unused stuff.

## TODO
- Get rid of the manual changes of functions.rs
- Auto-generate the documentation

## Contributing
Pull requests are very welcome :)

## License
[MIT](https://choosealicense.com/licenses/mit/)
