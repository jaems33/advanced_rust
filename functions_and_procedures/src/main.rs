fn some_function(param_a: f32, param_b: i32) -> f32 {
    println!("This is a some function");
    // Type casting
    //Can also do this 10f32 or 10 as f32
    let value = param_a + param_b as f32 + 10_f32;
    value
}

fn some_procedure(param_a: f32) {
    println!("You provided {}", param_a);
}

fn some_str_slice_procedure(param: &str){
    println!("I'm in a procedure: {}", param);
}

fn some_string_procedure(param: &String){
    println!("I'm in a procedure: {}", param);
}

// Technically a procedure most of the time
fn main() {
    let value = some_function(10.2, 30392);
    println!("Value was: {}", value);
    some_procedure(10f32);
    some_str_slice_procedure("Sample String Slice");
    let message = String::from("I am a string but with ampersand I'm a borrow");
    /*
        Rust co-erces a String to a string slice if it knows that a function
        or procedure does not change the data.some_string_procedure.
        
        If the compiler detects potential threading or memory issues it will complain.
    */
    some_str_slice_procedure(&message);
    some_string_procedure(&message);
}
