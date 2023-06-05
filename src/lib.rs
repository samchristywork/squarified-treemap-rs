use std::fs::File;
use std::io::prelude::*;

macro_rules! f_as_str {
    ($($arg:tt)*) => {
        format!($($arg)*).as_str()
    };
}

macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr, $fill:expr, $extra:expr) => {
        format!(
            "<rect x='{x}' y='{y}' width='{w}' height='{h}' fill='{fill}' {extra} />",
            x = $x,
            y = $y,
            w = $w,
            h = $h,
            fill = $fill,
            extra = $extra
        )
        .as_str()
    };
}

macro_rules! text {
    ($text:expr, $x:expr, $y:expr, $fill:expr, $extra:expr) => {
        format!(
            "<text x='{x}' y='{y}' fill='{fill}' {extra}>{text}</text>",
            x = $x,
            y = $y,
            fill = $fill,
            extra = $extra,
            text = $text
        )
        .as_str()
    };
}

macro_rules! gradient {
    ($id:expr, $color:expr) => {
        format!(
            "<defs>
           <linearGradient id='{id}' x1='0' x2='1' y1='0' y2='1'>
              <stop offset='0%' stop-color='white'/>
              <stop offset='100%' stop-color='{color}'/>
           </linearGradient>
        </defs>",
            id = $id,
            color = $color
        )
        .as_str()
    };
}

fn draw_cell(name: &str, value: &str, x: f64, y: f64, w: f64, h: f64, hue: u32) -> String {
    let mut svg = String::new();

    svg
}

pub fn draw_treemap() {
    let x = 0.;
    let y = 0.;
    let w = 1.;
    let h = 0.6;

    let mut svg = String::new();

    svg += f_as_str!("<svg viewBox='{x} {y} {w} {h}' xmlns='http://www.w3.org/2000/svg'>");
    svg += rect!(x, y, w, h, "pink", "");

    svg += &draw_cell("build/main", "40185", 0.1, 0.1, 0.2, 0.2, 0);

    svg += f_as_str!("</svg>");

    let mut file = File::create("treemap.svg").unwrap();
    file.write_all(svg.as_bytes()).unwrap();
    file.sync_all().unwrap();
}
