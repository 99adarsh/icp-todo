use std::cell::RefCell;

use ic_stable_structures::{Memory, StableBTreeMap};

use crate::{
    error::Error,
    pagination::PageInfo,
    todo::{Todo, TodoStatus},
};

pub(crate) struct TodoStore<'a, M: Memory> {
    store: &'a RefCell<StableBTreeMap<u32, Todo, M>>,
}

impl<'a, M: Memory> TodoStore<'a, M> {
    pub(crate) fn new(store: &'a RefCell<StableBTreeMap<u32, Todo, M>>) -> Self {
        Self { store }
    }

    pub(crate) fn add_todo(&self, todo: Todo) {
        self.store.borrow_mut().insert(todo.id, todo);
    }

    pub(crate) fn update_todo(&self, id: u32, updated_text: String) -> Result<(), Error> {
        let mut prev_todo = self.store.borrow().get(&id).ok_or(Error::TodoNotFound)?;
        prev_todo.update_text(updated_text);
        prev_todo.update_status(TodoStatus::Pending);
        self.store.borrow_mut().insert(id, prev_todo);
        Ok(())
    }

    pub(crate) fn update_todo_status(
        &self,
        id: u32,
        updated_status: TodoStatus,
    ) -> Result<(), Error> {
        let mut prev_todo = self.store.borrow().get(&id).ok_or(Error::TodoNotFound)?;
        prev_todo.update_status(updated_status);
        self.store.borrow_mut().insert(id, prev_todo);
        Ok(())
    }

    pub(crate) fn delete_todo(&self, id: u32) {
        self.store.borrow_mut().remove(&id);
    }

    pub(crate) fn get_todo(&self, id: u32) -> Result<Todo, Error> {
        self.store.borrow().get(&id).ok_or(Error::TodoNotFound)
    }

    pub(crate) fn get_all_todo(&self, page_info: &PageInfo) -> Vec<Todo> {
        self.store
            .borrow()
            .range((1u32)..)
            .skip(page_info.offset)
            .take(page_info.page_size)
            .map(|(_, todo)| todo.clone())
            .collect()
    }
}
