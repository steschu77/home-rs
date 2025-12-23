// ----------------------------------------------------------------------------
pub enum Key {
    Home,
    Exit,
    NextScene,
    PrevScene,
}

// ----------------------------------------------------------------------------
pub enum Event {
    MouseMove { x: i32, y: i32 },
    ButtonDown { button: u32 },
    ButtonUp { button: u32 },
    Wheel { delta: i32 },
    KeyDown { key: Key },
    KeyUp { key: Key },
}

// ----------------------------------------------------------------------------
pub struct Input {
    events: Vec<Event>,
}

// ----------------------------------------------------------------------------
impl Default for Input {
    fn default() -> Input {
        Input::new()
    }
}

// ----------------------------------------------------------------------------
impl Input {
    pub fn new() -> Input {
        Input { events: Vec::new() }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn take_events(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }
}
