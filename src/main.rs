fn main() {
    vectorexp();
    // tupleexp();
    // define_array();
    // basic_define_vars();
}

fn tupleexp() {
    let t = Bar(234, true, Foo { x: 9 });
    // let t2 = (23, false, Foo { x: 23 });
    println!("{} ,{},{}", t.0, t.1, t.2.x);
}
fn vectorexp() {
    let mut arr: Vec<u8> = Vec::new();
    arr.push(1);
    arr.push(3);
    println!("item:{:?}", arr.get(3));

    let mut ar2: Vec<Foo> = Vec::new();
    ar2.push(Foo { x: 9 });
    print!("item:{}", ar2.get(0).unwrap().x);
}
struct Bar(u8, bool, Foo);
pub fn define_array() {
    // let arr1: [u8; 5] = [1, 2, 23, 4, 1];
    let arr1: [u8; 5] = [0; 5];
    for x in arr1 {
        println!("{}", x);
    }
    let ar2: [Foo; 4] = [Foo { x: 8 }; 4];
    for x in ar2 {
        println!("{}", x.x)
    }
}

#[derive(Clone, Copy, Debug)]
struct Foo {
    x: u8,
}
fn basic_define_vars() {
    let mut x: i16 = 123;
    let mut y = 123.4;
    x += 1;
    y = (x as f64) + y;
    println!("Hello, world! {} {}", x, y);
    strucinfo();
    let coding: bool = true;
    if (coding) || (x < 10) {
        println!("coding");
    } else {
        print!("not coding")
    }
    let mood = if coding { "happy" } else { "sad" };
    println!("{}", mood);
    //string is imutable
    let fullname = "priyanshu parate";
    println!("hello {}", fullname);
    //we can build mutable string like stringbuilder in java from string constructor
    let mut name = String::from("priyanshu");
    name.push_str(" parate");
    println!("hello {}", name);
}

fn strucinfo() {
    let c = Contact {
        fulname: "priyanshu parat".to_string(),
        mobile: "2134239".to_string(),
    };
    let c1 = Contact::from_info("priyanshu".to_string(), "234234".to_string());
    println!("name :{} contact :{}", c.fulname, c.mobile);
    println!("{} {}", c1.info(), c.card());
}

pub struct Contact {
    fulname: String,
    mobile: String,
}
impl Contact {
    pub fn from_info(fulname: String, mobile: String) -> Contact {
        Contact { fulname, mobile }
    }
    fn info(&self) -> String {
        self.fulname.to_string()
    }
}

trait BussinessCard {
    fn card(&self) -> String;
}
impl BussinessCard for Contact {
    fn card(&self) -> String {
        format!("bussness card number: {}", self.fulname)
    }
}
trait HomeAddress {
    fn get_adrees(&self) -> String;
}
impl HomeAddress for Contact {
    fn get_adrees(&self) -> String {
        format!("{}", self.mobile)
    }
}
