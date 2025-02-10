use ic_cdk_macros::*;
use std::cell::RefCell;
use ic_cdk::{query, update};
use candid::CandidType;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
struct Task {
    id: u64,
    title: String,
    category: String, // Example: "Personal", "Work", etc.
    important: bool,
    completed: bool,
}

thread_local! {        
    static TASKS: RefCell<Vec<Task>> = RefCell::default();
}

#[update]
fn add_task(title: String, category: String, important: bool) -> u64 {   // adding tasks to vector     
    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        let id = (tasks.len() as u64) + 1;
        tasks.push(Task { id, title, category, important, completed: false });
        id
    })
}

#[query]
fn get_tasks() -> Vec<(u64, String, String, bool, bool)> {     //getting all tasks present in vector
    TASKS.with(|tasks| {
        tasks.borrow()
            .iter()
            .map(|task| (task.id, task.title.clone(), task.category.clone(), task.completed, task.important)) 
            .collect()
    })
}

#[query]
fn get_important_tasks() -> Vec<Task> { //getter for important tasks
    TASKS.with(|tasks| {
        tasks.borrow()
            .iter()
            .filter(|t| t.important)
            .cloned()
            .collect()
    })
}

#[query]
fn get_completed_tasks() -> Vec<Task> {  //tasks marked as  completed comme via this getter
    TASKS.with(|tasks| {
        tasks.borrow()
            .iter()
            .filter(|t| t.completed)
            .cloned()
            .collect()
    })
}

#[update]
fn toggle_task(id: u64) {    //toggling tasks to change it from completed to not completed
    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
            task.completed = !task.completed;
        }
    })
}

#[update]
fn mark_important(id: u64) {  //marking the tasks in array important
    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
            task.important = !task.important;
        }
    })
}

#[update]
fn delete_task(id: u64) {  //deleting tasks from array/vector
    TASKS.with(|tasks| {
        tasks.borrow_mut().retain(|task| task.id != id);
    })
}
