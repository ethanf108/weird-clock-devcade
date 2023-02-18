use fltk::{prelude::*, *};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let app = app::App::default();
    let mut window = window::Window::default();
    window.fullscreen(true);
    let mut draw = widget::Widget::default();
    window.end();
    window.show();

    let mut hours: Vec<u8> = (1..=12).collect();
    hours.shuffle(&mut thread_rng());

    draw.draw(move |_| {
        let w = window.w();
        let h = window.h();
        draw::draw_rect_fill(0, 0, w, h, enums::Color::Cyan);
        draw::set_draw_color(enums::Color::Black);
        draw::set_font(enums::Font::HelveticaBold, (w + h) / 100);
        for i in 0..=11 {
            let theta = i as f64 * core::f64::consts::PI / 6_f64;
            let num = hours[i as usize];
            draw::draw_text2(
                format!("{}", num).as_str(),
                (theta.cos() * 0.9 * (w / 2) as f64) as i32 + w / 2,
                (theta.sin() * 0.9 * (h / 2) as f64) as i32 + h / 2,
                0,
                0,
                enums::Align::Center,
            );
        }
    });
    app.run().unwrap();
}
