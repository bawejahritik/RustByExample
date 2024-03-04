fn main(){
    //Hello World this doesn't print
    
    /* 
    1
    2
    3
    Oh wow, this is hidden to the compiler as well!
    */
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x); // prints 10
    let x = 5 + 90 +  5;
    println!("Is `x` 10 or 100? x = {}", x); // prints 90
    println!("This works")
}