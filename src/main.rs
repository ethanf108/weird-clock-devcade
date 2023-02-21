use chrono::{Local, Timelike};
use core::f64::consts::PI;
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

    let mut hours: Vec<u32> = (1..=12).collect();
    hours.shuffle(&mut thread_rng());

    draw.draw(move |_| {
        let w = window.w();
        let h = window.h();

        let now = Local::now();
        let hour = now.hour() % 12;
        let minute = now.minute();
        let seconds = now.second();

        draw::draw_rect_fill(0, 0, w, h, enums::Color::Cyan);
        draw::set_draw_color(enums::Color::Black);
        draw::set_font(enums::Font::HelveticaBold, (w + h) / 100);

        for i in 0..=11 {
            let theta = i as f64 * PI / 6_f64;
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

        let hour_index = hours.iter().position(|&h| h == hour).unwrap() as i32;
        let next_hour_index = hours.iter().position(|&h| h == (hour + 1) % 12).unwrap() as i32;

        let hour_theta = (((next_hour_index - hour_index + 12) % 12) as f64
            * (minute as f64 / 60_f64)
            + hour_index as f64)
            * PI
            / 6_f64;

        draw::set_line_style(draw::LineStyle::Solid, 10);

        draw::draw_line(
            w / 2,
            h / 2,
            (hour_theta.cos() * 0.6 * (w / 2) as f64) as i32 + w / 2,
            (hour_theta.sin() * 0.6 * (h / 2) as f64) as i32 + h / 2,
        );

        let minute_index = hours.iter().position(|&h| h == (minute / 5) % 12).unwrap() as i32;
        let next_minute_index = hours
            .iter()
            .position(|&h| h == (minute / 5 + 1) % 12)
            .unwrap() as i32;

        let minute_theta = ((((minute % 5) as f64 / 5_f64)
            * ((next_minute_index - minute_index + 12) % 12) as f64)
            + minute_index as f64)
            * PI
            / 6_f64;

        draw::draw_line(
            w / 2,
            h / 2,
            (minute_theta.cos() * 0.8 * (w / 2) as f64) as i32 + w / 2,
            (minute_theta.sin() * 0.8 * (h / 2) as f64) as i32 + h / 2,
        );
    });
    app.run().unwrap();
}
