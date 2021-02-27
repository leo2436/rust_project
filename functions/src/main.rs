// fn main() {
//     let x = 5;

//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {}", y);
// }

// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {}", x);
// }

fn fib(level: i32) -> i32 {
    if level == 1 || level == 2 {
        return 1
    }
    return fib(level - 1) + fib(level - 2);
}

fn main() {
    let res = fib(6);
    println!("the result is {}", res);
}
