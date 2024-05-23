pub struct ThinkingPhilosopher {
    name: String,
}
impl ThinkingPhilosopher {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }

    pub fn take_forks(&self) -> HungryPhilosopher {

    }
}

pub struct HungryPhilosopher;
impl HungryPhilosopher {
    pub fn eat(&self) -> EatingPhilosopher {

    }
}

pub struct EatingPhilosopher;
impl EatingPhilosopher {
    pub fn put_forks(&self) -> ThinkingPhilosopher {

    }
}
