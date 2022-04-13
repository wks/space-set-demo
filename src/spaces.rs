pub trait Space {
    fn is_in_space(&self, objref: usize) -> bool;
    fn name(&self) -> String;
    fn trace_object(&self, objref: usize) -> usize;
}

pub struct SimpleSpace {
    pub name: String,
    pub start: usize,
    pub length: usize,
}

impl Space for SimpleSpace {
    fn is_in_space(&self, objref: usize) -> bool {
        self.start <= objref && objref < self.start + self.length
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn trace_object(&self, objref: usize) -> usize {
        println!("Tracing 0x{:x} in {}...", objref, self.name);
        objref
    }
}
