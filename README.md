# moecs

`moecs` (_micro ECS_) is a small ECS library written in Rust.

Built to be used with lightweight Rust-based game engines, like [ggez](https://github.com/ggez/ggez).

See example implementations [here](./example).

## Features

* Simple user-facing API.
* Lightweight library.
* Basic parallelism via. `rayon`.
* Defined `System` groups to provide control over when systems are run.
