mod error;
mod pagination;
mod state;
mod todo;
mod user_store;

use error::Error;
use pagination::PageInfo;
use state::{get_curr_and_update_next_todo_id, USER_TODOS};
use todo::{Todo, TodoStatus};
use user_store::UserStore;

#[ic_cdk::update]
fn add_new_todo(todo_text: String) -> u32 {
    let principal = ic_cdk::api::caller();
    let next_todo_id = get_curr_and_update_next_todo_id();

    USER_TODOS.with(|store| {
        let todo_store = UserStore::new(principal,store);
        let new_todo = Todo::new(next_todo_id, todo_text);
        todo_store.add_todo(new_todo);
    });
    next_todo_id
}

#[ic_cdk::update]
fn update_todo(todo_id: u32, updated_text: String) -> Result<(), Error> {
    let principal = ic_cdk::api::caller();
    USER_TODOS.with(|store| {
        let todo_store = UserStore::new(principal,store);
        todo_store.update_todo(todo_id, updated_text)
    })
}

#[ic_cdk::update]
fn update_todo_status(todo_id: u32, status: TodoStatus) -> Result<(), Error> {
    let principal = ic_cdk::api::caller();
    USER_TODOS.with(|store| {
        let todo_store = UserStore::new(principal,store);
        todo_store.update_todo_status(todo_id, status)
    })
}

#[ic_cdk::update]
fn delete_todo(todo_id: u32) {
    let principal = ic_cdk::api::caller();
    USER_TODOS.with(|store| {
        let todo_store = UserStore::new(principal,store);
        todo_store.delete_todo(todo_id)
    })
}

#[ic_cdk::query]
fn get_todo(todo_id: u32) -> Result<Todo, Error> {
    let principal = ic_cdk::api::caller();
    USER_TODOS.with(|store| {
        let todo_store = UserStore::new(principal,store);
        todo_store.get_todo(todo_id)
    })
}

#[ic_cdk::query]
fn get_all_todos(page: Option<u32>, page_size: Option<u32>) -> Vec<Todo> {
    let principal = ic_cdk::api::caller();
    USER_TODOS.with(|store| {
        let todo_store = UserStore::new(principal,store);
        let page_info = PageInfo::new(page, page_size);
        todo_store.get_all_todo(&page_info)
    })
}

ic_cdk::export_candid!();
