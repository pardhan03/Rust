use Rust::stack_fn;
fn main() {

    let sentence =String::from("This is my first Rust program");

    let first_word=get_first_word(sentence);

    println!("First word of the String is {}", first_word);

    let string= String::from("Hello world");
    println!("{}",string);
    println!("Hello, world!");

    stack_fn();
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