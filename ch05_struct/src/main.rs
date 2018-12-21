#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// In rust, it is allowed to provide method in differect block
impl Rectangle {
    fn can_hold(&self, target: &Rectangle) -> bool {
        self.width > target.width && self.height > target.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 30;
    println!(
        "The area i {}",
        area(width1, height1)
    );

    // using tuple
   println!(
        "The area i {}",
        area_by_tuple((10, 10))
    ); 

    let rect = Rectangle {
        width: 5,
        height: 5
    };

    let rect_small = Rectangle {
        width: 4,
        height: 4
    };
    
    println!("The area of struct {:?} is {}", &rect, area_by_struct(&rect));

    println!("The ares of a rect {:#?} using method is, {}", &rect, rect.area());

    println!("Can rect hold rect_small? {}", rect.can_hold(&rect_small));
}

fn area(width: u32, height: u32) -> u32 {
    return width * height
}

fn area_by_tuple(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1
}

fn area_by_struct(rect: &Rectangle) -> u32 {
    return rect.width * rect.height
}