#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

#[allow(clippy::unit_cmp, clippy::let_unit_value)]
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 70,
    };

    let rect4 = Rectangle::square(30);


    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?: {}", rect1.can_hold(&rect3));
    println!("{:#?}", &rect4);

    // Synthactic Sugar

    let mut r = Rectangle {
        width: 2,
        height: 4,
    };

    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);

    let r_width = r.set_width(2);
    let r_width1 = Rectangle::set_width(&mut r, 2);
    assert_eq!(r_width, r_width1);
}
