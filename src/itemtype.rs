
//!
//! item type 
//! 

#[derive(Debug, Clone)]
pub struct item_type<T>
{
    name : String,
    value : T
}


impl<T> item_type<T>
{ 
    pub fn new(name : String, value : T) -> Self {
        item_type{name : name, value : value}
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn set_name(&mut self, name : String) {
        self.name = name;
    }
    pub fn set_value(&mut self, value : T) {
        self.value = value;
    }
}
