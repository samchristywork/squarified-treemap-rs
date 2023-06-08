use crate::render::draw_cell;
use crate::Rect;
use crate::Tree;

fn aspect(w: f64, h: f64) -> f64 {
    if w > h {
        h / w
    } else {
        w / h
    }
}

fn squarified(tree: &Tree, viewport: &Rect, hue: i32) -> String {
    let mut svg = String::new();
    let mut newhue = hue;
    let tree_sum: f64 = tree.children.iter().map(|child| child.value).sum();

    let x = viewport.x;
    let y = viewport.y;
    let w = viewport.w;
    let h = viewport.h;

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

        let mut r1 = Rect {
            x,
            y,
            w: w * best_slice_sum / tree_sum,
            h,
        };
        let r2 = Rect {
            x: x + w * best_slice_sum / tree_sum,
            y,
            w: w - w * best_slice_sum / tree_sum,
            h,
        };

        for node in tree.children[0..best_n].iter() {
            if hue == -1 {
                newhue = rand::random::<i32>() % 360;
            }

            let ratio = node.value / best_slice_sum;
            let r = Rect {
                x: r1.x,
                y: r1.y,
                w: r1.w,
                h: r1.h * ratio,
            };

            if node.value == 0. {
                continue;
            }

            if node.children.len() == 0 {
                svg += &draw_cell(&node.name, &node.value.to_string(), &r, newhue);
                r1.y += r.h;
                continue;
            }

            svg += &squarified(
                &Tree {
                    name: node.name.to_string(),
                    value: node.value,
                    children: node.children.clone(),
                },
                &r,
                newhue,
            );
            r1.y += r.h;
        }

        if best_n < tree.children.len() {
            svg += &squarified(
                &Tree {
                    name: tree.name.to_string(),
                    value: tree.value,
                    children: tree.children[best_n..].to_vec(),
                },
                &r2,
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

        let mut r1 = Rect {
            x,
            y,
            w,
            h: h * best_slice_sum / tree_sum,
        };
        let r2 = Rect {
            x,
            y: y + h * best_slice_sum / tree_sum,
            w,
            h: h - h * best_slice_sum / tree_sum,
        };

        for node in tree.children[0..best_n].iter() {
            if hue == -1 {
                newhue = rand::random::<i32>() % 360;
            }

            let ratio = node.value / best_slice_sum;
            let r = Rect {
                x: r1.x,
                y: r1.y,
                w: r1.w * ratio,
                h: r1.h,
            };

            if node.value == 0. {
                continue;
            }

            if node.children.len() == 0 {
                svg += &draw_cell(&node.name, &node.value.to_string(), &r, newhue);
                r1.x += r.w;
                continue;
            }

            svg += &squarified(
                &Tree {
                    name: node.name.to_string(),
                    value: node.value,
                    children: node.children.clone(),
                },
                &r,
                newhue,
            );
            r1.x += r.w;
        }

        if best_n < tree.children.len() {
            svg += &squarified(
                &Tree {
                    name: tree.name.to_string(),
                    value: tree.value,
                    children: tree.children[best_n..].to_vec(),
                },
                &r2,
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

pub fn squarified_treemap(tree: &Tree, viewport: Rect) -> String {
    let mut svg = String::new();

    let tree = sort_tree(tree);

    svg += &squarified(&tree, &viewport, -1);

    svg
}
