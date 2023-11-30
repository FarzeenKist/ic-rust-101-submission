#[macro_use]
extern crate serde;
use candid::{Decode, Encode, Principal};
use ic_cdk::api::{time, caller};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;


#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct ToDo {
    owner: Principal,
    id: u64,
    title: String,
    body: String,
    completed: bool,
    created_at: u64,
    updated_at: Option<u64>,
    deadline: Option<u64>,
    completed_late: bool // a boolean field that stores whether the to_do was completed after the deadline
}


// a trait that must be implemented for a struct that is stored in a stable struct
impl Storable for ToDo {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// another trait that must be implemented for a struct that is stored in a stable struct
impl BoundedStorable for ToDo {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static STORAGE: RefCell<StableBTreeMap<u64, ToDo, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct ToDoPayload {
    title: String,
    body: String,
    deadline: u64
}

#[ic_cdk::query]
fn get_to_do(id: u64) -> Result<ToDo, Error> {
    match _get_to_do(&id) {
        Some(to_do) => Ok(to_do),
        None => Err(Error::NotFound {
            msg: format!("a to-do with id={} not found", id),
        }),
    }
}


#[ic_cdk::query]
fn get_completed_to_dos() -> Result<Vec<ToDo>, Error> {
    
    let todosmap : Vec<(u64, ToDo)> =  STORAGE.with(|service| service.borrow().iter().collect());
    let length = todosmap.len();
    let mut todos: Vec<ToDo> = Vec::new();
    
    // loop through the todosmap to find and push all completed to-dos to the todos Vec
    for key in 0..length {
        // fetch to-do from todosmap with index key
        let todo = todosmap.get(key).unwrap().clone().1;
        // if to-do is completed, push to the todos Vec
        // otherwise, continue to the next iteration
        if todo.completed {
            todos.push(todo);
        }else{
            continue;
        }
       
    }
    // if no todo has been completed, return an error
    // otherwise return the completed todos
    if todos.len() == 0 {
        Err(Error::NotFound {
            msg: format!("There are currently no completed to-dos"),
        })
    }else{
        Ok(todos)
    }
}


// function to fetch and return all todos
#[ic_cdk::query]
fn get_all_to_dos() -> Result<Vec<ToDo>, Error> {
    let todos : Vec<ToDo> =  STORAGE.with(|service| service.borrow().iter().map(|to_do| to_do.1).collect());
    if todos.len() == 0 {
        return Err(Error::NotFound {
            msg: format!("There are currently no to-dos"),
        });
    }
    Ok(todos)
}

#[ic_cdk::update]
fn add_to_do(payload: ToDoPayload) -> Option<ToDo> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let to_do = ToDo {
        owner: caller(),
        id,
        title: payload.title,
        body: payload.body,
        completed: false,
        created_at: time(),
        updated_at: None,
        deadline: Some(payload.deadline),
        completed_late: false
    };
    do_insert(&to_do);
    Some(to_do)
}


#[ic_cdk::update]
fn update_to_do(id: u64, payload: ToDoPayload) -> Result<ToDo, Error> {
    match STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut to_do) => {
            // if caller isn't the owner of the to-do, return an error
            if !_check_if_owner(&to_do){
                return Err(Error::NotAuthorized {
                    msg: format!(
                        "couldn't update a to-do with id={}. to-do not found",
                        id
                    ),
                    caller: caller()
                })
            }
            // update to-do with the payload
            to_do.body = payload.body;
            to_do.title = payload.title;
            to_do.updated_at = Some(time());
            do_insert(&to_do);
            Ok(to_do)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a to-do with id={}. to-do not found",
                id
            ),
        }),
    }
}

#[ic_cdk::update]
fn complete_to_do(id: u64) -> Result<ToDo, Error> {
    match STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut to_do) => {
            // if caller isn't the owner of the to-do, return an error
            if !_check_if_owner(&to_do){
                return Err(Error::NotAuthorized {
                    msg: format!(
                        "You're not the owner of the to-do with id={}",
                        id
                    ),
                    caller: caller()
                })
            }
            // Ensure that this function can only mutate to-dos that haven't been completed
            assert!(!to_do.completed, "To-do is already completed.");
            // if deadline of to-do is over, set the completed_late field to true
            if  to_do.deadline.is_some_and(|deadline| time() > deadline){
                to_do.completed_late = true;
            }
            to_do.completed = true;
            do_insert(&to_do);
            Ok(to_do)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a to-do with id={}. To-do not found",
                id
            ),
        }),
    }
}

// helper method to perform insert.
fn do_insert(to_do: &ToDo) {
    STORAGE.with(|service| service.borrow_mut().insert(to_do.id, to_do.clone()));
}

#[ic_cdk::update]
fn delete_to_do(id: u64) -> Result<ToDo, Error> {
    let to_do = _get_to_do(&id).expect("Todo not found.");
    // if caller isn't the owner of the to-do, return an error message
    if !_check_if_owner(&to_do){
        return Err(Error::NotAuthorized {
            msg: format!(
                "You're not the owner of the to-do with id={}",
                id
            ),
            caller: caller()
        })
    }
    match STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(to_do) => Ok(to_do),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a to-do with id={}. To-do not found.",
                id
            ),
        }),
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    NotAuthorized {msg: String , caller: Principal},
}

// a helper method to get a to-do by id
fn _get_to_do(id: &u64) -> Option<ToDo> {
    STORAGE.with(|service| service.borrow().get(id))
}

// a helper function to check whether the caller is the owner of the to-do
fn _check_if_owner(to_do: &ToDo) -> bool {
    if to_do.owner.to_string() != caller().to_string(){
        false  
    }else{
        true
    }
}



// need this to generate candid
ic_cdk::export_candid!();
