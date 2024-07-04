// Store the all required states here

use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableCell};
use std::cell::RefCell;

use crate::todo::Todo;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    pub(crate) static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    /// store for next todo id
    pub(crate) static NEXT_ID: RefCell<StableCell<u32, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),1u32
        ).unwrap()
    );

    /// Store for user's todos, Mapped as [(Principal,todo_id) -> Todo]
    pub(crate) static USER_TODOS: RefCell<StableBTreeMap<(Principal,u32),Todo, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
}

/// increase next_id by 1 and return the previous next id
pub(crate) fn get_curr_and_update_next_todo_id() -> u32 {
    NEXT_ID.with(|store| {
        let mut id = store.borrow_mut();
        let next_id = *id.get();
        id.set(next_id + 1).unwrap();
        next_id
    })
}
