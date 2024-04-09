use std::cell::RefCell;
use std::process;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

#[cfg(target_family = "wasm")]
pub mod emscripten;

static BLACK: Color = Color::RGB(0, 0, 0);

pub fn main_loop(
    ctx: Rc<RefCell<Sdl>>,
    rect: Rc<RefCell<Rect>>,
    canvas: Rc<RefCell<WindowCanvas>>,
) -> impl FnMut() {
    let mut events = ctx.borrow_mut().event_pump().unwrap();
    let mut dvd_logo = DVDLogo {
        position: Position { x: 0, y: 0 },
        velocity: Velocity { x: -2, y: -2 },
        width: 140,
        height: 100,
        color: Color::RGB(0, 255, 255),
    };

    move || {
        let mut hit_counter = 0;
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    process::exit(1);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    rect.borrow_mut().x -= 10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    rect.borrow_mut().x += 10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    rect.borrow_mut().y -= 10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    rect.borrow_mut().y += 10;
                }
                _ => {}
            }
        }

        // The rest of the game loop goes here...
        if dvd_logo.position.x + dvd_logo.width as i32 >= 800 || dvd_logo.position.x <= 0 {
            dvd_logo.velocity.x = -dvd_logo.velocity.x;
            dvd_logo.color = get_random_color();
            hit_counter += 1;
        }
        if dvd_logo.position.y + dvd_logo.height as i32 >= 600 || dvd_logo.position.y <= 0 {
            dvd_logo.velocity.y = -dvd_logo.velocity.y;
            dvd_logo.color = get_random_color();
            hit_counter += 1;
        }

        if hit_counter == 2 {
            println!("Hit corner");
        }

        // Uncomment to visualize pattern
        //points.push(Point::new(dvd_logo.position.x, dvd_logo.position.y));

        dvd_logo.position.x += dvd_logo.velocity.x;
        dvd_logo.position.y += dvd_logo.velocity.y;

        //texture.set_color_mod(dvd_logo.color.r, dvd_logo.color.g, dvd_logo.color.b);

        rect.borrow_mut().x = dvd_logo.position.x;
        rect.borrow_mut().y = dvd_logo.position.y;
        rect.borrow_mut().w = dvd_logo.width as i32;
        rect.borrow_mut().h = dvd_logo.height as i32;

        let _ = canvas.borrow_mut().set_draw_color(BLACK);
        let _ = canvas.borrow_mut().clear();
        let _ = canvas.borrow_mut().set_draw_color(dvd_logo.color);
        let _ = canvas.borrow_mut().fill_rect(rect.borrow().clone());
        let _ = canvas.borrow_mut().present();
    }
}

use rand::Rng;

struct DVDLogo {
    position: Position,
    velocity: Velocity,
    width: u32,
    height: u32,
    color: Color,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

fn get_random_color() -> Color {
    Color::RGB(
        rand::thread_rng().gen_range(0..255),
        rand::thread_rng().gen_range(0..255),
        rand::thread_rng().gen_range(0..255),
    )
}
