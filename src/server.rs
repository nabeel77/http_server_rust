
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        // Self is same like returning Server
        Self {
            address
        }
    }

    pub fn run(self) {
        println!("Server is listening on {} ", self.address);
    }
}
