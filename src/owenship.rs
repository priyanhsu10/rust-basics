pub fn exp() {
    let mut foo = Foo { x: 9 };
    let foo_b = &mut foo;
    do_stuff(foo_b);
    do_stuff(foo_b);
    print!("{}", &foo.x)
}

fn do_stuff(foo: &mut Foo) {
    foo.x = foo.x + 1;
    println!("{}", foo.x)
}
struct Foo {
    x: u8,
}
