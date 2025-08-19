struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rectl = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuples(rectl)
    );

    // Struct
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rectangle)
    );

    // -----------------------------------------------------------------------------
    // ERROR: Rust não sabe automaticamente como exibir structs com {} no println!.
    // Para tipos primitivos como i32, f64, etc., Rust já sabe como imprimir.
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // println!("rect1 is {rect1}");
    // -----------------------------------------------------------------------------

    //     Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the outer attribute #[derive(Debug)] just before the struct definition, as shown in Listing 5-12.

    // #[derive(Debug)]
    // struct Rectangle {
    //     width: u32,
    //     height: u32,
    // }

    // fn main() {
    //     let rect1 = Rectangle {
    //         width: 30,
    //         height: 50,
    //     };

    //     println!("rect1 is {rect1:?}");
    // }
}

fn area(width1: u32, height1: u32) -> u32 {
    width1 * height1
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// & Borrow
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
