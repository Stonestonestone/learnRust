fn main() {
    println!("Hello, world!");

    let a = &Rect {
        width: 30,
        length:50,
    };
    let b = &Rect {
        width: 20,
        length:40,
    };
    let c = &Rect {
        width: 31,
        length:51,
    };
    let d = &Rect::square(30);

    println!("{}",a.area());

    println!("{}",a.can_hold(b));

    println!("{}",a.can_hold(c));

    println!("{}",d.area());

    println!("{:#?}",d);

    println!("the width is '{}' length is '{}' of c", c.widthx() ,c.lengthx());

    println!("the width is '{}' length is '{}' of a", a.width,a.length);

    let b_length = &b.length;
    let b_width = &b.width;

    println!("the width is '{}' length is '{}' of b", b_width,b_width)
}

#[derive(Debug)]
struct Rect {
    width: u32,
    length: u32,
}

impl Rect {
    fn area (&self) -> u32 {
        self.width * self.length
    }
    fn can_hold (&self, another: &Rect) -> bool {
        self.length > another.length && self.width > another.width
    }
    fn square (size: u32) -> Rect {
        Rect {
            width: size,
            length: size,
        }
    }
    fn lengthx (&self) -> u32 {
        self.length
    }
    fn widthx (&self) -> u32 {
        self.width
    }

}