# Ray tracer
A rust attempt at a ray tracer that runs in your browser (compiles to [wasm](https://webassembly.org/)).
You can check the live demo at this [page](https://kikos.tech/ray-tracer/).

## Build
For building you just need [rust](https://rustup.rs/) and run: 
1. cargo build
2. cargo run -- -f png -o img.png --width 1080


## Example
### Diffuse render:
![Examples](/images/world.png "Diffuse sphere")
### Normals render:
![Examples](/images/sphere_normal.png "Normal anti aliased shaded sphere")
