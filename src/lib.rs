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
#[derive(Debug, Clone)]
pub struct DVDLogo {
    pub rect: Rect,
    pub velocity: Velocity,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

static BLACK: Color = Color::RGB(0, 0, 0);

pub fn main_loop(
    ctx: Rc<RefCell<Sdl>>,
    dvd_logo: Rc<RefCell<DVDLogo>>,
    canvas: Rc<RefCell<WindowCanvas>>,
) -> impl FnMut() {
    let mut events = ctx.borrow_mut().event_pump().unwrap();
    move || {
        let mut dvd_logo = dvd_logo.borrow_mut();
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
                _ => {}
            }
        }

        // The rest of the game loop goes here...
        if dvd_logo.rect.x() + dvd_logo.rect.width() as i32 >= 800 || dvd_logo.rect.x <= 0 {
            dvd_logo.velocity.x *= -1;
            dvd_logo.color = get_random_color();
            hit_counter += 1;
        }
        if dvd_logo.rect.y() + dvd_logo.rect.height() as i32 >= 600 || dvd_logo.rect.y <= 0 {
            dvd_logo.velocity.y *= -1;
            dvd_logo.color = get_random_color();
            hit_counter += 1;
        }

        if hit_counter == 2 {
            println!("Hit corner");
        }

        // Uncomment to visualize pattern
        //points.push(Point::new(dvd_logo.position.x, dvd_logo.position.y));

        //texture.set_color_mod(dvd_logo.color.r, dvd_logo.color.g, dvd_logo.color.b);

        dvd_logo.rect.x += dvd_logo.velocity.x;
        dvd_logo.rect.y += dvd_logo.velocity.y;

        let _ = canvas.borrow_mut().set_draw_color(BLACK);
        let _ = canvas.borrow_mut().clear();
        let _ = canvas.borrow_mut().set_draw_color(dvd_logo.color);
        let _ = canvas.borrow_mut().fill_rect(dvd_logo.rect.clone());
        let _ = canvas.borrow_mut().present();
    }
}

use rand::Rng;

fn get_random_color() -> Color {
    Color::RGB(
        rand::thread_rng().gen_range(0..255),
        rand::thread_rng().gen_range(0..255),
        rand::thread_rng().gen_range(0..255),
    )
}
