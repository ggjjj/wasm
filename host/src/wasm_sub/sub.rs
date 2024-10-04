/* fn main() {
    let num1 = 5;
    let num2 = 10;
    let result = num1 - num2;

    println!("The difference of {} and {} is {}", num1, num2, result);
}
*/
#[no_mangle]
pub extern "C" fn sub(a: u32, b: u32) -> u32 {
    a - b
}