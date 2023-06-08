use crate::f_as_str;
use crate::gradient;
use crate::rect;
use crate::text;
use crate::Rect;

fn draw_cell_label(
    label1: &str,
    label2: &str,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    font_size: f64,
    hue: i32,
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

fn draw_cell_body(x: f64, y: f64, w: f64, h: f64, hue: i32) -> String {
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

pub fn draw_cell(name: &str, value: &str, viewport: &Rect, hue: i32) -> String {
    let x = viewport.x;
    let y = viewport.y;
    let w = viewport.w;
    let h = viewport.h;

    let mut svg = String::new();

    let tooltip = format!("Name: {name}<br>Value: {value}");

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
