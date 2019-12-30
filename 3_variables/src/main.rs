fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("Spaces length: {}", spaces);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("This array has entries: {}, {}, and {}, among others", a[0], a[2], a[4]);

    another_function(10);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    println!("5: {}", five());

    println!("6: {}", plus_one(five()));
}

fn another_function(x: i32) {
    println!("This is from another function and I received parameter {}.", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
