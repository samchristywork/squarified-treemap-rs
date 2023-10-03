mod render;
mod treemap;
mod util;

use crate::treemap::squarified_treemap;


#[derive(Clone, Debug)]
pub struct Tree {
    pub name: String,
    pub value: f64,
    pub children: Vec<Tree>,
}

#[derive(Clone, Debug)]
pub struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

pub fn draw_treemap(tree: &Tree) -> String {
    let viewport = Rect {
        x: 0.,
        y: 0.,
        w: 1.,
        h: 0.6,
    };

    let mut svg = String::new();

    svg += f_as_str!(
        "<svg viewBox='{} {} {} {}' xmlns='http://www.w3.org/2000/svg'>",
        viewport.x,
        viewport.y,
        viewport.w,
        viewport.h
    );
    svg += rect!(viewport.x, viewport.y, viewport.w, viewport.h, "pink", "");

    svg += &squarified_treemap(&tree, viewport);

    svg += f_as_str!("</svg>");

    svg
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
            for _ in 0..10 {
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
