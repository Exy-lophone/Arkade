use crossterm::event::{self, Event};
use std::{sync::mpsc, thread};

pub struct EventHandler {
    tx: mpsc::Sender<Event>,
    pub rx: mpsc::Receiver<Event>,
    handle: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Event>();
        let tx1 = tx.clone();
        let handle = thread::spawn(move || {
            loop {
                tx1.send(event::read().expect("Error occured while reading event"))
                    .expect("Was not able to send message");
            }
        });
        EventHandler { tx, rx, handle }
    }

    pub fn get_new_tx(&self) -> mpsc::Sender<Event> {
        self.tx.clone()
    }
}
