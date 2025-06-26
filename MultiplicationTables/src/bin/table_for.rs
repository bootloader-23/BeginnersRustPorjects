use text_io::read;
fn main(){
    println!("Enter the number: ");
    let num : i32 = read!();
    println!("Enter the limit: ");
    let limit : i32 = read!();
    
    for i in 1..=limit {
        println!("{} x {} = {}", num, i, num*i);
    }
}