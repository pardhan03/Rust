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