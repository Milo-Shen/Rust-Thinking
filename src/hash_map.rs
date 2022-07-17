use std::collections::HashMap;

pub fn hash_map() {
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();

    println!("{:?}", teams_map);

    // 所有权转移
    // 1. 若类型实现 Copy 特征，该类型会被复制进 HashMap，因此无所谓所有权
    // 2. 若没实现 Copy 特征，所有权将被转移给 HashMap 中
    // 如果你使用引用类型放入 HashMap 中，请确保该引用的生命周期至少跟 HashMap 活得一样久：

    // 查询 HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);

    // get 方法返回一个 Option<&i32> 类型：当查询不到时，会返回一个 None，查询到时返回 Some(&i32)
    // &i32 是对 HashMap 中值的借用，如果不使用借用，可能会发生所有权的转移

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 更新 HashMap 中的值
    let mut scores = HashMap::new();

    scores.insert("Blue", 10);

    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入
}
