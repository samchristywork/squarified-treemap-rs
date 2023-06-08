use rand::prelude::*;
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

#[derive(Clone, Debug)]
pub struct Tree {
    name: String,
    value: f64,
    children: Vec<Tree>,
}

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

fn draw_cell(name: &str, value: &str, x: f64, y: f64, w: f64, h: f64, hue: i32) -> String {
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

fn naive(data: Vec<(&str, f64)>, x: f64, y: f64, w: f64, h: f64) -> String {
    let mut svg = String::new();

    let sum: f64 = data.iter().map(|(_, value)| value).sum();

    let mut x = x;
    for (name, value) in data.iter() {
        let w = w * value / sum;
        let h = h;
        let y = 0.0;

        svg += &draw_cell(name, &value.to_string(), x, y, w, h, 0);

        x += w;
    }

    svg
}

fn aspect(w: f64, h: f64) -> f64 {
    if w > h {
        h / w
    } else {
        w / h
    }
}

fn squarified_treemap(tree: &Tree, x: f64, y: f64, w: f64, h: f64, hue: i32) -> String {
    let mut svg = String::new();

    let mut newhue = hue;

    let tree_sum: f64 = tree.children.iter().map(|child| child.value).sum();

    if w > h {
        let mut best_ratio = 0.;
        let mut best_n = 1;
        let mut best_slice_sum = 0.;

        for n in 1..=tree.children.len() {
            let slice_sum: f64 = tree.children[0..n].iter().map(|child| child.value).sum();

            let mut average_normalized_ratio = 0.;
            for node in tree.children[0..n].iter() {
                average_normalized_ratio +=
                    aspect(w * slice_sum / tree_sum, h * node.value / slice_sum);
            }
            average_normalized_ratio /= n as f64;

            if average_normalized_ratio > best_ratio {
                best_ratio = average_normalized_ratio;
                best_n = n;
                best_slice_sum = slice_sum;
            }
        }

        let mut r1 = (x, y, w * best_slice_sum / tree_sum, h);
        let r2 = (
            x + w * best_slice_sum / tree_sum,
            y,
            w - w * best_slice_sum / tree_sum,
            h,
        );

        for node in tree.children[0..best_n].iter() {
            if hue == -1 {
                newhue = rand::random::<i32>() % 360;
            }

            let ratio = node.value / best_slice_sum;
            let r = (r1.0, r1.1, r1.2, r1.3 * ratio);

            if node.value == 0. {
                continue;
            }

            if node.children.len() == 0 {
                svg += &draw_cell(
                    &node.name,
                    &node.value.to_string(),
                    r.0,
                    r.1,
                    r.2,
                    r.3,
                    newhue,
                );
                r1.1 += r.3;
                continue;
            }

            svg += &squarified_treemap(
                &Tree {
                    name: node.name.to_string(),
                    value: node.value,
                    children: node.children.clone(),
                },
                r.0,
                r.1,
                r.2,
                r.3,
                newhue,
            );
            r1.1 += r.3;
        }

        if best_n < tree.children.len() {
            svg += &squarified_treemap(
                &Tree {
                    name: tree.name.to_string(),
                    value: tree.value,
                    children: tree.children[best_n..].to_vec(),
                },
                r2.0,
                r2.1,
                r2.2,
                r2.3,
                hue,
            );
        }
    } else {
        let mut best_ratio = 0.;
        let mut best_n = 1;
        let mut best_slice_sum = 0.;

        for n in 1..=tree.children.len() {
            let slice_sum: f64 = tree.children[0..n].iter().map(|child| child.value).sum();

            let mut average_normalized_ratio = 0.;
            for node in tree.children[0..n].iter() {
                average_normalized_ratio +=
                    aspect(w * node.value / slice_sum, h * slice_sum / tree_sum);
            }
            average_normalized_ratio /= n as f64;

            if average_normalized_ratio > best_ratio {
                best_ratio = average_normalized_ratio;
                best_n = n;
                best_slice_sum = slice_sum;
            }
        }

        let mut r1 = (x, y, w, h * best_slice_sum / tree_sum);
        let r2 = (
            x,
            y + h * best_slice_sum / tree_sum,
            w,
            h - h * best_slice_sum / tree_sum,
        );

        for node in tree.children[0..best_n].iter() {
            if hue == -1 {
                newhue = rand::random::<i32>() % 360;
            }

            let ratio = node.value / best_slice_sum;
            let r = (r1.0, r1.1, r1.2 * ratio, r1.3);

            if node.value == 0. {
                continue;
            }

            if node.children.len() == 0 {
                svg += &draw_cell(
                    &node.name,
                    &node.value.to_string(),
                    r.0,
                    r.1,
                    r.2,
                    r.3,
                    newhue,
                );
                r1.0 += r.2;
                continue;
            }

            svg += &squarified_treemap(
                &Tree {
                    name: node.name.to_string(),
                    value: node.value,
                    children: node.children.clone(),
                },
                r.0,
                r.1,
                r.2,
                r.3,
                newhue,
            );
            r1.0 += r.2;
        }

        if best_n < tree.children.len() {
            svg += &squarified_treemap(
                &Tree {
                    name: tree.name.to_string(),
                    value: tree.value,
                    children: tree.children[best_n..].to_vec(),
                },
                r2.0,
                r2.1,
                r2.2,
                r2.3,
                hue,
            );
        }
    }

    svg
}

fn sort_tree(tree: &Tree) -> Tree {
    let mut tree = tree.clone();

    tree.children
        .sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap());

    for child in tree.children.iter_mut() {
        *child = sort_tree(child);
    }

    tree
}

fn squarified_treemap_wrapper(tree: &Tree, x: f64, y: f64, w: f64, h: f64) -> String {
    let mut svg = String::new();

    svg += &squarified_treemap(tree, x, y, w, h, -1);

    svg
}

pub fn draw_treemap(tree: &Tree) {
    let x = 0.;
    let y = 0.;
    let w = 1.;
    let h = 0.6;

    let mut svg = String::new();

    svg += f_as_str!("<svg viewBox='{x} {y} {w} {h}' xmlns='http://www.w3.org/2000/svg'>");
    svg += rect!(x, y, w, h, "pink", "");

    let tree = sort_tree(tree);

    svg += &squarified_treemap_wrapper(&tree, x, y, w, h);

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
        let mut tree = Tree {
            name: "root".to_string(),
            value: 0.,
            children: vec![],
        };

        for _ in 0..10 {
            let mut node = Tree {
                name: "foo".to_string(),
                value: 0.,
                children: vec![],
            };

            let mut sum = 0.;
            for i in 0..10 {
                let value = rand::random::<u32>() % 100 + 10;

                let faz = Tree {
                    name: "bar".to_string(),
                    value: value as f64,
                    children: vec![],
                };
                sum += faz.value;

                node.children.push(faz);
            }
            node.value = sum;

            tree.children.push(node);
        }

        println!("{:#?}", tree);

        draw_treemap(&tree);
    }
}
