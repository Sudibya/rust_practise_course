pub fn fib(num : u32)->u32{

    let mut first = 0;
    let mut second =1;

    if num ==0{
        return 0;
    }
    if num ==1{
        return 1;
    }

    for _ in 0..num-1{
        let temp= second;
        second = first + second;
        first = temp;
    }

    return second;
}