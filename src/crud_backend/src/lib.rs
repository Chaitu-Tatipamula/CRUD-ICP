use ic_cdk::{init, query, storage, update};
use candid::{Nat, CandidType, Deserialize};

#[derive(Default, CandidType, Deserialize, Clone)]
struct Todo{
    id : Nat,
    desc : String,
    completed : bool
}

#[derive(Default, CandidType, Deserialize)]
struct TodoList {
    items: Vec<Todo>,
}

#[init]
fn init() {
    let initial_state = TodoList::default();
    storage::stable_save((initial_state,)).unwrap();
}

#[update]
fn create_todo(desc : String)-> Nat{
    let mut todo_list: TodoList = storage::stable_restore::<(TodoList,)>().unwrap().0;
    let id = Nat::from(todo_list.items.len() as u64 + 1);
    todo_list.items.push(Todo{
        id : id.clone(),
        desc,
        completed : false
    });
    storage::stable_save((todo_list,)).unwrap();
    id
}

#[update]
fn update_todo(id : Nat, new_desc : Option<String>, new_completed : Option<bool>)-> String {
    let mut todo_list: TodoList = storage::stable_restore::<(TodoList,)>().unwrap().0;
    if let Some(item) = todo_list.items.iter_mut().find(|item| item.id == id) {
        if let Some(description) = new_desc {
            item.desc = description;
        }
        if let Some(completed) = new_completed {
            item.completed = completed;
        }
        storage::stable_save((todo_list,)).unwrap();
        "Todo item updated successfully".to_string()    
    }
    else {
        "Todo item not found".to_string()
    }
}

#[query]
fn get_todos()-> Vec<Todo>{
    let todo_list: TodoList = storage::stable_restore::<(TodoList,)>().unwrap().0;
    todo_list.items.clone()
}

#[query]
fn get_paginated_todos(page_number : usize, page_size : usize)-> Vec<Todo>{
    let todo_list: TodoList = storage::stable_restore::<(TodoList,)>().unwrap().0;
    let start = page_number*page_size;
    let end = (start + page_size).min(todo_list.items.len());
    todo_list.items[start..end].to_vec()
}
#[query]
fn get_latest_todos()-> Vec<Todo>{
    let mut todo_list: TodoList = storage::stable_restore::<(TodoList,)>().unwrap().0;
    todo_list.items.sort_by_key(|item| item.id.clone());
    todo_list.items.into_iter().rev().take(10).collect()
}

#[update]
fn delete_todo(id : Nat)-> String{
    let mut todo_list: TodoList = storage::stable_restore::<(TodoList,)>().unwrap().0;
    if let Some(pos) = todo_list.items.iter().position(|item| item.id == id){
        todo_list.items.remove(pos);
        storage::stable_save((todo_list,)).unwrap();
        "Todo item updated successfully".to_string()    
    }
    else {
        "Todo item not found".to_string()
    }
}

ic_cdk::export_candid!();