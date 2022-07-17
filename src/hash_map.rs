use ahash::RandomState;
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

    let text = "hello world wonderful world";

    // 在已有值的基础上更新
    let mut map = HashMap::new();
    // 根据空格来切分字符串(英文单词都是通过空格切分)
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    // 上面代码中，新建一个 map 用于保存词语出现的次数，插入一个词语时会进行判断：若之前没有插入过，则使用该词语作 Key，插入次数 0 作为 Value，若之前插入过则取出之前统计的该词语出现的次数，对其加一。
    // 有两点值得注意：
    // 1. or_insert 返回了 &mut v 引用，因此可以通过该可变引用直接修改 map 中对应的值
    // 2. 使用 count 引用时，需要先进行解引用 *count，否则会出现类型不匹配

    // 哈希函数
    // 因此，一个类型能否作为 Key 的关键就是是否能进行相等比较，或者说该类型是否实现了 std::cmp::Eq 特征。
    // f32 和 f64 浮点数，没有实现 std::cmp::Eq 特征，因此不可以用作 HashMap 的 Key

    // 高性能三方库
    // 因此若性能测试显示当前标准库默认的哈希函数不能满足你的性能需求，就需要去 crates.io 上寻找其它的哈希函数实现，使用方法很简单：
    let mut map: HashMap<i32, i32, RandomState> = HashMap::default();
    map.insert(12, 34);
    println!("{:?}", map);

    // 目前，HashMap 使用的哈希函数是 SipHash，它的性能不是很高，但是安全性很高。SipHash 在中等大小的 Key 上，性能相当不错，但是对于小型的 Key （例如整数）或者大型 Key （例如字符串）来说，性能还是不够好。
    // 若你需要极致性能，例如实现算法，可以考虑这个库：ahash
}
