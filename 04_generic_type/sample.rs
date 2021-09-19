// itemの型は定義されていない
struct BagOfHolding<T> {
    item: T,
}

fn main() {
    // 型指定した場合
    let i32_bag = BagOfHolding::<i32> { item: 42};
    let bool_bag = BagOfHolding::<bool> { item: true};

    // 型推論した場合
    let float_bag = BagOfHolding { item: 3.14};

    let bag_in_bag = BagOfHolding {
        item: BagOfHolding { item: "boom!"},
    };

    println!("{} {} {} {}", i32_bag.item, bool_bag.item, float_bag.item, bag_in_bag.item.item)
}