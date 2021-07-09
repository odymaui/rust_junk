//https://joshleeb.com/posts/rust-traits-and-trait-objects/#:~:text=Rust%20Traits%20and%20Trait%20Objects%201%20Trait%20Objects.,5%20dyn%20Trait.%20...%206%20Wrapping%20Up.%20

//https://doc.rust-lang.org/stable/rust-by-example/trait.html


//NOTE:: display trait will cause a stack overflow if having write accept &self
//recursive definition...
pub trait MyClone : std::fmt::Display {
    fn clone(&self) -> Box<dyn MyClone>;
    fn get_name(&self) -> String;
}

//struct property names not required as part of interface can be anything since implementation is by struct
/*
    Limitation:  factory function needs to know the property name which is external to the struct...
*/
struct Foo {
    //prop: String
    foo: String
}

struct Bar {
    //prop: String
    bar: String
}

struct Baz {
    //prop: String
    baz:String
}

impl MyClone for Foo {
    fn clone(&self) -> Box<dyn MyClone> {
        Box::new(Foo { foo: "Clone:Foo".to_string() })
    }

    fn get_name(&self) -> String {
        self.foo.to_owned()
    }
}

impl std::fmt::Display for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //NOTE:: display trait will cause a stack overflow if having write accept &self
        //thread 'main' has overflowed its stack
        //error: process didn't exit successfully: `target\debug\traits_with_type.exe` (exit code: 0xc00000fd, STATUS_STACK_OVERFLOW)
        //write!(f, "{}", &self)
        write!(f, "{}", &self.get_name())
    }
}

impl MyClone for Bar {
    fn clone(&self) -> Box<dyn MyClone> {
        Box::new(Bar { bar: "Clone:Bar".to_string() })
    }

    fn get_name(&self) -> String{
        self.bar.to_owned()
    }
}

impl std::fmt::Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.get_name())
    }
}

impl MyClone for Baz {
    fn clone(&self) -> Box<dyn MyClone> {
        Box::new(Baz { baz: "Clone:Baz".to_string() })
    }
    fn get_name(&self) -> String {
        self.baz.to_owned()
    }
}

impl std::fmt::Display for Baz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.get_name())
    }
}

fn factory_clone(name: &str) -> Box<dyn MyClone> {
    match name {
        "Foo" => Box::new(Foo { foo: "SomeFoo".to_string() }),
        "Bar" => Box::new(Bar { bar: "SomeBar".to_string() }),
        "Baz" => Box::new(Baz { baz: "SomeBaz".to_string() }),
         _ => panic!("Unknown Type!")
    }
}

fn main() {
    println!("Hello, world!");

    let foo = factory_clone("Foo");
    let bar = factory_clone("Bar");
    let baz = factory_clone("Baz");

    let clones = vec!(&foo,&bar,&baz);

    println!("{}", foo.get_name());
    println!("{}", bar.get_name());
    println!("{}", baz.get_name());

    for clone in clones {
        println!("Clone Name: {} -> {}", clone, clone.get_name());
    }

    println!("{}", foo);
    println!("{}", bar);
    println!("{}", baz);

    println!("{}", foo.clone());
    println!("{}", bar.clone());
    println!("{}", baz.clone());

}
