use squarified_treemap_rs::*;

fn main() {
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
