#[derive(Debug)]
struct Fighter {
    name: String,
}

impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }
}


fn main() {
    let mut fighters = vec![
        Fighter::new("Dustin Poirier")
    ];
    
    println!("{:?}", fighters)
}
