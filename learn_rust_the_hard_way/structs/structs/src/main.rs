fn main() {
    let joe = Person::new("Joe Alex".to_string(), 32, 64, 140);
    let frank = Person::new("Frank Blank".to_string(), 20, 72, 180);
    println!("Joe is at memory location :");
    joe.print();
    println!("Joe is at memory location ");
    frank.print();
;
}

struct Person {
    name: String,
    age: i32,
    height: i32,
    weight: i32,
}

impl Person {
    fn new(name: String, age: i32, height: i32, weight: i32) -> Person {
        Person {
            name,
            age, 
            height,
            weight,
        }
    }
    fn print(&self) {
        println!("Name: {}", self.name);
        println!("\tAge: {}", self.age);
        println!("\tHeight: {}", self.height);
        println!("\tWeight: {}", self.weight);
    }
}