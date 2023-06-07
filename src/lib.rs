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

fn aspect(w: f64, h: f64) -> f64 {
    if w > h {
        h / w
    } else {
        w / h
    }
}

fn squarified_treemap(data: Vec<(&str, f64)>, x: f64, y: f64, w: f64, h: f64) -> String {
    let mut svg = String::new();

    let data_sum: f64 = data.iter().map(|(_, value)| value).sum();

    if w > h {
        let mut best_ratio = 0.;
        let mut best_n = 1;
        let mut best_slice_sum = 0.;

        for n in 1..=data.len() {
            let slice_sum: f64 = data[0..n].iter().map(|(_, value)| value).sum();

            let mut average_normalized_ratio = 0.;
            for (name, value) in data[0..n].iter() {
                average_normalized_ratio += aspect(w*slice_sum/data_sum,
                    h*value/slice_sum);
            }
            average_normalized_ratio /= n as f64;

            if average_normalized_ratio > best_ratio {
                best_ratio = average_normalized_ratio;
                best_n = n;
                best_slice_sum = slice_sum;
            }
        }

        let mut r1 = (x, y, w*best_slice_sum/data_sum, h);
        let r2 = (x + w*best_slice_sum/data_sum, y, w - w*best_slice_sum/data_sum, h);

        for (name, value) in data[0..best_n].iter() {
            let ratio = value / best_slice_sum;
            let r = (r1.0, r1.1, r1.2, r1.3 * ratio);

            if value == &0. {
                continue;
            }

            svg += &draw_cell(name, &value.to_string(), r.0, r.1, r.2, r.3, 0);
            r1.1 += r.3;
        }

        if best_n < data.len() {
            svg += &squarified_treemap(data[best_n..].to_vec(), r2.0, r2.1, r2.2, r2.3);
        }
    } else {
        let mut best_ratio = 0.;
        let mut best_n = 1;
        let mut best_slice_sum = 0.;

        for n in 1..=data.len() {
            let slice_sum: f64 = data[0..n].iter().map(|(_, value)| value).sum();

            let mut average_normalized_ratio = 0.;
            for (name, value) in data[0..n].iter() {
                average_normalized_ratio += aspect(w*value/slice_sum,
                    h*slice_sum/data_sum);
            }
            average_normalized_ratio /= n as f64;

            if average_normalized_ratio > best_ratio {
                best_ratio = average_normalized_ratio;
                best_n = n;
                best_slice_sum = slice_sum;
            }
        }

        let mut r1 = (x, y, w, h*best_slice_sum/data_sum);
        let r2 = (x, y + h*best_slice_sum/data_sum, w, h - h*best_slice_sum/data_sum);

        for (name, value) in data[0..best_n].iter() {
            let ratio = value / best_slice_sum;
            let r = (r1.0, r1.1, r1.2 * ratio, r1.3);

            if value == &0. {
                continue;
            }

            svg += &draw_cell(name, &value.to_string(), r.0, r.1, r.2, r.3, 0);
            r1.0 += r.2;
        }

        if best_n < data.len() {
            svg += &squarified_treemap(data[best_n..].to_vec(), r2.0, r2.1, r2.2, r2.3);
        }
    }

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

    let mut data = data;

    svg += &squarified_treemap(data, x, y, w, h);

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

        let mut data = vec![
        ];

        for i in 0..100 {
            data.push(("hi", i as f64));
        }

        draw_treemap(data);
    }
}
