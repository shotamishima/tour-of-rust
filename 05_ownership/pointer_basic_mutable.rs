//mutableな参照

// struct Foo {
//     x: i32,
// }

// fn do_something(f: Foo) {
//     println!("{}", f.x);
// }

// fn main() {
//     let mut foo = Foo {x: 42};
//     let f = &mut foo;

//     f.x = 13;

//     println!("{}", foo.x);

//     foo.x = 7;

//     do_something(foo);
// }

fn main() {
    let mut foo = 42;
    let f = &mut foo; // fooのポインタf
    let bar = *f; // ポインタfの値をbar
    *f = 13; // ポインタfの値（bar）を変更する

    println!("{}", bar); //42
    println!("{}", foo); //13
}