use std::{sync::mpsc::channel, thread, time::Duration};

use super::InputEvent;

pub struct Events {
    rx: tokio::sync::mpsc::Receiver<InputEvent>,
    _tx: tokio::sync::mpsc::Sender<InputEvent>,
}

impl Events {}
