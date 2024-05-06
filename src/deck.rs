use std::fmt;
use crate::misc;
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
        let mut deck = Deck {
            cards: Vec::new(),
        };
        for _i in 1..4{
            deck = deck.push_new_card(Card::stab_card());    
        }
        deck = deck.push_new_card(Card::recite_litany_of_respite_card());
        deck = deck.push_new_card(Card::smoke_bomb_card());
        deck = deck.push_new_card(Card::anticipate_your_foe_card());
        return deck;
    }
}

pub struct Card {
    name: String,
    flavor_text: String,
    cost: Cost,
    attributes: Vec<Attribute>,
    target: Target
}
impl Card {
    //misc card functions
    pub fn print_card(&self) {
        print!(
            "Card Name:  {}\nText:  {}\nCost:  {}\nTarget:  {}\nAttributes:\n",
            self.name, self.flavor_text, self.cost, self.target
        );
        misc::print_vec(&self.attributes);
    }

    pub fn push_new_attribute(mut self, new_attr: Attribute) -> Self {
        self.attributes.push(new_attr);
        self
    }
}

pub enum Item {
    SmokeBomb(),
}

pub enum Cost {
    Body(u16),
    Mind(u16),
    Spirit(u16),
    ItemConsumed(u16, Item),
}


pub enum Attribute {
    Damage(u16),        //X damagage inflicted
    Poison(u16, u16),   //X Poison damage for Y Rounds
    Armor(u16),         //protect from x damage
    HealBody(u16),      //Increase Body by X
    HealMind(u16),      //Increase mind by X
    HealSpirit(u16),    //Increase Spirit by X
    Blindness(u16) //Impart Blindness for x turns
}


impl Card {
    // Standard Card factories
    pub fn stab_card() -> Card {
        Card {
            name: "Stab".to_string(),
            flavor_text: "Viciously stab your opponent.".to_string(),
            cost: Cost::Body(1),
            attributes: vec![Attribute::Damage(2)],
            target: Target::SingleTarget(),
        }
    }
    pub fn anticipate_your_foe_card() -> Card {
        Card {
            name: "Anticipate Your Foe".to_string(),
            flavor_text: "Not all armor is made of steel.".to_string(),
            cost: Cost::Mind(2),
            attributes: vec![Attribute::Armor(4)],
            target: Target::SelfTarget(),
        }
    }
    pub fn recite_litany_of_respite_card() -> Card {
        Card {
            name: "Recite Litany of Respite".to_string(),
            flavor_text: "The enigmatic monks of the temple of the Silver Star Enclave teach that one must be centered before attempting communion.".to_string(),
            cost: Cost::Spirit(5),
            attributes: vec![Attribute::HealBody(3), Attribute::HealMind(4)],
            target: Target::SelfTarget(),
        }
    }
    pub fn smoke_bomb_card() -> Card {
        Card {
            name: "Smoke Bomb".to_string(),
            flavor_text: "Always have a trick up your sleeve.".to_string(),
            cost: Cost::ItemConsumed(1, Item::SmokeBomb()),
            attributes: vec![Attribute::Blindness(2)],
            target: Target::MultiTarget(),
        }
    }
}

enum Target {
    //Target indicates what agent the Card will operate on. I.e. a healing spell works on the agent that uses the card.
    SelfTarget(), //Targets the user of the card
    SingleTarget(),
    MultiTarget(),
}



//Display functions:
impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Attribute::Damage(amount) => write!(f, "Inflicts {} Damage", amount),
            Attribute::Poison(poison_damage, rounds) => write!(
                f,
                "Inflicts {} Poison Damage for {} rounds",
                poison_damage, rounds
            ),
            Attribute::Armor(armor) => write!(f, "Player gains {} Armor", armor),
            Attribute::HealBody(h) => write!(f, "Player's Body heals by {}", h),
            Attribute::HealMind(h) => write!(f, "Player's Mind heals by {}", h),
            Attribute::HealSpirit(h) => write!(f, "Player's Spirit heals by {}", h),
            Attribute::Blindness(rounds) => write!(f, "Imparts Blindness for {} rounds", rounds)
        }
    }
}
impl fmt::Display for Cost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Cost::Body(amount) => write!(f, "{} Body", amount),
            Cost::Mind(amount) => write!(f, "{} Mind", amount),
            Cost::Spirit(amount) => write!(f, "{} Spirit", amount),
            Cost::ItemConsumed(amount, item_type) => write!(f, "{} {}(s)", amount, item_type)
        }
    }
}
impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Target::SelfTarget() => write!(f, "Targets self."),
            Target::SingleTarget() => write!(f, "Targets a single enemy."),
            Target::MultiTarget() => write!(f,"Targets multiple enemies.")
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Item::SmokeBomb() => write!(f, "Smoke Bomb")
        }
    }
}

