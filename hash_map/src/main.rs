fn main() {
    // hello_hash_map2();
    hello_ownership();
}

fn hello_ownership() {
    use std::collections::HashMap;

    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(name, age); // 所有权被转移给 handsome_boys

    println!("因为过于无耻，{}已经被从帅气男孩名单中除名", handsome_boys["Sunface"]);
    println!("还有，他的真实年龄远远不止{}岁", age);
}

fn hello_hash_map2() {
    use std::collections::HashMap;

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let teams_map: HashMap<_,_> = teams_list.into_iter().collect();

    println!("{:?}",teams_map)
}

fn hello_hash_map1() {
    use std::collections::HashMap;

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let mut teams_map = HashMap::new();
    for team in &teams_list {
        teams_map.insert(&team.0, team.1);
    }

    println!("{:?}",teams_map)
}