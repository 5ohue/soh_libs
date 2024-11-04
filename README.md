# soh_libs

This repo contains the libraries that I use in my rust projects. I'm aiming to
turn this into a self made game engine (and some other utility libs).

It can also be used for web stuff (`soh_rng`, `soh_thread` and `soh_log` are
not game specific).

This repo is WIP.

## Libraries

The libraries in this repo are:

- `soh_math` --- Math library. It contains types to do linear algebra ( `Vec2`,
  `Vec3`, `Mat2`, `Mat3` ).
- `soh_rng` --- Pseudo random number generator classes.
- `soh_log` --- Very easy to use logger.
- `soh_vk` --- vulkan wrapper ( I'm currently learning vulkan so this is where
  I'm working at it ).
- `soh_ui` --- TODO
- `soh_thread` --- thread pool library.

## Usage

Each one of those libraries can be enabled using a feature. Like this:

```toml
soh = { ..., features = ["rng"] }
```

After that you can use the rng library like this:

```rust
let mut rng = soh::rng::Rng32::new();
// ...
```

## Extra features

Enabling some features enables other features in sub libs. For example,
enabling `"log"` feature with `"vk"` will make the `soh_vk` library use the
`soh_log` to log stuff.

Also there's a `"serde"` feature enabling which will derive the `Serialize` and
`Deserialize` traits for types.
