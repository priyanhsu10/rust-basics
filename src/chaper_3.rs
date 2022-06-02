pub fn exp() {
    variableshadow();
    tupleexp();
    print_array();
}

pub fn variableshadow() {
    let x = 10;
    println!("value of x :{}", x);
    let x = "priyanshu";
    println!("value of x after shadowing: {}", x)
}

pub fn scaller_datatypes() {
    //Integers
    let f: u8 = 255;
    //Floating point number
    //Boolean
    //Character
}

fn tupleexp() {
    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("one element tuple: {:?}", (5,));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{}, {},{},{}", a, b, c, d);
}

fn print_array() {
    let a = [1, 3, 4, 3, 5, 6];
    for e in a {
        println!("{}", e);
    }
    for numer in 0..a.len() {
        println!("{}", a[numer])
    }
}
