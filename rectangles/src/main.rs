fn main() {
    let rect_a = Rectangle {
        width: 30,
        height: 50,
    };
    let rect_b = Rectangle::square(10);
    let rect_c = Rectangle::square(10);

    println!("rectangle a: {:#?}", rect_a);
    println!("rectangle b: {:#?}", rect_b);
    println!("rectangle c: {:#?}", rect_c);
    println!("The area of rectangle a is: {}.", rect_a.area());
    println!("The area of rectangle b is: {}.", rect_b.area());
    println!("The area of rectangle c is: {}.", rect_c.area());
    println!(
        "Can rectangle a hold rectangle b?: {}",
        rect_a.can_hold(&rect_b),
    );
    println!(
        "Can rectangle b hold rectangle c?: {}",
        rect_b.can_hold(&rect_c),
    );
    println!(
        "Can rectangle c hold rectangle a?: {}",
        rect_c.can_hold(&rect_a),
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 23,
            height: 21,
        };
        let smaller = Rectangle {
            width: 11,
            height: 20,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 23,
            height: 21,
        };
        let smaller = Rectangle {
            width: 11,
            height: 20,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
