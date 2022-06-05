#![allow(unused)]
mod arrayexp;
mod chaper_3;
mod chaptor_2;
mod dynamic;
mod owenship;
mod smart_poiters;
mod threadexp;

use arrayexp::define_array;
use std::cmp::Ord;
use std::cmp::Ordering;
fn main() {
    // vectorexp();
    // tupleexp();
    // define_array();
    // basic_define_vars();
    // let act: Activity = Activity::Sleeping(5);
    // println!("{}", enumexp(act).to_string());

    // generics_exp();

    // let f: Option<Foo> = Some(Foo { x: 9 });
    // if f.is_some() {
    //     println!("not NOne")
    // }
    // println!("foo: {:?}", f);
    // // print!("{:?}", exeptionexp(10.0, 4.0));
    // match exeptionexp(10., 2.) {
    //     Ok(r) => println!("result {}", r),
    //     Err(s) => println!("error {}", s),
    // }
    // owenship::exp();
    // threadexp::exp();
    // chaptor_2::guessing_game();
    // chaper_3::exp();
    // dynamic::exp();
    smart_poiters::exp();
}
fn exeptionexp(num: f32, by: f32) -> Result<i32, String> {
    if by == 0. {
        Err("cannot divede by zero".to_string())
    } else {
        Ok((num / by).floor() as i32)
    }
}

fn generics_exp() {
    let c1 = Container { item: 10 };
    let c2 = Container {
        item: "Hello".to_string(),
    };
    println!("{}", c1.item);
    println!("{}", c2.item);
    println!("{:?}", c1.compare_item(&1));
    println!("{:?}", c2.compare_item(&"Hello".to_string()));
    println!("{:?}", compare(&10, &120));
}

struct Container<T> {
    item: T,
}
impl<T: Ord> Container<T> {
    fn compare_item(&self, other: &T) -> Ordering {
        self.item.cmp(other)
    }
}
fn compare<T: Ord>(item: &T, item2: &T) -> Ordering {
    item.cmp(item2)
}
fn tupleexp() {
    let t = Bar(234, true, Foo { x: 9 });
    // let t2 = (23, false, Foo { x: 23 });
    println!("{} ,{},{}", t.0, t.1, t.2.x);
}
enum Activity {
    Sleeping(u8),
    Skiing,
    Coding(String),
}
fn enumexp(act: Activity) -> String {
    let data = match act {
        Activity::Coding(lang) => format!("Codding {}", lang),
        Activity::Sleeping(hrs) if hrs > 8 => format!("wake up"),
        Activity::Sleeping(hr) => format!("let hime sleep for {}", (8 - hr)),
        Activity::Skiing => format!("skinng"),
    };
    return data;
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
