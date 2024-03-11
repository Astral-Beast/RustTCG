use std::fmt;

pub struct Agent {
    mind: u16,
    body: u16,
    spirit: u16,
    conditions: Vec<Condition>,
}

pub enum Condition {
    Burning(),
    Paralyzed(),
    Poisoned(),
    Inspired(),
    Demoralized(),
}
