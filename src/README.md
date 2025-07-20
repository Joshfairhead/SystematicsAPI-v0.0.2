Horizon 1 API:
- State manager runs the operation in a hash table.
- Gets its info from genercis
- Which get their info from the data files

Horizon 2 API:
- Replace the hash table with a DHT / Content addressable storage
- Use hashs as field references
- Implement capabilities by reference

Horizon 3....
- Self describing protocols?
- Liberation tools?

TODO
- Rename designations?
- Spike capabilities?
- Spike nested systems? 




//--------------------------------
// Capability Traits Sketch
//--------------------------------

pub trait Capability: std::fmt::Debug {}

pub struct SystemCapabilities {
    pub term_caps: HashMap<String, Vec<Box<dyn Capability>>>,
    pub conn_caps: HashMap<String, Vec<Box<dyn Capability>>>,
}

impl SystemCapabilities {
    pub fn new() -> Self {
        Self {
            term_caps: HashMap::new(),
            conn_caps: HashMap::new(),
        }
    }
    pub fn add_term_capability(&mut self, label: &str, cap: Box<dyn Capability>) {
        self.term_caps.entry(label.to_string()).or_default().push(cap);
    }
    pub fn get_term_capabilities(&self, label: &str) -> Option<&Vec<Box<dyn Capability>>> {
        self.term_caps.get(label)
    }
    pub fn add_conn_capability(&mut self, label: &str, cap: Box<dyn Capability>) {
        self.conn_caps.entry(label.to_string()).or_default().push(cap);
    }
    pub fn get_conn_capabilities(&self, label: &str) -> Option<&Vec<Box<dyn Capability>>> {
        self.conn_caps.get(label)
    }
} 