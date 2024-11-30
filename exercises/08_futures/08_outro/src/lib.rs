// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  + Create a ticket
//  + Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TicketId(u64);

impl TicketId {
    pub fn new(id: u64) -> Self {
        TicketId(id)
    }
}

#[derive(Debug, Clone)]
pub struct Ticket {
    pub id: TicketId,
    pub title: String,
    pub description: String,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketDraft {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, Clone)]
pub struct TicketStore {
    tickets: HashMap<TicketId, Arc<Ticket>>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        TicketStore {
            tickets: HashMap::new(),
            counter: 0,
        }
    }

    pub async fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        let ticket = Arc::new(ticket);
        self.tickets.insert(id, ticket);
        id
    }

    pub async fn get(&self, id: TicketId) -> Option<Arc<Ticket>> {
        self.tickets.get(&id).cloned()
    }

    pub async fn get_mut(&mut self, id: TicketId) -> Option<&mut Arc<Ticket>> {
        self.tickets.get_mut(&id)
    }
}
