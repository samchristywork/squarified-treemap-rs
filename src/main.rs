use squarified_treemap_rs::*;

fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    let mut tree = Tree {
        name: "root".to_string(),
        value: 0.,
        children: vec![],
    };

    loop {
        let line = read_line();
        if line == "" {
            break;
        }

        let mut node = Tree {
            name: line,
            value: 0.,
            children: vec![],
        };

        let mut sum = 0.;
        loop {
            let line = read_line();
            if line == "" {
                break;
            }

            let value = line.parse::<f64>().unwrap();

            let faz = Tree {
                name: line,
                value: value,
                children: vec![],
            };
            sum += faz.value;

            node.children.push(faz);
        }
        node.value = sum;

        tree.children.push(node);
    }

    let s = draw_treemap(&tree);
    println!("<!DOCTYPE html>");
    println!("{}", s);
    println!("</html>");
}
