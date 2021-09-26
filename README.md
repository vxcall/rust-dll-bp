# :telescope: rust-dll-bp

This is a boiler plate code that will be generated as a dll binary.
I personally cache this here for me but if you're intend to create game hack, having this is fine to begin with.

You can inject this to running process without any issues.

# toy-arms crate

This crates uses [toy-arms crate](https://github.com/s3pt3mb3r/toy-arms) which eases developping dll.

The variant without toy-arms is on [gist](https://gist.github.com/s3pt3mb3r/c86f87fd0259d175bf6cbdc5b8f49f56).

# windows api

This is used [windows-rs](https://github.com/microsoft/windows-rs) officially maintained by Microsoft as opposed to [winapi](https://github.com/retep998/winapi-rs) which isn't maintained now.

# :warning: important

`.cargo/config` is the [config file](https://doc.rust-lang.org/cargo/reference/config.html) specifying target architecture.
Inside it must be like this:

```
[build]
target = "i686-pc-windows-msvc"
```

Make sure to change target value as which architecture you're targetting to.

| target                 | remarkable                |     |
| ---------------------- | ------------------------- | --- |
| i686-pc-windows-msvc   | 32-bit MSVC (Windows 7+)  |     |
| x86_64-pc-windows-msvc | 64-bit MSVC (Windows 7+)  |     |
| i686-pc-windows-gnu    | 32-bit MinGW (Windows 7+) |     |
| x86_64-pc-windows-gnu  | 64-bit MinGW (Windows 7+) |     |
