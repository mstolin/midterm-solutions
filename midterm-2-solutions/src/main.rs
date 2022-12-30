extern crate core;

mod ex_1;
mod ex_2;
mod ex_3;
mod ex_4;
mod ex_5;

use crate::ex_1::printnext;
use crate::ex_2::Wrapper;
use crate::ex_3::basicbox_sum;
use crate::ex_4::{Content, List, Node};
// use crate::ex_5::{Content, Graph, SameBool};

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

    let w = Wrapper{v:vec![1,2,3,4,5,6,7,8]};
    let mut iw = w.iter();
    println!("first: {}",iw.next().unwrap());
    for el in iw{
        println!("evens: {}",el);
    }

    let w = Wrapper{v:vec![11,12,13,14,15,16,17,18]};
    let mut iw = w.iter();
    println!("first: {}",iw.next().unwrap());
    for el in iw{
        println!("evens: {}",el);
    }

    // ------------------ 3. ------------------

    let s = vec!["asd".to_string(), "where".to_string(), "what".to_string()];
    println!("boxed s: {:?}", basicbox_sum(s));

    let s = vec!["nope".to_string(), "game".to_string(), "bananas".to_string()];
    println!("boxed s: {:?}", basicbox_sum(s));

    // ------------------ 4. ------------------

    let l : List<i32> = List::new();
    println!("{:?}",l);
    assert_eq!(l.size(),0);
    let l = List{ head: Some(Box::new(Node{ elem: 4, next: None })), len: 1 };
    assert_eq!(l.size(),1);
    let s : String = format!("{:?}",l);
    assert_eq!(s.contains("Vec"),false);

    let elem1 = Content::new_with("what".to_string(),true,2);
    let elem2 = Content::new_with("this".to_string(),false,18);
    let elem3 = Content::new_with("dope".to_string(),false,5);
    let mut l : List<Content> = List::new();
    println!("{:?}",l);
    l.add(elem1);
    println!("{:?}",l);
    l.add(elem2);
    println!("{:?}",l);
    l.add(elem3);
    println!("{:?}",l);
    let elem4 = Content::new_with("nope".to_string(),false,1);
    l.add(elem4);
    println!("{:?}",l);

    // ------------------ 5. ------------------

    /*let mut el1 = Content{i:10, b:true};
    let mut el2 = Content{i:11, b:true};
    let mut el3 = Content{i:11, b:false};
    assert_eq!(el1<el2,true);
    assert_eq!(el2<el1,false);
    assert_eq!(el2==el3,true);

    let mut el1 = Content{i:10, b:true};
    let mut el2 = Content{i:11, b:true};
    let mut el3 = Content{i:11, b:false};
    assert_eq!(el1.samebool(&el2), true);
    assert_eq!(el1.samebool(&el3), false);

    let mut g : Graph<Content> = Graph::new();
    println!("{:?}",g);
    */

}
