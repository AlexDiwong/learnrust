fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("Chars {} {} {}", c, z, heart_eyed_cat);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
