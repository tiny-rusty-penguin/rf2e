
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
pub enum AnimalTraits {
    EscapedCaptivity,
}
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
pub enum Animals {
    Animal(AnimalTraits),
    Monkey,
    GoldenEagle,
    Beaver,
    HouseCat,
    Serpent,
    Snake,
    Lion,
    Goat,
    Rooster,
    Wasp,
    Butterfly,
    Moth,
    Caterpillar,
    Owl,
    Sparrow,
    Dragonfly,
    Stag,
    Mule,
    Donkey,
    Horse,
    Pony,
    Camel,
    Elephant,
    Ox,
    Rhinoceros,
    Snail,
    WhiteTiger,
    Mantis,
    Octopus,
}
// todo review implementations
impl Animals {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Monkey,
            Self::GoldenEagle,
            Self::Beaver,
            Self::HouseCat,
            Self::Serpent,
            Self::Snake,
            Self::Lion,
            Self::Goat,
            Self::Rooster,
            Self::Wasp,
            Self::Butterfly,
            Self::Moth,
            Self::Caterpillar,
            Self::Owl,
            Self::Sparrow,
            Self::Dragonfly,
            Self::Stag,
            Self::Mule,
            Self::Donkey,
            Self::Horse,
            Self::Pony,
            Self::Camel,
            Self::Elephant,
            Self::Ox,
            Self::Rhinoceros,
        ]
    }
    pub fn beasts_of_burden() -> Vec<Self> {
        vec![
            Self::Horse,
            Self::Donkey,
            Self::Pony,
            Self::Mule,
            Self::Camel,
            Self::Elephant,
            Self::Ox,
        ]
    }
    pub fn mounts() -> Vec<Self> {
        vec![
            Self::Horse,
            Self::Donkey,
            Self::Pony,
            Self::Mule,
            Self::Camel,
            Self::Elephant,

        ]
    }
    pub fn snakes() -> Vec<Self> {
        vec![
            Self::Serpent,
            Self::Snake,
        ]
    }
    pub fn poisonous_animals() -> Vec<Self> {
        vec![

        ]
    }

    pub fn hounds() -> Vec<Self> {
        vec![

        ]
    }
    pub fn eagles() -> Vec<Self> {
        vec![
            Self::GoldenEagle,
        ]
    }

    pub fn messenger_birds() -> Vec<Self> {
        vec![
            Self::Rooster,
            Self::Sparrow,
        ]
    }
    pub fn farm_animals() -> Vec<Self> {
        vec![
            Self::Goat,
            Self::Rooster,
            Self::Mule,
            Self::Donkey,
            Self::Horse,
            Self::Pony,
            Self::Ox,
        ]
    }
    pub fn hoofed_animals() -> Vec<Self> {
        vec![
            Self::Goat,
            Self::Mule,
            Self::Donkey,
            Self::Horse,
            Self::Pony,
            Self::Ox,
        ]
    }
    pub fn migratory_birds() -> Vec<Self> {
        vec![
        ]
    }
    pub fn watchdog() -> Vec<Self> {
        vec![

        ]
    }




}
