pub fn stack_fn(){
    let a=10;
    let b=10;
    let c=a+b;
    println!("String function: sum of {} and {} is {}",a,b,c);
}

pub fn heap_fn() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

pub fn update_string() {
    // Start with a base string on the heap
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);
    print!("Capacity {}, length {}, pointer {:p}", s.capacity(), s.len(), s.as_ptr());

    // Append some text to the string
    s.push_str(" and some additional text");
    print!("Capacity {}, length {}, pointer {:p}", s.capacity(), s.len(), s.as_ptr());
    println!("After update: {}", s);

    // if we add some big strig that pointer will also chanage because it require contigous memory to stoe the string
}

// headp variable to undrestand ownership model

pub fn heap_own() {
    let s1= String::from("Hello");
    // println!("si variable value {}, s1")

    // beyound this point s1 is no longer valid
    // now the s1 is no longer valid
    // s2 is the owner of the string in the heap
    let s2:String = s1;

    println!("This is the heap variable {}", s2);
}

pub fn string_own() {
    let my_string = String::from("hello");

    takes_ownership(my_string);

    // then the owner will remin the my_string in the string_own function
    // takes_ownership(my_string.clone()); so this line cause error because the ownerhip has been moved

    // till here the own is now takes_ownershop function
    // println!("{}", my_string);
}

pub fn takes_ownership(some_string:String){
    println!("{}", some_string);
}