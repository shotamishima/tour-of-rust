struct Foo {
    x: i32,
}

fn main() {
    let foo = Foo {x: 42};
    let f = &foo;
    println!("{}", f.x);
}