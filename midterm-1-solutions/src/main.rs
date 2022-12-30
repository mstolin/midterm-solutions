mod ex_6;
mod ex_7;
mod ex_8;
mod ex_9;
mod ex_10;

use crate::ex_6::*;
use crate::ex_7::*;
use crate::ex_8::*;
use crate::ex_9::*;
use crate::ex_10::*;

fn main() {

    // ------------------

    #[derive(Debug)]
    enum A {
        A1(i32, i32, i32),
        A2(char, char),
    }

    #[derive(Debug)]
    enum B {
        B1(i32, i32),
        B2(String),
    }

    fn bfroma(a: A) -> B {
        match a {
            A::A1(a, b, c) => B::B2(format!("{}-{}-{}", a, b, c)),
            A::A2(x, y) => B::B1(x as i32, y as i32),
        }
    }

    let a1 = A::A1(1,2,3);
    let a2 = A::A2('a','b');
    println!("B2: {:?}, B1:{:?}", bfroma(a1), bfroma(a2));
    //assert_eq!(a1, B::B2("1-2-3".to_string()));
    //assert_eq!(a2, B::B1(97, 98));

    let a1 = A::A1(1,6,30);
    let a2 = A::A2('t','z');
    println!("B2: {:?}, B1:{:?}", bfroma(a1), bfroma(a2));
    //assert_eq!(a1, B::B2("1-6-30".to_string()));
    //assert_eq!(a2, B::B1(116, 122));

    // ------------------

    #[derive(Debug)]
    enum Z {
        Y1(i32, i32),
        Y2(bool, String),
    }

    fn maybelength(x: &Z) -> Option<usize> {
        return match x {
            Z::Y1(_, _) => None,
            Z::Y2(_, s) => Some(s.len()),
        }
    }

    let z1 = Z::Y1(1,2);
    let z2 = Z::Y2(true,String::from("new"));
    println!("len {:?}", maybelength(&z1));
    println!("len {:?}", maybelength(&z2));

    let z1 = Z::Y1(1,2);
    let z2 = Z::Y2(true,String::from("newtest"));
    println!("len {:?}", maybelength(&z1));
    println!("len {:?}", maybelength(&z2));

    let z1 = Z::Y1(1,2);
    println!("len {:?}", maybelength(&z1));
    println!("len {:?}", maybelength(&z1));

    // ------------------

    fn prevchar(c: char) -> char {
        let prev = (c as u32) - 1;
        return std::char::from_u32(prev).expect("not able to get previous char");
    }

    fn replwithprev(s: &mut String) -> Result<String, ()> {
        if s.contains('a') {
            return Err(());
        }
        let res = s
            .chars()
            .map(|c| prevchar(c))
            .collect::<String>();

        Ok(res)
    }

    println!("char {}, prev {}", 'c', prevchar('c'));
    println!("char {}, prev {}", 'a', prevchar('a'));
    println!("char {}, prev {}", 'z', prevchar('z'));
    let mut s = String::from("Pign");
    println!("S {}",s);
    let ret = replwithprev(&mut s);
    println!("Returned: {:?}",ret);
    let mut s = String::from("pigna");
    println!("S {}",s);
    let ret = replwithprev(&mut s);
    println!("Returned: {:?}",ret);

    // ------------------

    let x = X::new();
    let y = Y::new();
    println!("X {:?} - Y {:?}", x, y);
    let (x,y) = swapstr(x,y);
    println!("X {} - Y {}", x, y);

    // ------------------

    let c1 = C::C1(0,1);
    let c2 = C::C2(true, String::from("no way jose"));
    println!("gotten {:?}",D::new());
    let d1 = D::new_with_C(c1);
    println!("dbg {:?}",d1);
    println!("fmt {}",d1);
    let d2 = D::new_with_C(c2);
    println!("dbg {:?}",d2);
    println!("fmt {}",d2);
    println!("larger {}",d1.larger());
    println!("larger {}",d2.larger());

    // ------------------

    let mut v1 = vec![String::from("some");12];
    println!("Lengths {:?}", veclengths(&v1));

    // ------------------

    let mut v: Vec<usize> = vec![2;4];
    v.push(10);
    v.push(8);
    v.push(10);
    println!("{:?}",v);
    removeelfromvector(&mut v, 8);
    println!("{:?}",v);

    // ------------------

    let mut v = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
        "e".to_string(),
        "f".to_string(),
    ];

    let ptr = swapelconcat(&mut v, 1, 5);
    println!("{:?}", ptr.expect(""));
}
