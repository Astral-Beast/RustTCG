use std::{fmt, path::Display};
pub mod misc;
pub struct Deck {
    cards: Vec<Card>,
}
impl Deck {
    pub fn print_deck(&self) {
        println!("-----------------");
        println!("Player Deck:");
        println!("********************");
        for i in &self.cards {
            i.print_card();
            println!("********************");
        }
    }
    pub fn push_new_card(mut self, new_card: Card) -> Self {
        self.cards.push(new_card);
        self
    }
    pub fn default_deck() -> Deck {
        Deck {
            cards: vec![Card::stab_card(), Card::stab_card()],
        }
    }
}

pub struct Card {
    name: String,
    flavor_text: String,
    cost: Cost,
    attributes: Vec<Attribute>,
}
impl Card {
    //misc card functions
    pub fn print_card(&self) {
        print!(
            "Card Name: {}\nText: {}\nCost: {}\nAttributes:\n",
            self.name, self.flavor_text, self.cost
        );
        misc::print_vec(&self.attributes);
    }

    pub fn push_new_attribute(mut self, new_attr: Attribute) -> Self {
        self.attributes.push(new_attr);
        self
    }
}

pub enum Cost {
    Body(u16),
    Mind(u16),
    Spirit(u16),
}
impl fmt::Display for Cost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Cost::Body(amount) => write!(f, "{} Body", amount),
            Cost::Mind(amount) => write!(f, "{} Mind", amount),
            Cost::Spirit(amount) => write!(f, "{} Spirit", amount),
        }
    }
}

pub enum Attribute {
    Damage(u16),      // X damagage inflicted
    Poison(u16, u16), //X Poison damage for Y Rounds
    Armor(u16),       //protect from x damage
    HealBody(u16),    //Increase Body by X
    HealMind(u16),    //Increase mind by X
    HealSpirit(u16),
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Attribute::Damage(amount) => write!(f, "Inflicts {} Damage", amount),
            Attribute::Poison(poison_damage, rounds) => write!(
                f,
                "Inflicts {} Poison Damage for {} rounds",
                poison_damage, rounds
            ),
            Attribute::Armor(armor) => write!(f, "Player gains {} armor", armor),
            Attribute::HealBody(h) => write!(f, "Player's body heals by {}", h),
            Attribute::HealMind(h) => write!(f, "Player's Mind heals by {}", h),
            Attribute::HealSpirit(h) => write!(f, "Player's Spirit heals by {}", h),
        }
    }
}

impl Card {
    // Standard Card factories
    pub fn stab_card() -> Card {
        Card {
            name: "Stab".to_string(),
            flavor_text: "Viciously stab your opponent.".to_string(),
            cost: Cost::Body(1),
            attributes: vec![Attribute::Damage(2)],
        }
    }
    pub fn anticipate_your_foe_card() -> Card {
        Card {
            name: "Anticipate Your Foe".to_string(),
            flavor_text: "Not all armor is made of steel.".to_string(),
            cost: Cost::Mind(2),
            attributes: vec![Attribute::Armor(4)],
        }
    }
    pub fn recite_litany_of_respite() -> Card {
        Card {
            name: "Recite Litany of Respite".to_string(),
            flavor_text: "The enigmatic monks of the temple of the Silver Star Enclave teach that one must be centered before attempting communion.".to_string(),
            cost: Cost::Spirit(5),
            attributes: vec![Attribute::HealBody(3), Attribute::HealMind(4)],
        }
    }
}
