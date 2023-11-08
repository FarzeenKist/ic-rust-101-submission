#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ToDo {
    id: u64,
    title: String,
    body: String,
    completed: bool,
    created_at: u64,
    updated_at: Option<u64>,
    deadline: u64
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
fn get_message(id: u64) -> Result<ToDo, Error> {
    match _get_message(&id) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!("a message with id={} not found", id),
        }),
    }
}


#[ic_cdk::query]
fn _get_comepleted_to_dos() -> Result<Vec<ToDo>, Error> {
    let length = STORAGE.with(|service| service.borrow().len());
    let mut todos: Vec<ToDo> = Vec::new();
    for key in 0..length {
        match STORAGE.with(|service| service.borrow().get(&key)) {
            Some(todo) => {
                if todo.completed {
                    todos.push(todo);
                }else {
                    continue;
                }
            }
            None => {},
        }
       
    }
    
    if todos.len() == 0 {
        Err(Error::NotFound {
            msg: format!("There are currently no completed to-dos"),
        })
    }else{
        Ok(todos)
    }
}

#[ic_cdk::update]
fn add_message(message: ToDoPayload) -> Option<ToDo> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let message = ToDo {
        id,
        title: message.title,
        body: message.body,
        completed: false,
        created_at: time(),
        updated_at: None,
        deadline: message.deadline
    };
    do_insert(&message);
    Some(message)
}

#[ic_cdk::update]
fn update_message(id: u64, payload: ToDoPayload) -> Result<ToDo, Error> {
    match STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut message) => {
            message.deadline = payload.deadline;
            message.body = payload.body;
            message.title = payload.title;
            message.updated_at = Some(time());
            do_insert(&message);
            Ok(message)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a message with id={}. message not found",
                id
            ),
        }),
    }
}

#[ic_cdk::update]
fn complete_to_do(id: u64) -> Result<ToDo, Error> {
    match STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut to_do) => {
            to_do.completed = true;
            do_insert(&to_do);
            Ok(to_do)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a message with id={}. message not found",
                id
            ),
        }),
    }
}

// helper method to perform insert.
fn do_insert(message: &ToDo) {
    STORAGE.with(|service| service.borrow_mut().insert(message.id, message.clone()));
}

#[ic_cdk::update]
fn delete_message(id: u64) -> Result<ToDo, Error> {
    match STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a message with id={}. message not found.",
                id
            ),
        }),
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

// a helper method to get a message by id. used in get_message/update_message
fn _get_message(id: &u64) -> Option<ToDo> {
    STORAGE.with(|service| service.borrow().get(id))
}



// need this to generate candid
ic_cdk::export_candid!();