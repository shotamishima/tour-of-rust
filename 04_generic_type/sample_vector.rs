fn main() {
    // 型指定で生成　Vectorは可変長リスト
    let mut i32_vec = Vec::<i32>::new();
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    // 型推論あり
    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);

    // マクロvec!でも生成可能
    let string_vec = vec![String::from("hello"), String::from("world")];

    for word in string_vec.iter() {
        println!("{}", word);
    }

}