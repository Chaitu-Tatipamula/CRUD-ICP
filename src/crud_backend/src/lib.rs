use ic_cdk::{init, query, update};
use candid::{Nat, CandidType, Deserialize};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Default, CandidType, Deserialize, Clone)]
struct Todo {
    id: Nat,
    desc: String,
    completed: bool,
}

type TodoId = Nat;

thread_local! {
    static TODOS: RefCell<HashMap<TodoId, Todo>> = RefCell::new(HashMap::new());
    static LAST_ID: RefCell<TodoId> = RefCell::new(Nat::from(0u64));
}

#[init]
fn init() {
    TODOS.with(|todos| {
        *todos.borrow_mut() = HashMap::new();
    });
    LAST_ID.with(|last_id| {
        *last_id.borrow_mut() = Nat::from(0u64);
    });
}

#[update]
fn create_todo(desc: String) -> Nat {
    TODOS.with(|todos| {
        LAST_ID.with(|last_id| {
            let mut last_id = last_id.borrow_mut();
            *last_id += Nat::from(1u64);
            let id = last_id.clone();
            todos.borrow_mut().insert(id.clone(), Todo {
                id: id.clone(),
                desc,
                completed: false,
            });
            id
        })
    })
}

#[update]
fn update_todo(id: Nat, new_desc: Option<String>, new_completed: Option<bool>) -> String {
    TODOS.with(|todos| {
        let mut todos = todos.borrow_mut();
        if let Some(item) = todos.get_mut(&id) {
            if let Some(description) = new_desc {
                item.desc = description;
            }
            if let Some(completed) = new_completed {
                item.completed = completed;
            }
            "Todo item updated successfully".to_string()
        } else {
            "Todo item not found".to_string()
        }
    })
}

#[query]
fn get_todos() -> Vec<Todo> {
    TODOS.with(|todos| {
        let todos = todos.borrow();
        todos.values().cloned().collect()
    })
}

#[query]
fn get_paginated_todos(page_number: usize, page_size: usize) -> Vec<Todo> {
    TODOS.with(|todos| {
        let todos = todos.borrow();
        let start = page_number * page_size;
        let end = (start + page_size).min(todos.len());
        todos.values().skip(start).take(end - start).cloned().collect()
    })
}

#[query]
fn get_latest_todos() -> Vec<Todo> {
    TODOS.with(|todos| {
        let mut todos = todos.borrow().values().cloned().collect::<Vec<_>>();
        todos.sort_by_key(|item| item.id.clone());
        todos.into_iter().rev().take(10).collect()
    })
}

#[update]
fn delete_todo(id: Nat) -> String {
    TODOS.with(|todos| {
        let mut todos = todos.borrow_mut();
        if todos.remove(&id).is_some() {
            "Todo item deleted successfully".to_string()
        } else {
            "Todo item not found".to_string()
        }
    })
}

ic_cdk::export_candid!();