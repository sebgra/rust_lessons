#[derive(Debug)]
struct Rectangle{

    width : u32,
    height : u32,
}

impl Rectangle{
    fn area(&self) -> u32{

        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle)->bool{ // self for applicatble method on the object itself and other as a reference for comparison

        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self{ // associated function, self mentioned that return type is Rectangle
        Self{
            width : size,
            height : size,
        }
    }
}

impl Rectangle{
    fn print_description(&self){
        println!("Rectangle: {} x {}", self.width, self.height);
    }
}   

fn main(){
    let rect1 = Rectangle{
        width : 30, 
        height : 50,
    };

    let rect2 = Rectangle{
        width : 10,
        height : 40,
    };

    let rect3 = Rectangle{
        width : 60,
        height : 45,
    };

    let square = Rectangle::square(3);

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("The area of the square is {} square pixels.", square.area());
    rect1.print_description();
}
