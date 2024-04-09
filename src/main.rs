use std::cell::RefCell;
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use hello_rust_sdl2_wasm::{main_loop, DVDLogo, Velocity};

// Resources
//     https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm
//     https://puddleofcode.com/story/definitive-guide-to-rust-sdl2-and-emscriptem/

// To build locally:
//     cargo run

// To build for the web:
//     rustup target add asmjs-unknown-emscripten
//     export EMCC_CFLAGS="-s USE_SDL=2"
//     cargo build --target asmjs-unknown-emscripten && open index.html
fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window = match video_ctx
        .window("Hello, Rust / SDL2 / WASM!", 800, 600)
        .position_centered()
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err),
    };

    let canvas = match window.into_canvas().present_vsync().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("failed to create canvas: {}", err),
    };

    let ctx = Rc::new(RefCell::new(ctx));
    let canvas = Rc::new(RefCell::new(canvas));
    let dvd_logo = Rc::new(RefCell::new(DVDLogo {
        rect: Rect::new(0, 0, 140, 100),
        velocity: Velocity { x: -2, y: -2 },
        color: Color::RGB(0, 255, 255),
    }));

    #[cfg(target_family = "wasm")]
    use hello_rust_sdl2_wasm::emscripten;

    #[cfg(target_family = "wasm")]
    emscripten::set_main_loop_callback(main_loop(
        Rc::clone(&ctx),
        Rc::clone(&dvd_logo),
        Rc::clone(&canvas),
    ));

    #[cfg(not(target_family = "wasm"))]
    {
        use std::thread::sleep;
        use std::time::Duration;
        loop {
            main_loop(Rc::clone(&ctx), Rc::clone(&dvd_logo), Rc::clone(&canvas))();
            sleep(Duration::from_millis(10))
        }
    }
}
