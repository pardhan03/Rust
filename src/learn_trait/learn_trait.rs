pub trait CanSpeak{
    fn speak(&self);
}

struct Human;
struct Dog;

impl CanSpeak for Human{
    fn speak(&self){
        println!("Hello, I am Human");
    }
}

impl CanSpeak for Dog{
    fn speak(&self){
        println!("woof!, woof!")
    }
}

pub fn make_it_speak<T: CanSpeak>(thing: T){
    thing.speak();
}

pub fn get_trait_use() {
    let h = Human;
    let d = Dog;

    make_it_speak(h);
    make_it_speak(d);
}