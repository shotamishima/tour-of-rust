struct Foo {
    x: i32,
}

// move ownership
fn do_something(f: Foo) {
    println!("{}", f.x)
}

fn do_something2() -> Foo {
    Foo { x: 42 }
}

fn main() {
    let foo_a = Foo { x: 42};
    // 構造体をインスタンス化して、変数に束縛、メモリリソースを作成する
    // fooは所有者
    let foo_b = Foo { x: 13};

    println!("{}", foo_a.x);
    println!("{}", foo_b.x);
    // スコープの終わりでメモリ開放＝drop

    let foo = Foo { x: 42 };
    // fooの所有権はdo_somethingに移動する
    do_something(foo);

    // do_something2の中の変数の所有権はfoo2に移る 
    let foo2 = do_something2();
    
}