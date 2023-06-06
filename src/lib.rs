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

fn draw_cell_label(
    label1: &str,
    label2: &str,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    font_size: f64,
    hue: u32,
) -> String {
    let mut svg = String::new();

    let y = y + 0.5 * font_size / 2.;

    let can_show_two = h > font_size * 2.;
    let can_show_one = h > font_size;
    let can_show_label1 = w > font_size * label1.len() as f64 / 2.;
    let can_show_label2 = w > font_size * label2.len() as f64 / 2.;

    if can_show_two && can_show_label1 && can_show_label2 {
        svg += text!(
            label1,
            x + w / 2.,
            y + h / 2. - font_size / 2.,
            format!("hsl({hue}, 50%, 20%)"),
            format!("font-size='{font_size}' text-anchor='middle'")
        );

        svg += text!(
            label2,
            x + w / 2.,
            y + h / 2. + font_size / 2.,
            format!("hsl({hue}, 50%, 20%)"),
            format!("font-size='{font_size}' text-anchor='middle'")
        );

        return svg;
    }

    if can_show_one && can_show_label1 {
        svg += text!(
            label1,
            x + w / 2.,
            y + h / 2.,
            format!("hsl({hue}, 50%, 20%)"),
            format!("font-size='{font_size}' text-anchor='middle'")
        );

        return svg;
    }

    if can_show_one && can_show_label2 {
        svg += text!(
            label2,
            x + w / 2.,
            y + h / 2.,
            format!("hsl({hue}, 50%, 20%)"),
            format!("font-size='{font_size}' text-anchor='middle'")
        );

        return svg;
    }

    svg
}

fn draw_cell_body(x: f64, y: f64, w: f64, h: f64, hue: u32) -> String {
    let mut svg = String::new();

    svg += rect!(x, y, w, h, format!("url(#Gradient{hue})"), "class='solid'");
    svg += rect!(
        x,
        y,
        w,
        h,
        "none",
        format!("stroke='hsl({hue}, 50%, 20%)' stroke-width='.001'")
    );

    svg
}

fn draw_cell(name: &str, value: &str, x: f64, y: f64, w: f64, h: f64, hue: u32) -> String {
    let mut svg = String::new();

    let tooltip = "Name: build/main<br>Value: 40184";

    svg += gradient!(format!("Gradient{hue}"), format!("hsl({hue}, 50%, 70%)"));
    svg += f_as_str!(
        "<g class='hover-element' data-tooltip='{tooltip}'
        onclick='console.log(\"{tooltip}\")'>"
    );

    svg += &draw_cell_body(x, y, w, h, hue);
    svg += &draw_cell_label(name, value, x, y, w, h, 0.03, hue);
    svg += f_as_str!("</g>");

    svg
}

pub fn draw_treemap(data: Vec<(&str, f64)>) {
    let x = 0.;
    let y = 0.;
    let w = 1.;
    let h = 0.6;

    let mut svg = String::new();

    svg += f_as_str!("<svg viewBox='{x} {y} {w} {h}' xmlns='http://www.w3.org/2000/svg'>");
    svg += rect!(x, y, w, h, "pink", "");

    let sum: f64 = data.iter().map(|(_, value)| value).sum();

    let mut x = x;
    for (name, value) in data.iter() {
        let w = w * value / sum;
        let h = h;
        let y = 0.0;

        svg += &draw_cell(name, &value.to_string(), x, y, w, h, 0);

        x += w;
    }

    svg += f_as_str!("</svg>");

    let mut file = File::create("treemap.svg").unwrap();
    file.write_all(svg.as_bytes()).unwrap();
    file.sync_all().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todo() {
        let data = vec![
            ("foo", 6.),
            ("bar", 6.),
            ("baz", 4.),
            ("qux", 3.),
            ("quux", 2.),
            ("quuz", 2.),
            ("corge", 1.),
        ];
        draw_treemap(data);
    }
}
