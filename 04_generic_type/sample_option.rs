struct BagOfHolding<T> {
    item: Option<T>,
}

fn main() {
    let i32_bag = BagOfHolding::<i32> { item: None};

    if i32_bag.item.is_none() {
        println!("Nothing in a bag.")
    } else {
        println!("Something in a bag")
    }

    let i32_bag = BagOfHolding::<i32> { item: Some(42)};

    if i32_bag.item.is_some() {
        println!("Something in a bag")
    } else {
        println!("Nothing in a bag")
    }

    // 別の書き方
    match i32_bag.item {
        Some(v) => println!("{} was found in a bag", v),
        None => println!("Nothin was found."),
    }
}