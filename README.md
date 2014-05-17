rust-pong
=========

A pong clone in Rust using the Piston game engine

| Dependency | Online Docs |
|---------|------|------------|
| [piston](https://github.com/bvssvni/piston) | [piston docs](http://bvssvni.github.io/docs/piston/piston/) |
| [rust-graphics](https://github.com/bvssvni/rust-graphics) | [rust-graphics docs](http://bvssvni.github.io/docs/rust-graphics/graphics/) |
[rust-sdl2](https://github.com/AngryLawyer/rust-sdl2) | [rust-sdl2 docs](http://bvssvni.github.io/docs/rust-sdl2/sdl2/) |
| [rust-opengles](https://github.com/mozilla-servo/rust-opengles) | [rust-opengles docs](http://bvssvni.github.io/docs/rust-opengles/opengles/) |


How to play
-----------
Use `Space` to emit the ball.
Use `Up` and `Down` to control player.

![alt tag](./rust-pong.png)

## Building Instructions

Add the dependencies and add the '.rlib' files to '/target/cpu-vendor-os/lib/':

In the Terminal window, navigate to the project folder and type:

```
make run
```

