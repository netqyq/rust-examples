// trait likes interface in Go

struct Student {
    name: String,
    age: u8,
}

impl ToString for Student {
    fn to_string(&self) -> String {
        return format!("My name is {} and I am {}", self.name, self.age);
    }
}

pub fn traits_test() {
    println!("Hello, world!");
    let s1 = Student {
        name: String::from("lily"),
        age: 30,
    };
    println!("{}", s1.to_string());
}
