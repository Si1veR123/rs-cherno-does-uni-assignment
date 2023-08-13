// https://www.youtube.com/watch?v=kQsHF7C-FUY

#[derive(Clone)]
pub enum SlotStatus {
    NeverUsed,
    Tombstone,
    Occupied(String)
}

pub struct HashTable {
    pub inner: [SlotStatus; 26]
}

impl HashTable {
    pub fn new() -> Self {
        Self { inner: std::array::from_fn(|_| SlotStatus::NeverUsed) }
    }

    fn hash_function(value: &str) -> usize {
        let last_char = value.chars().last().unwrap();
        (last_char as usize) - 97
    }

    pub fn search(&self, value: &str) -> Option<usize> {
        let hash_table_index = HashTable::hash_function(value);

        for i in 0..26 {
            let slot_index = (hash_table_index+i)%26;
            let slot = &self.inner[slot_index];

            match slot {
                SlotStatus::NeverUsed => return None,
                SlotStatus::Occupied(s) => {
                    if s == value {
                        return Some(slot_index)
                    }
                },
                SlotStatus::Tombstone => continue
            }
        }

        None
    }

    pub fn insert(&mut self, value: String) -> bool {
        if self.search(&value).is_some() {
            return false
        }

        let hash_table_index = HashTable::hash_function(&value);

        for i in 0..26 {
            let slot_index = (hash_table_index+i)%26;
            let slot = self.inner.get_mut(slot_index).unwrap();

            match slot {
                SlotStatus::NeverUsed | SlotStatus::Tombstone => {
                    *slot = SlotStatus::Occupied(value);
                    return true;
                },
                SlotStatus::Occupied(_) => {
                    continue
                },
            }
        }

        false
    }

    pub fn delete(&mut self, value: &str) -> Option<String> {
        let slot = self.search(value)?;
        let string = std::mem::replace(&mut self.inner[slot], SlotStatus::Tombstone);
        if let SlotStatus::Occupied(original_value) = string {
            return Some(original_value)
        }
        None
    }

    pub fn execute_operations<I>(&mut self, ops: I)
        where I: IntoIterator<Item = Operation>
    {
        for op in ops {
            match op {
                Operation::Insert(value) => {
                    self.insert(value);
                },
                Operation::Delete(value) => {
                    self.delete(&value);
                }
            }
        }
    }
}

pub enum Operation {
    Insert(String),
    Delete(String)
}

impl TryFrom<&str> for Operation {
    type Error = ();

    fn try_from(value: &str) -> Result<Operation, Self::Error> {
        let (operation, word) = value.split_at(1);
        let operation_char = operation.chars().next().unwrap();

        match operation_char {
            'A' => Ok(Operation::Insert(word.to_string())),
            'D' => Ok(Operation::Delete(word.to_string())),
            _ => Err(())
        }
    }
}

pub fn parse_operations(input: &str) -> Vec<Operation> {
    let mut operations: Vec<Operation> = Vec::new();
    for operation in input.split_ascii_whitespace() {
        operations.push(operation.try_into().expect("Error parsing input."))
    }
    operations
}
