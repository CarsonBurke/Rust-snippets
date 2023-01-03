fn main() {
    
    pub struct Manager {
        id_index: i32,
    }
    
    impl Manager {
        pub fn new_id(&mut self) -> String {
            
            self.id_index += 1;
            return (&self.id_index).to_string()
        }
    }
    
    let mut manager = Manager { id_index: 0 };
    
    let mut id = manager.new_id();
    println!("{}", id);
    
    id = manager.new_id();
    println!("{}", id);
}