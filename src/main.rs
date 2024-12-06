fn main() {
    println!("{}", is_even(43));

    //We can write the code in another way as well

    let ans = is_even(101);

    println!("{}", ans);
}


fn is_even(num: u32) -> bool{

    if num % 2 ==0{
        return true;
    }else{
        return false;
    }

}
