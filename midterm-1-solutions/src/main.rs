mod ex_10;
mod ex_11;
mod ex_12;
mod ex_13;
mod ex_14;
mod ex_15;
mod ex_16;
mod ex_17;
mod ex_18;
mod ex_19;
mod ex_9;

use crate::ex_10::*;
use crate::ex_11::*;
use crate::ex_12::*;
use crate::ex_13::*;
use crate::ex_14::*;
use crate::ex_15::*;
use crate::ex_16::*;
use crate::ex_17::*;
use crate::ex_18::*;
use crate::ex_19::*;
use crate::ex_9::*;

fn main() {
    // ------------------ 9. ------------------

    let a1 = A::A1(1, 2, 3);
    let a2 = A::A2('a', 'b');
    println!("B2: {:?}, B1:{:?}", bfroma(a1), bfroma(a2));

    let a1 = A::A1(1, 6, 30);
    let a2 = A::A2('t', 'z');
    println!("B2: {:?}, B1:{:?}", bfroma(a1), bfroma(a2));

    // ------------------ 10. ------------------

    let z1 = Z::Y1(1, 2);
    let z2 = Z::Y2(true, String::from("new"));
    println!("len {:?}", maybelength(&z1));
    println!("len {:?}", maybelength(&z2));

    let z1 = Z::Y1(1, 2);
    let z2 = Z::Y2(true, String::from("newtest"));
    println!("len {:?}", maybelength(&z1));
    println!("len {:?}", maybelength(&z2));

    let z1 = Z::Y1(1, 2);
    println!("len {:?}", maybelength(&z1));
    println!("len {:?}", maybelength(&z1));

    // ------------------ 11. ------------------

    let ex = enumx::X::Y(String::from("test"));
    let sx = structx::X {
        i: String::from("asd"),
    };
    println!("Longer {}", modfun::longer(&ex, &sx));
    let ex = enumx::X::Y(String::from("asdasd"));
    println!("Longer {}", modfun::longer(&ex, &sx));

    let ex = enumx::X::Y(String::from("test"));
    let sx = structx::X {
        i: String::from("asdasd"),
    };
    println!("Longer {}", modfun::longer(&ex, &sx));
    let ex = enumx::X::Y(String::from("asdasd"));
    println!("Longer {}", modfun::longer(&ex, &sx));

    let ex = enumx::X::Y(String::from("test"));
    let sx = structx::X {
        i: String::from("asd"),
    };
    println!("Longer {}", modfun::longer(&ex, &sx));
    let ex = enumx::X::Y(String::from("asd"));
    println!("Longer {}", modfun::longer(&ex, &sx));

    // ------------------ 12. ------------------

    println!("Maybediv 2/3 {:?} ", maybediv(2, 3));
    println!("Maybediv 2/0 {:?} ", maybediv(2, 0));

    println!("Maybediv 2/0 {:?} ", maybediv(2, 0));
    println!("Maybediv 2/0 {:?} ", maybediv(2, 0));

    println!("Maybediv 6/3 {:?} ", maybediv(6, 3));
    println!("Maybediv 5/5 {:?} ", maybediv(5, 5));

    // ------------------ 13. ------------------

    let b = Balance {
        amt: 100,
        active: true,
    };
    let b2 = Balance {
        amt: 200,
        active: true,
    };
    println!("maybericher {:?}", b.maybericher(b2));

    let b = Balance {
        amt: 100,
        active: true,
    };
    let b2 = Balance {
        amt: 0,
        active: true,
    };
    println!("maybericher {:?}", b.maybericher(b2));

    let b = Balance {
        amt: 100,
        active: false,
    };
    let b2 = Balance {
        amt: 200,
        active: true,
    };
    println!("maybericher {:?}", b.maybericher(b2));

    let b = Balance {
        amt: 100,
        active: true,
    };
    let b2 = Balance {
        amt: 200,
        active: false,
    };
    println!("maybericher {:?}", b.maybericher(b2));

    // ------------------ 14. ------------------

    println!("char {}, prev {}", 'c', prevchar('c'));
    println!("char {}, prev {}", 'a', prevchar('a'));
    println!("char {}, prev {}", 'z', prevchar('z'));
    let mut s = String::from("Pign");
    println!("S {}", s);
    let ret = replwithprev(&mut s);
    println!("Returned: {:?}", ret);
    let mut s = String::from("pigna");
    println!("S {}", s);
    let ret = replwithprev(&mut s);
    println!("Returned: {:?}", ret);

    // ------------------ 15. ------------------

    let x = X::new();
    let y = Y::new();
    println!("X {:?} - Y {:?}", x, y);
    let (x, y) = swapstr(x, y);
    println!("X {} - Y {}", x, y);

    // ------------------ 16. ------------------

    let c1 = C::C1(0, 1);
    let c2 = C::C2(true, String::from("no way jose"));
    println!("gotten {:?}", D::new());
    let d1 = D::new_with_C(c1);
    println!("dbg {:?}", d1);
    println!("fmt {}", d1);
    let d2 = D::new_with_C(c2);
    println!("dbg {:?}", d2);
    println!("fmt {}", d2);
    println!("larger {}", d1.larger());
    println!("larger {}", d2.larger());

    // ------------------ 17. ------------------

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

    // ------------------ 18. ------------------

    let mut v1 = vec![String::from("some"); 12];
    println!("Lengths {:?}", veclengths(&v1));

    // ------------------ 19. ------------------

    let mut v: Vec<usize> = vec![2; 4];
    v.push(10);
    v.push(8);
    v.push(10);
    println!("{:?}", v);
    removeelfromvector(&mut v, 8);
    println!("{:?}", v);
}
