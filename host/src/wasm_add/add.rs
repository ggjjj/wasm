/*pub fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn main() {  
    let num1 = 5;
    let num2 = 10;
    println!("The sum of {} and {} is {:?}", num1, num2, add(num1, num2));
}
*/

#[no_mangle]
pub extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}
