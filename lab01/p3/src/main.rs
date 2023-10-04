fn main() {
    let mut x: i32 = 99;
    while x!=0{
        if x==1{
        println!("{x} bottle of beer on the wall,");
        println!("{x} bottle of beer.");
        println!("Take one down, pass it around,");
        x -= 1;
        }
        else{
        println!("{x} bottles of beer on the wall,");
        println!("{x} bottles of beer.");
        println!("Take one down, pass it around,");
        x -= 1;
        println!("{x} bottle of beer on the wall.");
        }
        println!(" ");
    }
    println!("No bottles of beer on the wall.");

}
