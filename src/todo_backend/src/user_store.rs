use std::cell::RefCell;

use candid::Principal;
use ic_stable_structures::{Memory, StableBTreeMap};

use crate::{
    error::Error,
    pagination::PageInfo,
    todo::{Todo, TodoStatus},
};

pub(crate) struct UserStore<'a, M: Memory> {
    principal: Principal,
    store: &'a RefCell<StableBTreeMap<(Principal,u32), Todo, M>>,
}

impl<'a, M: Memory> UserStore<'a, M> {
    pub(crate) fn new(principal: Principal,store: &'a RefCell<StableBTreeMap<(Principal,u32), Todo, M>>) -> Self {
        Self { principal,store }
    }

    pub(crate) fn add_todo(&self, todo: Todo) {
        self.store.borrow_mut().insert((self.principal,todo.id), todo);
    }

    pub(crate) fn update_todo(&self, id: u32, updated_text: String) -> Result<(), Error> {
        let mut prev_todo = self.store.borrow().get(&(self.principal,id)).ok_or(Error::TodoNotFound)?;
        prev_todo.update_text(updated_text);
        prev_todo.update_status(TodoStatus::Pending);
        self.store.borrow_mut().insert((self.principal,id), prev_todo);
        Ok(())
    }

    pub(crate) fn update_todo_status(
        &self,
        id: u32,
        updated_status: TodoStatus,
    ) -> Result<(), Error> {
        let mut prev_todo = self.store.borrow().get(&(self.principal,id)).ok_or(Error::TodoNotFound)?;
        prev_todo.update_status(updated_status);
        self.store.borrow_mut().insert((self.principal,id), prev_todo);
        Ok(())
    }

    pub(crate) fn delete_todo(&self, id: u32) {
        self.store.borrow_mut().remove(&(self.principal,id));
    }

    pub(crate) fn get_todo(&self, id: u32) -> Result<Todo, Error> {
        self.store.borrow().get(&(self.principal,id)).ok_or(Error::TodoNotFound)
    }

    pub(crate) fn get_all_todo(&self, page_info: &PageInfo) -> Vec<Todo> {
        self.store
            .borrow()
            .range((self.principal,1u32)..)
            .skip(page_info.offset)
            .take_while(|((addr,_),_)|addr == &self.principal)
            .take(page_info.page_size)
            .map(|(_, todo)| todo.clone())
            .collect()
    }
}
