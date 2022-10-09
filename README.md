![](sample.png)

# Simple raytracer in Rust

A basic implementation of [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley in Rust

I wrote it in C++, Rust and Go as a learning exercise and to compare the languages.

## Dependencies

Rust compiler version 1.64 and up

## Compile and Run

```sh
cargo build --release
```

Run

```sh
time ./target/release/raytracer simple > test.ppm
```
