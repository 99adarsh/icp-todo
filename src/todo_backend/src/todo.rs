use std::borrow::Cow;

use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};

#[derive(CandidType, Deserialize, Clone)]
pub enum TodoStatus {
    Pending,
    Completed,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Todo {
    pub id: u32,
    text: String,
    status: TodoStatus,
}

impl Todo {
    pub fn new(id: u32, text: String) -> Self {
        Self {
            id,
            text,
            status: TodoStatus::Pending,
        }
    }

    pub fn update_text(&mut self, new_text: String) {
        self.text = new_text
    }

    pub fn update_status(&mut self, new_status: TodoStatus) {
        self.status = new_status
    }
}

// specifies how the type can be serialized/deserialized to be used in StableBTreeMap
impl Storable for Todo {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
