use text_io::read;
fn main() {
    
    println!("Enter the number: ");
    let num : i32 = read!();
    
    println!("Till where do you want the table?");
    let limit : i32 = read!();
    
    let mut i = 1;
    
    loop {
        if i <= limit {
            println!("{} x {} = {}", num, i, num*i);
            i += 1;
        }
        else {
            break;
        }
    }
}