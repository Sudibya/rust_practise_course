mod q1_is_even;
mod q2_fibnth_number;
mod q3_characters_in_str;
mod understand_struct;

fn main() {
    // This portion is for q1 that I got from harkirat's course
    let ans = q1_is_even::is_even(23);
    println!("The number is even: {}", ans);
    // **************************************

    // Learn loop and conditions by finding the Fibonacci of the nth number
    let ans_fib: u32 = q2_fibnth_number::fib(33);
    println!("The 10th Fibonacci number is: {}", ans_fib);

//  Learn 
    let str = String:: from("Sudibya");
    let get_len = q3_characters_in_str:: characteres_in_str(str);
    println!("The length of the string str is {}", get_len);



    //understanding the Structs

    understand_struct::get_user();


    //*************************** 


}

