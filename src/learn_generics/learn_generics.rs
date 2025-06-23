use std::vec;

pub fn largest_i32(list:&[i32]) -> &i32{
    let mut largest = &list[0];

    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

pub fn gen_fun(){
    let list: Vec<i32> = vec![1,2,3,4];
    let largest_num = largest_i32(&list);
    println!("Largest Number in the list is {}", largest_num)
}