# :telescope: rust-dll-bp

This is a boiler plate code that will be generated as a dll binary.
I personally cache this here for me but if you're intend to create game hack, having this is great to begin with.

You can inject this to running process without any issues.

# toy-arms crate

This crate consists of winapi crate only.
However, there is an example which uses [toy-arms crate](https://github.com/s3pt3mb3r/toy-arms) that eases developing dll in `examples` directory.

The variant without toy-arms is on [gist](https://gist.github.com/s3pt3mb3r/c86f87fd0259d175bf6cbdc5b8f49f56).

# :warning: important

`.cargo/config.toml` is the [config file](https://doc.rust-lang.org/cargo/reference/config.html) specifying target architecture.
Inside it must be like this:

```
[build]
target = "i686-pc-windows-msvc"
```

Make sure to change target value as which architecture you're targeting to.

| target                 | remarkable                |
| ---------------------- | ------------------------- |
| i686-pc-windows-msvc   | 32-bit MSVC (Windows 7+)  |
| x86_64-pc-windows-msvc | 64-bit MSVC (Windows 7+)  |
| i686-pc-windows-gnu    | 32-bit MinGW (Windows 7+) |
| x86_64-pc-windows-gnu  | 64-bit MinGW (Windows 7+) |
