fn main() {
    let x = five();
    let squared_num: i32 = square(x);
    let plused_value: i32 = plus_two(squared_num);
    println!("{x}");
    println!("The square of {x} is {squared_num}");
    println!("{squared_num} + 2 is  {plused_value}");

    let y = {
        let x = 66;
        x + 1
    };


    println!("{y}");
}


fn five() -> i32 {
    return 5
}

fn square(num: i32) -> i32{
    return num * num
}

fn plus_two(number: i32) -> i32 {
    return number + 2
}