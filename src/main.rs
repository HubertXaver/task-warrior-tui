use eyre::Result;
use std::{cell::RefCell, rc::Rc};

use tuitui::{app::App, start_ui};

pub fn main() -> Result<()> {
    let app = Rc::new(RefCell::new(App::new().unwrap()));
    start_ui(app)?;
    Ok(())
}
