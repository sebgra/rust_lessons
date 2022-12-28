#[derive(Debug)] // Allow structure to be printed
struct Rectangle{
    width: u32, 
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square piexels.", area(width1, height1));

    // refactoring with tuple
    let rect1 = (30, 50);

    println!("The area of the rectangle is : {} square pixels.", area_bis(rect1));

    // refactoring with structures
    let rect2 = Rectangle{
        width : 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area_ter(rect2));

    let rect3 = Rectangle{ // rect2 cannot be printed because of previous modification. Let'sdefine another rectangle
        width: 30,
        height: 50,
    };

    println!("rect3 is : {:?}", rect3);
    println!("rect3 is with stylished output : {:#?}", rect3);

    // use of the macro dbg!

    let scale = 2;

    let rect4 = Rectangle{ // rect2 cannot be printed because of previous modification. Let'sdefine another rectangle
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect4); // use to display instead of println! scale multiplication is interpreted before display



}


fn area(width: i32, height: i32) -> i32{

    width * height
}

// refactoring with tuple
fn area_bis(dimension: (u32, u32)) -> u32{ // Using a tuple as argument

    dimension.0 * dimension.1
} 

fn area_ter(rectangle: Rectangle) -> u32{
    rectangle.width * rectangle.height
}