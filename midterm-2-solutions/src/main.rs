extern crate core;

mod ex_1;
mod ex_2;
mod ex_3;
mod ex_4;
mod ex_5;

use crate::ex_1::printnext;
use crate::ex_2::Wrapper;
use crate::ex_3::basicbox_sum;
use crate::ex_4::{Content as EX4_Content, List, Node};
use crate::ex_5::{Content as EX5_Content, Graph, SameBool};

fn main() {
    // ------------------ 1. ------------------

    let x = 5;
    printnext(x);
    let s = 's';
    printnext(s);
    let x = 11;
    printnext(x);
    let s = 'f';
    printnext(s);

    // ------------------ 2. ------------------

    let w = Wrapper {
        v: vec![1, 2, 3, 4, 5, 6, 7, 8],
    };
    let mut iw = w.iter();
    println!("first: {}", iw.next().unwrap());
    for el in iw {
        println!("evens: {}", el);
    }

    let w = Wrapper {
        v: vec![11, 12, 13, 14, 15, 16, 17, 18],
    };
    let mut iw = w.iter();
    println!("first: {}", iw.next().unwrap());
    for el in iw {
        println!("evens: {}", el);
    }

    // ------------------ 3. ------------------

    let s = vec!["asd".to_string(), "where".to_string(), "what".to_string()];
    println!("boxed s: {:?}", basicbox_sum(s));

    let s = vec![
        "nope".to_string(),
        "game".to_string(),
        "bananas".to_string(),
    ];
    println!("boxed s: {:?}", basicbox_sum(s));

    // ------------------ 4. ------------------

    let l: List<i32> = List::new();
    println!("{:?}", l);
    assert_eq!(l.size(), 0);
    let l = List {
        head: Some(Box::new(Node {
            elem: 4,
            next: None,
        })),
        len: 1,
    };
    assert_eq!(l.size(), 1);
    let s: String = format!("{:?}", l);
    assert_eq!(s.contains("Vec"), false);

    let l: List<i32> = List::new();
    println!("{:?}", l);
    assert_eq!(l.get(0), None);
    assert_eq!(l.get(1), None);
    let l = List {
        head: Some(Box::new(Node {
            elem: 4,
            next: None,
        })),
        len: 1,
    };
    assert_eq!(l.get(0), Some(&4));
    let s: String = format!("{:?}", l);
    assert_eq!(s.contains("Vec"), false);

    let elem1 = EX4_Content::new_with("what".to_string(), true, 2);
    let elem2 = EX4_Content::new_with("this".to_string(), false, 18);
    let elem3 = EX4_Content::new_with("dope".to_string(), false, 5);
    let mut l: List<EX4_Content> = List::new();
    println!("{:?}", l);
    l.add(elem1);
    println!("{:?}", l);
    l.add(elem2);
    println!("{:?}", l);
    l.add(elem3);
    println!("{:?}", l);
    let elem4 = EX4_Content::new_with("nope".to_string(), false, 1);
    l.add(elem4);
    println!("{:?}", l);

    let elem1 = EX4_Content::new_with("what".to_string(), true, 2);
    let elem2 = EX4_Content::new_with("this".to_string(), false, 18);
    let elem3 = EX4_Content::new_with("who".to_string(), true, 18);

    assert_eq!(elem1 < elem2, true);
    assert_eq!(elem2 < elem1, false);
    assert_eq!(elem3 == elem2, true);

    // ------------------ 5. ------------------

    let mut el1 = EX5_Content { i: 10, b: true };
    let mut el2 = EX5_Content { i: 11, b: true };
    let mut el3 = EX5_Content { i: 11, b: false };
    assert_eq!(el1 < el2, true);
    assert_eq!(el2 < el1, false);
    assert_eq!(el2 == el3, true);

    let mut el1 = EX5_Content { i: 10, b: true };
    let mut el2 = EX5_Content { i: 11, b: true };
    let mut el3 = EX5_Content { i: 11, b: false };
    assert_eq!(el1.samebool(&el2), true);
    assert_eq!(el1.samebool(&el3), false);

    let mut g: Graph<EX5_Content> = Graph::new();
    println!("{:?}", g);

    let mut el1 = EX5_Content { i: 10, b: true };
    let mut el2 = EX5_Content { i: 11, b: true };
    let mut el3 = EX5_Content { i: 12, b: false };
    let mut g = Graph::new();
    println!("{:?}", g);
    g.add_node(el1);
    println!("{:?}", g);
    g.add_node(el2);
    println!("{:?}", g);
    g.add_node(el3);
    println!("{:?}", g);

    let mut el1 = EX5_Content { i: 10, b: true };
    let mut el2 = EX5_Content { i: 8, b: false };
    let mut g = Graph::new();
    println!("{:?}", g);
    g.add_node(el1);
    println!("{:?}", g);
    g.add_node(el2);
    println!("{:?}", g);

    let mut el1 = EX5_Content { i: 10, b: true };
    let mut el2 = EX5_Content { i: 11, b: true };
    let mut el3 = EX5_Content { i: 12, b: true };
    let mut g = Graph::new();
    println!("{:?}", g);
    g.add_node(el1);
    println!("{:?}", g);
    g.add_node(el2);
    println!("{:?}", g);
    g.add_node(el3);
    println!("{:?}", g);

    let mut el1 = EX5_Content { i: 10, b: true };
    let mut el2 = EX5_Content { i: 9, b: false };
    let mut el3 = EX5_Content { i: 8, b: true };
    let mut g = Graph::new();
    println!("{:?}", g);
    g.add_node(el1);
    println!("{:?}", g);
    g.add_node(el2);
    println!("{:?}", g);
    g.add_node(el3);
    println!("{:?}", g);
}
