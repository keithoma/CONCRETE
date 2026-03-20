//! RNG-CONCRETE: The Logic Warehouse

// 1. The Structure (Public so the Lab can see it)
pub struct Unit; 

impl Unit {
    // 2. An 'Associated Function' (like a static method)
    pub fn new(_seed: u8) -> Self { 
        Unit 
    }

    // 3. Methods (they take &self or &mut self)
    pub fn observe(&self) -> u8 { 
        0 
    }
    
    pub fn advance(&mut self) {
        // Logic goes here later
    }

    // 4. A 'Hello' function tied to the Unit
    pub fn hello() {
        println!("Hello from inside the Unit!");
    }
}

// 5. A standalone 'Hello' function (not tied to Unit)
pub fn standalone_hello() {
    println!("Hello, world! I'm a standalone function.");
}