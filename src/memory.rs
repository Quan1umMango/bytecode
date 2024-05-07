use crate::{
    constants_and_types::*
};

use std::fmt;

/// Main Memory MemoryHandler which contains `Mmeory` units which are separated from each other
/// Think of them like separate regions of allocated memory. 
pub struct MemoryHandler {
    units: Vec<Memory>,

    /// This is used for creating ids for `Memory`.
    /// When a new Memory unit is created, it increments. It never decrements so that no two memory units have the same id 
    ids_count: usize,
}

/// One single Memory Unit 
#[derive(Clone)]
pub struct Memory {
    /// `id` is just like a pointer.
    id: usize,
    contents: Vec<Option<RegisterDataType>>
}

impl MemoryHandler {
    pub fn new() -> Self {
        Self {
            units: Vec::new(),
            ids_count: 0
        }
    }

    pub fn create_memory_unit(&mut self,size:usize) -> usize {
        let id = self.get_new_id();
        let new_unit = Memory::new(id,vec![None;size]);
        self.units.push(new_unit);
        id
    }

    pub fn get_new_id(&mut self) -> usize {
        let id = self.ids_count;
        self.ids_count+=1;
        return id;
    }

    pub fn get(&self,id:usize) -> Option<&Memory> {
        for m in self.units.iter() {
            if m.id == id {
                return Some(m)
            }
        }
        None
    }

    pub fn get_mut(&mut self, id:usize) -> Option<&mut Memory> {
        for m in self.units.iter_mut() {
            if m.id == id {
                return Some(m)
            }
        }
        None
    }

    pub fn free(&mut self, id:usize) -> Result<(),MemoryError> {
        if self.get(id).is_none() {
            return Err(MemoryError::new("Unit does not exist.".to_string()))
        }
        self.units.retain(|m| m.id != id);
        Ok(())
    }
}

impl Memory {
    pub fn new(id:usize,contents: Vec<Option<RegisterDataType>>) -> Self {
        Self {
            id, contents
        }
    }

    pub fn id(&self) -> usize {
        return self.id 
    }

    pub fn get(&self, offset:usize) -> Option<RegisterDataType> {
        *self.contents.get(offset).unwrap_or(&None)
    }

    pub fn try_set(&mut self, offset:usize, new_value:RegisterDataType) -> Result<(),MemoryError> {
        if self.contents.get(offset).is_none() {
            return Err(MemoryError::new(format!("Cannot set memory location {:?} in memory unit {:?} as it does not exist.",offset,self.id)))
        }else {
            self.contents[offset] = Some(new_value);
            Ok(())
        }
    }

}


pub struct MemoryError {
    msg: String 
}

impl MemoryError {
    pub fn new(msg:String) -> Self {
        Self { msg }
    }
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Memory Error: {:?}",self.msg) 
    }
}

impl fmt::Debug for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}
