mod utils;

use utils::learn_enum::{log_direction, calculate};
use Rust::stack_fn;
use Rust::heap_fn;
use Rust::update_string;
use Rust::heap_own;
use Rust::string_own;

use Rust::learn_vec;
fn main() {

    let sentence =String::from("This is my first Rust program");

    let first_word=get_first_word(sentence);

    println!("First word of the String is {}", first_word);

    let string= String::from("Hello world");
    println!("{}",string);
    println!("Hello, world!");

    stack_fn(); // call the function that uses stack memory
    heap_fn(); // call the function that uses heap memory
    update_string(); // call the function that changes size of variable at runtime

    heap_own();

    string_own();

    log_direction();

    calculate();

    utils::create_user();

    utils::test_rectangle();

    learn_vec::learn_vec();
}

fn get_first_word(sentence: String) -> String {
    let mut ans=String::from("");
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return  ans;
}