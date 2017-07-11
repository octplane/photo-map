use std::sync::mpsc::{channel, Sender, Receiver};


pub enum CoordinatorMessage {
    NewClient(Sender<String>),
    Timeout()
}