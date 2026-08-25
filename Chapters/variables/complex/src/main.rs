fn main() {
    // tuple data type Can store elements of any type
    let mytup: (i32, f32, u8) = (12, 13.2, 1);
    println!("{}", mytup.0);
    let (a, b, c) = mytup;
    println!("{a}");
    println!("{b}");
    println!("{c}");

    // array can store elements of one type only 
    let myarr = [1,2,3,4,5];
    println!("{}", myarr[0]);
    println!("{}", myarr[3]);
    println!("{}", myarr[1]);
    
    

    let myarr2: [i32; 5 ] = [11,12,13,14,15];
    println!("{}", myarr2[3]);


    // Quick way to start and populate an array

    let quick_arr = [4; 10];
    

}
