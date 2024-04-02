use std::fmt;
use crate::misc;

pub struct Agent {
    name: String,
    mind: u16,
    body: u16,
    spirit: u16,
    armor: u16,
    conditions: Vec<Condition>,
}

impl Agent {
    pub fn player_start() -> Agent {
        Agent{
            name: "Player".to_string(),
            mind: 10,
            body: 10,
            spirit: 10,
            armor: 0,
            conditions: vec![],
        }
    }
    pub fn add_condition(mut self, new_condition: Condition) -> Agent{
        // Adds conditions to an agent, such as burning, paralyzed, etc
        for i in self.conditions.iter_mut(){
            if i.condition_type == new_condition.condition_type{
                i.condition_amount =i.condition_amount+new_condition.condition_amount;
                match i.magnitude {
                    Some(_x) =>{
                        i.magnitude = Some(i.magnitude.unwrap()+new_condition.magnitude.unwrap())}
                    _ => (),    
                } 
                //i.condition_amount += new_condition.condition_amount;
                return self;
            }

        }
        self.conditions.push(new_condition);
        return self;

    }
    pub fn print_agent(&self){
        print!("Agent Name: {}\nHealth Values:\nMind: {}  Body: {}  Spirit: {}\nArmor: {}\n", self.name, self.mind, self.body, self.spirit, self.armor);
        misc::print_vec(&self.conditions);

    }
}

#[derive(PartialEq, Eq)]
pub enum ConditionType {
    Burning(),
    Paralyzed(),
    Poisoned(),
    Inspired(),
    Demoralized(),

}
pub struct Condition {
    condition_type: ConditionType,
    condition_amount: u16,
    magnitude: Option<u16>,
}
impl Condition {

    pub fn on_fire() -> Condition{
        Condition{
        condition_type:ConditionType::Burning(),
        condition_amount: 5,
        magnitude:Some(3)
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f,"Agent has the condition ''{}'' for {} rounds. The condition has a magnitude of {:?}.", 
                        self.condition_type.stringify(), self.condition_amount, self.magnitude.unwrap());
        
    }
}

impl ConditionType {
    fn stringify(&self) -> &str {
        // Makes printing conditions prettier without using multiple layers of fmt::Display
        match &self {
            ConditionType::Burning() =>"Burning",
            ConditionType::Demoralized() => "Demoralyzed",
            ConditionType::Inspired() => "Inspired",
            ConditionType::Paralyzed() => "Paralyzed",
            ConditionType::Poisoned() => "Poisoned",
        }
    }
}