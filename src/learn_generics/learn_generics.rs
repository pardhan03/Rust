use std::vec;

pub fn largest<A : std::cmp::PartialOrd>(list:&[A]) -> &A{
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

    let list_2 =vec![23.2, 14.8, 33.4,88.9];

    let largest_num = largest(&list);

    // let largest_num_2 = largest_i32(&list_2) Now we can use the same function because it has different return type
    // to resolve this proble we will use generics
    let largest_num_2 = largest(&list_2);

    println!("Largest Number in the lists are {}, {}", largest_num, largest_num_2)
}