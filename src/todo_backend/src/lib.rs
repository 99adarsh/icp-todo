mod error;
mod pagination;
mod state;
mod todo;
mod todo_store;

use error::Error;
use pagination::PageInfo;
use state::{get_curr_and_update_next_todo_id, TODOS};
use todo::{Todo, TodoStatus};
use todo_store::TodoStore;

#[ic_cdk::update]
fn add_new_todo(todo_text: String) -> u32 {
    let next_todo_id = get_curr_and_update_next_todo_id();

    TODOS.with(|store| {
        let todo_store = TodoStore::new(store);
        let new_todo = Todo::new(next_todo_id, todo_text);
        todo_store.add_todo(new_todo);
    });
    next_todo_id
}

#[ic_cdk::update]
fn update_todo(todo_id: u32, updated_text: String) -> Result<(), Error> {
    TODOS.with(|store| {
        let todo_store = TodoStore::new(store);
        todo_store.update_todo(todo_id, updated_text)
    })
}

#[ic_cdk::update]
fn update_todo_status(todo_id: u32, status: TodoStatus) -> Result<(), Error> {
    TODOS.with(|store| {
        let todo_store = TodoStore::new(store);
        todo_store.update_todo_status(todo_id, status)
    })
}

#[ic_cdk::update]
fn delete_todo(todo_id: u32) {
    TODOS.with(|store| {
        let todo_store = TodoStore::new(store);
        todo_store.delete_todo(todo_id)
    })
}

#[ic_cdk::query]
fn get_todo(todo_id: u32) -> Result<Todo, Error> {
    TODOS.with(|store| {
        let todo_store = TodoStore::new(store);
        todo_store.get_todo(todo_id)
    })
}

#[ic_cdk::query]
fn get_all_todos(page: Option<u32>, page_size: Option<u32>) -> Vec<Todo> {
    TODOS.with(|store| {
        let todo_store = TodoStore::new(store);
        let page_info = PageInfo::new(page, page_size);
        todo_store.get_all_todo(&page_info)
    })
}

ic_cdk::export_candid!();
