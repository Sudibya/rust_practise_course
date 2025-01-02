// The structs in the Rust is similar to the objects in the JS. It is there to store the datat in a stunctured manner. 


//Now we will have a module with the Stuct to understand it properly.


#[derive(Debug)] // top prevent the rust bebugger to debug the below code.

struct User {
    user_id: usize,
    username: String,
    email: String,
    no_of_singned_in: usize
}



pub fn get_user(){
    
    let user = User{
        user_id: 1234,
        username: String::from("Sudibya"),
        email: String::from("sonujena085@gmail.com"),
        no_of_singned_in :4
    };



    println!("The user that we have is {:?}" , user.username);


    //The below line will never work. Because when try to print the stuct with the debug formatter that is {:?} then  we need to use the #[derive(Debug)]  So that the Rust don't debug the line.

    println!("{:?}", user);
}