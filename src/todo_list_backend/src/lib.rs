use ic_cdk::storage;
use ic_cdk_macros::*;
use std::cell::RefCell;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
struct Task {
    id: u64,
    title: String,
    category: String, // "Personal", "Work", etc.
    important: bool,
    completed: bool,
}

thread_local! {
    static TASKS: RefCell<Vec<Task>> = RefCell::default();
}

#[update]
fn add_task(title: String, category: String, important: bool) -> u64 {
    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        let id = tasks.len() as u64 + 1;
        tasks.push(Task { id, title, category, important, completed: false });
        id
    })
}

#[query]
fn get_tasks() -> Vec<Task> {
    TASKS.with(|tasks| tasks.borrow().clone())
}

#[query]
fn get_important_tasks() -> Vec<Task> {
    TASKS.with(|tasks| tasks.borrow().iter().filter(|t| t.important).cloned().collect())
}

#[query]
fn get_completed_tasks() -> Vec<Task> {
    TASKS.with(|tasks| tasks.borrow().iter().filter(|t| t.completed).cloned().collect())
}

#[update]
fn toggle_task(id: u64) {
    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
            task.completed = !task.completed;
        }
    })
}

#[update]
fn mark_important(id: u64) {
    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
            task.important = !task.important;
        }
    })
}

#[update]
fn delete_task(id: u64) {
    TASKS.with(|tasks| {
        tasks.borrow_mut().retain(|task| task.id != id);
    })
}
