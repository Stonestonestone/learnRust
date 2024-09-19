/*fn main() {
    let rect = (30 , 50);
    let b = area(rect);
    println!("{}",area(rect));
}

fn area (dim: (usize , usize)) -> usize {
    dim.0 * dim.1
}
*/

struct C {
    width: usize,
    length: usize,
}

fn area (a: &C) -> usize {
    a.width * a.length
}

fn main() {
    let a = &C {
        width: 30,
        length: 50,
    };
    println!("{}",area(a));
    println!("{:#?}",a);
}

//以上字段在let a 后面设置为借用 &C，故下部的print中无需为&a，否则应为&a