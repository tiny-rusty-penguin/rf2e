use std::vec;

use crate::deities::Deities;
use crate::{AbilityScores, Rarity, Sizes, Specials, Traits};
use crate::language::Languages;
use strum::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Ancestries {
    Dwarf,
    Elf,
    Gnome,
    Goblin,
    Halfling,
    Human,
    Leshy,
    Orc,

    Athamaru,
    Azarketi,
    Catfolk,
    Centaur,
    Fetchling,
    Hobgoblin,
    Kholo,
    Kitsune,
    Kobold,
    Lizardfolk,
    Merfolk,
    Minotaur,
    Nagaji,
    Ratfolk,
    Samsaran,
    Tanuki,
    Tengu,
    Tripkee,
    Vanara,
    Wayang,

    Anadi,
    Android,
    Automaton,
    AwakenedAnimal,
    Conrasu,
    Fleshwarp,
    Ghoran,
    Goloma,
    Kashrishi,
    Poppet,
    Sarangay,
    Shisk,
    Shoony,
    Skeleton,
    Sprite,
    Strix,
    Surki,
    Vishkanya,
    Yaksha,
    Yaoguai,

}

pub struct Ancestry {
    pub ancestry: Ancestries,
    pub traits: Vec<Traits>,
    pub hp: u8,
    pub rarity: Rarity,
    pub adult_hood: f32,
    pub life_expectancy: i16,
    pub average_height: f32,
    pub size: Sizes,
    pub speed: u8,
    pub ability_boosts: Vec<AbilityScores>,
    pub ability_flaws: Option<Vec<AbilityScores>>,
    pub free_boosts: u8,
    pub languages: Vec<Languages>,
    pub free_languages: u8,
    pub specials: Vec<Specials>,

    pub common_languages: Option<Vec<Languages>>,
}
pub struct AncestryDescription {
    pub description: &'static str,
    pub you_might: Vec<&'static str>,
    pub others_probaly: Vec<&'static str>,
    pub physical_description: &'static str,
    pub society: &'static str,
    pub beliefs: Option<&'static str>,
}

impl Ancestry {
    pub fn all() -> Vec<Self> {
        vec![
            Self::dwarf(),
            Self::elf(),
            Self::gnome(),
            Self::goblin(),
            Self::halfling(),
            Self::human(),
            Self::leshy(),
            Self::orc(),
            Self::athamaru(),
        ]
    }

    pub fn str_to_ancestry(s: &str) -> Option<Ancestry> {
        match s {
            "dwarf" =>  Some(Self::dwarf()),
            "elf" =>  Some(Self::elf()),
            "gnome" =>  Some(Self::gnome()),
            "goblin" =>  Some(Self::goblin()),
            "halfling" => Some(Self::halfling()),
            "human" => Some(Self::human()),
            "leshy" =>  Some(Self::leshy()),
            "orc" => Some(Self::orc()),
            "athamaru" => Some(Self::athamaru()),
            "azarketi" => Some(Self::azarketi()),
            "catfolk" => Some(Self::catfolk()),
            "centaur" => Some(Self::centaur()),
            "fetchling" => Some(Self::fetchling()),
            "hobgoblin" => Some(Self::hobgoblin()),
            "kholo" => Some(Self::kholo()),
            "kitsune" => Some(Self::kitsune()),
            "kobold" => Some(Self::kobold()),
            "lizardfolk" => Some(Self::lizardfolk()),
            "merfolk" => Some(Self::merfolk()),
            "minotaur" => Some(Self::minotaur()),
            "nagaji" => Some(Self::nagaji()),
            "ratfolk" => Some(Self::ratfolk()),
            "samsaran" => Some(Self::samsaran()),
            "tanuki" => Some(Self::tanuki()),
            "tengu" => Some(Self::tengu()),
            "tripkee" => Some(Self::tripkee()),
            "vanara" => Some(Self::vanara()),
            "wayang" => Some(Self::wayang()),
            "anadi" => Some(Self::anadi()),
            "android" => Some(Self::android()),
            "automaton" => Some(Self::automaton()),
            "awakened_animal" => Some(Self::awakened_animal()),
            "conrasu" => Some(Self::conrasu()),
            "fleshwarp" => Some(Self::fleshwarp()),
            "ghoran" => Some(Self::ghoran()),
            "goloma" => Some(Self::goloma()),
            "kashrishi" => Some(Self::kashrishi()),
            "poppet" => Some(Self::poppet()),
            "sarangay" => Some(Self::sarangay()),
            "shisk" => Some(Self::shisk()),
            "shoony" => Some(Self::shoony()),
            "skeleton" => Some(Self::skeleton()),
            "sprite" => Some(Self::sprite()),
            "strix" => Some(Self::strix()),
            "surki" => Some(Self::surki()),
            "vishkanya" => Some(Self::vishkanya()),
            "yaksha" => Some(Self::yaksha()),
            "yaoguai" => Some(Self::yaoguai()),
            _ => None,
        }
    }

    // Common Ancestries
    pub fn dwarf() -> Self {
        Ancestry {
            ancestry: Ancestries::Dwarf,
            traits: vec![Traits::Dwarf, Traits::Humanoid],
            hp: 10,
            rarity: Rarity::Common,
            adult_hood: 25.0,
            life_expectancy: 350,
            average_height: 4.5,
            size: Sizes::Medium,
            speed: 20,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Wisdom],
            ability_flaws: Some(vec![AbilityScores::Charisma]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Dwarven],
            free_languages: 0,
            specials: vec![Specials::Darkvision, Specials::ClanDagger],
            common_languages: Some(vec![
                Languages::Gnomish,
                Languages::Goblin,
                Languages::Jotun,
                Languages::Orcish,
                Languages::Petran,
                Languages::Sakvroth,
            ]),
        }
    }
    pub fn elf() -> Self {
        Ancestry {
            ancestry: Ancestries::Elf,
            traits: vec![Traits::Elf, Traits::Humanoid],
            hp: 6,
            rarity: Rarity::Common,
            adult_hood: 20.0,
            life_expectancy: 600,
            average_height: 6.5,
            size: Sizes::Medium,
            speed: 30,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Intelligence],
            ability_flaws: Some(vec![AbilityScores::Constitution]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Elven],
            free_languages: 0,
            specials: vec![Specials::LowLightVision],
            common_languages: Some(vec![
                Languages::Draconic,
                Languages::Empyrean,
                Languages::Fey,
                Languages::Gnomish,
                Languages::Goblin,
                Languages::Kholo,
                Languages::Orcish,
            ]),
        }
    }
    pub fn gnome() -> Self {
        Ancestry {
            ancestry: Ancestries::Gnome,
            traits: vec![Traits::Gnome, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Common,
            adult_hood: 18.0,
            life_expectancy: 400,
            average_height: 3.0,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Charisma],
            ability_flaws: Some(vec![AbilityScores::Strength]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Fey, Languages::Gnomish],
            free_languages: 0,
            specials: vec![Specials::LowLightVision],
            common_languages: Some(vec![
                Languages::Draconic,
                Languages::Dwarven,
                Languages::Elven,
                Languages::Goblin,
                Languages::Jotun,
                Languages::Orcish,
            ]),
        }
    }
    pub fn goblin() -> Self {
        Ancestry {
            ancestry: Ancestries::Goblin,
            traits: vec![Traits::Goblin, Traits::Humanoid],
            hp: 6,
            rarity: Rarity::Common,
            adult_hood: 4.5,
            life_expectancy: 20,
            average_height: 3.0,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Charisma],
            ability_flaws: Some(vec![AbilityScores::Wisdom]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Goblin],
            free_languages: 0,
            specials: vec![Specials::Darkvision],
            common_languages: Some(vec![
                Languages::Draconic,
                Languages::Dwarven,
                Languages::Gnomish,
                Languages::Halfling,
                Languages::Kholo,
                Languages::Orcish,
            ]),
        }
    }
    pub fn halfling() -> Self {
        Ancestry {
            ancestry: Ancestries::Halfling,
            traits: vec![Traits::Halfling, Traits::Humanoid],
            hp: 6,
            rarity: Rarity::Common,
            adult_hood: 20.0,
            life_expectancy: 150,
            average_height: 3.0,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Wisdom],
            ability_flaws: Some(vec![AbilityScores::Strength]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Halfling],
            free_languages: 0,
            specials: vec![Specials::KeenEyes],
            common_languages: Some(vec![
                Languages::Dwarven,
                Languages::Elven,
                Languages::Gnomish,
                Languages::Goblin,
            ]),
        }
    }
    pub fn human() -> Self {
        Ancestry {
            ancestry: Ancestries::Human,
            traits: vec![Traits::Human, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Common,
            adult_hood: 15.0,
            life_expectancy: 90,
            average_height: 5.5,
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![],
            ability_flaws: None,
            free_boosts: 2,
            languages: vec![Languages::Common],
            free_languages: 1,
            specials: vec![],
            common_languages: None,

        }
    }
    pub fn leshy() -> Self {
        Ancestry {
            ancestry: Ancestries::Leshy,
            traits: vec![Traits::Leshy, Traits::Plant],
            hp: 8,
            rarity: Rarity::Common,
            adult_hood: 0.0,
            life_expectancy: -1,
            average_height: 3.0,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Wisdom, AbilityScores::Constitution],
            ability_flaws: Some(vec![AbilityScores::Intelligence]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Fey],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::PlantNourishment],
            common_languages: Some(vec![
                Languages::Draconic,
                Languages::Elven,
                Languages::Goblin,
                Languages::Gnomish,
                Languages::Halfling,
                Languages::Sakvroth,
            ]),
        }
    }
    pub fn orc() -> Self {
        Ancestry {
            ancestry: Ancestries::Orc,
            traits: vec![Traits::Orc, Traits::Humanoid],
            hp: 10,
            rarity: Rarity::Common,
            adult_hood: 17.0,
            life_expectancy: 60,
            average_height: 7.0,
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Constitution],
            ability_flaws: Some(vec![AbilityScores::Intelligence]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Orcish],
            free_languages: 0,
            specials: vec![Specials::Darkvision],
            common_languages: Some(vec![
                Languages::Goblin,
                Languages::Jotun,
                Languages::Petran,
                Languages::Sakvroth,
            ]),
        }
    }

    // Uncommon Ancestries
    pub fn athamaru() -> Self {
        Self {
            ancestry: Ancestries::Athamaru,
            traits: vec![Traits::Amphibious, Traits::Athamaru, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Uncommon, // todo review ages later could be a better way to deal with this
            adult_hood: 15.0, // unknown using human
            life_expectancy: 90, // unknown using human
            average_height: 5.5,
            size: Sizes::Medium,
            speed: 20, // todo 25 feet swim speed
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Wisdom],
            ability_flaws: Some(vec![AbilityScores::Intelligence]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Thalassic],
            free_languages: 0,
            specials: vec![Specials::LowLightVision],
            common_languages: Some(vec![
                Languages::Alghollthu,
                Languages::AncientAzlanti, // todo ancient??
                Languages::Fey,
                Languages::Tien,
            ]),
        }
    }
    pub fn azarketi() -> Self {
        Self {
            ancestry: Ancestries::Azarketi,
            traits: vec![Traits::Amphibious, Traits::Azarketi, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Uncommon,
            adult_hood: 15.0,
            life_expectancy: 100, // todo "known to live longer than humans"
            average_height: 5.5,
            size: Sizes::Medium,
            speed: 20,  // todo 30 feet swim speed
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Charisma],
            ability_flaws: Some(vec![AbilityScores::Wisdom]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Alghollthu],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::Hydration],
            common_languages: Some(vec![
                Languages::Aklo,
                Languages::Thalassic,
                Languages::AncientAzlanti,
                Languages::Draconic,
                Languages::Elven,
                Languages::Sakvroth,
            ]),
        }
    }
    pub fn catfolk() -> Self {
        Self {
            ancestry: Ancestries::Catfolk,
            traits: vec![Traits::Catfolk, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Uncommon,
            adult_hood: 15.0,
            life_expectancy: 65,
            average_height: 5.5,
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Charisma],
            ability_flaws: Some(vec![AbilityScores::Wisdom]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Amurrun],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::LandOnYourFeet],
            common_languages: Some(vec![
                Languages::Draconic,
                Languages::Elven,
                Languages::Gnomish,
                Languages::Goblin,
                Languages::Halfling,
                Languages::Iruxi,
                Languages::Jotun,
                Languages::Fey,
            ]),
        }
    }
    pub fn centaur() -> Self {
        Self {
            ancestry: Ancestries::Centaur,
            traits: vec![Traits::Centaur, Traits::Humanoid, Traits::Beast],
            hp: 8,
            rarity: Rarity::Uncommon,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 7.0,
            size: Sizes::Large,
            speed: 30,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Wisdom],
            ability_flaws: Some(vec![AbilityScores::Charisma]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Fey],
            free_languages: 0,
            specials: vec![Specials::Darkvision, Specials::Mount, Specials::Robust],
            common_languages: Some(vec![
                Languages::Arboreal,
                Languages::Cyclops,
                Languages::Dwarven,
                Languages::Elven,
                Languages::Gnomish,
                Languages::Halfling,
                Languages::Jotun,
            ]),
        }
    }
    pub fn fetchling() -> Self {
        Self {
            ancestry: Ancestries::Fetchling,
            traits: vec![Traits::Fetchling, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Uncommon,
            adult_hood: 15.0,
            life_expectancy: 90, // todo unknown
            average_height: 5.5,
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity],
            ability_flaws: None,
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Shadowtonuge], // todo shadowtounge spelling
            free_languages: 0,
            specials: vec![Specials::Darkvision],
            common_languages: Some(vec![
                Languages::Aklo,
                Languages::Draconic,
                Languages::Dziriak,
                Languages::Necril,
                Languages::Sakvroth,
            ]),
        }
    }
    pub fn hobgoblin() -> Self {
        Self {
            ancestry: Ancestries::Hobgoblin,
            traits: vec![Traits::Hobgoblin, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Uncommon,
            adult_hood: 14.0,
            life_expectancy: 70,
            average_height: 5.5,  // todo unknown
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Intelligence],
            ability_flaws: Some(vec![AbilityScores::Wisdom]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Goblin],
            free_languages: 0,
            specials: vec![Specials::Darkvision],
            common_languages: Some(vec![
                Languages::Draconic,
                Languages::Dwarven,
                Languages::Jotun,
                Languages::Halfling,
                Languages::Kholo,
                Languages::Orcish,
            ]),
        }
    }
    pub fn kholo() -> Self {
        Self {
            ancestry: Ancestries::Kholo,
            traits: vec![Traits::Kholo, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Uncommon,
            adult_hood: 15.0,
            life_expectancy: 60,
            average_height: 6.5,
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Intelligence],
            ability_flaws: Some(vec![AbilityScores::Wisdom]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Kholo],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::Bite],
            common_languages: Some(vec![
                Languages::Draconic,
                Languages::Elven,
                Languages::Fey,
                Languages::Iruxi,
                Languages::Necril,
                Languages::Orcish,
            ]),
        }
    }
    pub fn kitsune() -> Self {
        Self {
            ancestry: Ancestries::Kitsune,
            traits: vec![Traits::Kitsune, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Uncommon,
            adult_hood: 15.0,  // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 5.5, // todo unknown
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Charisma],
            ability_flaws: None,
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Tien],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::ChangeShape],
            common_languages: Some(vec![
                Languages::Elven,
                Languages::Gnomish,
                Languages::Goblin,
                Languages::Halfling,
                Languages::Dwarven,
                Languages::Fey,
            ]),
        }
    }
    pub fn kobold() -> Self {
        Self {
            ancestry: Ancestries::Kobold,
            traits: vec![Traits::Kobold, Traits::Humanoid],
            hp: 6,
            rarity: Rarity::Uncommon,
            adult_hood: 12.0,
            life_expectancy: 60,
            average_height: 3.0,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Charisma],
            ability_flaws: Some(vec![AbilityScores::Constitution]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Sakvroth],
            free_languages: 0,
            specials: vec![Specials::Darkvision],
            common_languages: Some(vec![
                Languages::Aklo,
                Languages::Diabolic,
                Languages::Draconic,
                Languages::Dwarven,
                Languages::Empyrean,
                Languages::Fey,
                Languages::Gnomish,
                Languages::Petran,
            ]),
        }
    }
    pub fn lizardfolk() -> Self {
        Self {
            ancestry: Ancestries::Lizardfolk,
            traits: vec![Traits::Lizardfolk, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Uncommon,
            adult_hood: 15.0,
            life_expectancy: 120,
            average_height: 6.5,
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Wisdom],
            ability_flaws: Some(vec![AbilityScores::Intelligence]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Iruxi],
            free_languages: 0,
            specials: vec![Specials::Claws, Specials::AquaticAdaptation],
            common_languages: Some(vec![
                Languages::Amurrun,
                Languages::Boggard,
                Languages::Draconic,
                Languages::Elven,
                Languages::Fey,
                Languages::Jotun,
                Languages::Thalassic,
            ]),
        }
    }
    pub fn merfolk() -> Self {
        Self {
            ancestry: Ancestries::Merfolk,
            traits: vec![Traits::Merfolk, Traits::Humanoid, Traits::Amphibious],
            hp: 8,
            rarity: Rarity::Uncommon,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 5.5, // todo unknown
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Charisma],
            ability_flaws: Some(vec![AbilityScores::Constitution]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Thalassic],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::AquaticGrace, Specials::Hydration],
            common_languages: Some(vec![
                Languages::Aklo,
                Languages::AncientAzlanti,
                Languages::Draconic,
                Languages::Elven,
                Languages::Fey,
            ]),
        }
    }
    pub fn minotaur() -> Self {
        Self {
            ancestry: Ancestries::Minotaur,
            traits: vec![Traits::Minotaur, Traits::Humanoid, Traits::Beast],
            hp: 10,
            rarity: Rarity::Uncommon,
            adult_hood: 17.0, // todo unknown
            life_expectancy: 150, // todo unknown
            average_height: 7.0, // todo unknown
            size: Sizes::Large,
            speed: 25,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Constitution],
            ability_flaws: Some(vec![AbilityScores::Charisma]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Jotun],
            free_languages: 0,
            specials: vec![Specials::Darkvision, Specials::Horns],
            common_languages: Some(vec![
                Languages::Cyclops,
                Languages::Dwarven,
                Languages::Fey,
                Languages::Petran,
                Languages::Sakvroth,
            ]),
        }
    }
    pub fn nagaji() -> Self {
        Self {
            ancestry: Ancestries::Nagaji,
            traits: vec![Traits::Nagaji, Traits::Humanoid],
            hp: 10,
            rarity: Rarity::Uncommon,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 5.5, // todo unknown
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Strength],
            ability_flaws: None,
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Nagaji],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::Fangs],
            common_languages: Some(vec![
                Languages::Aklo,
                Languages::Amurrun,
                Languages::Thalassic,
                Languages::Empyrean,
                Languages::Draconic,
                Languages::Sakvroth,
                Languages::Shadowtonuge,
                Languages::Tengu,
                Languages::Vanara,
            ]),
        }
    }
    pub fn ratfolk() -> Self {
        Self {
            ancestry: Ancestries::Ratfolk,
            traits: vec![Traits::Ratfolk, Traits::Humanoid],
            hp: 6,
            rarity: Rarity::Uncommon,
            adult_hood: 12.0, // todo unknown
            life_expectancy: 60, // todo unknown
            average_height: 4.0,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Intelligence],
            ability_flaws: Some(vec![AbilityScores::Strength]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Ysoki],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::SharpTeeth],
            common_languages: Some(vec![
                Languages::Aklo,
                Languages::Draconic,
                Languages::Dwarven,
                Languages::Goblin,
                Languages::Gnomish,
                Languages::Halfling,
                Languages::Kholo,
                Languages::Orcish,
                Languages::Sakvroth,
            ]),
        }
    }
    pub fn samsaran() -> Self {
        Self {
            ancestry: Ancestries::Samsaran,
            traits: vec![Traits::Samsaran, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Uncommon,
            adult_hood: 15.0,
            life_expectancy: 100,
            average_height: 5.5, // todo unknown
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Wisdom, AbilityScores::Constitution],
            ability_flaws: Some(vec![AbilityScores::Charisma]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Samsaran],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::WanderersSoul, Specials::Cryptomnesia],
            common_languages: Some(vec![
                Languages::Chthonian,
                Languages::Diabolic,
                Languages::Draconic,
                Languages::Empyrean,
                Languages::Jotun,
                Languages::Petran,
                Languages::Pyric,
                Languages::Sussuran,
                Languages::Thalassic,
            ]),
        }
    }
    pub fn tanuki() -> Self {
        Self {
            ancestry: Ancestries::Tanuki,
            traits: vec![Traits::Tanuki, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Uncommon,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 3.5,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Charisma],
            ability_flaws: Some(vec![AbilityScores::Wisdom]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Tanuki],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::ChangeShape], // todo duplicate change shape with different effects
            common_languages: Some(vec![
                Languages::Elven,
                Languages::Fey,
                Languages::Goblin,
                Languages::Gnomish,
                Languages::Kitsune,
                Languages::Tengu,
            ]),
        }
    }
    pub fn tengu() -> Self {
        Self {
            ancestry: Ancestries::Tengu,
            traits: vec![Traits::Tengu, Traits::Humanoid],
            hp: 6,
            rarity: Rarity::Uncommon,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 4.5,
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity],
            ability_flaws: None,
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Tengu],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::SharpBeak],
            common_languages: Some(vec![
                Languages::Dwarven,
                Languages::Elven,
                Languages::Halfling,
                Languages::Goblin,
                Languages::Gnomish,
                Languages::Fey,
            ]),
        }
    }
    pub fn tripkee() -> Self {
        Self {
            ancestry: Ancestries::Tripkee,
            traits: vec![Traits::Tripkee, Traits::Humanoid],
            hp: 6,
            rarity: Rarity::Uncommon,
            adult_hood: 12.0, 
            life_expectancy: 60, 
            average_height: 2.0,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Wisdom],
            ability_flaws: Some(vec![AbilityScores::Strength]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Tripkee],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::NaturalClimber],
            common_languages: Some(vec![
                Languages::Boggard,
                Languages::Chthonian,
                Languages::Draconic,
                Languages::Elven,
                Languages::Fey,
                Languages::Iruxi,
                Languages::Thalassic,
            ]),
        }
    }
    pub fn vanara() -> Self {
        Self {
            ancestry: Ancestries::Vanara,
            traits: vec![Traits::Vanara, Traits::Humanoid],
            hp: 6,
            rarity: Rarity::Uncommon,
            adult_hood: 13.0, 
            life_expectancy: 60, 
            average_height: 4.5,
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity],
            ability_flaws: None,
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Vanara],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::PrehensileTail],
            common_languages: Some(vec![
                Languages::Elven,
                Languages::Gnomish,
                Languages::Diabolic,
                Languages::Goblin,
                Languages::Fey,
            ]),
        }
    }
    pub fn wayang() -> Self {
        Self {
            ancestry: Ancestries::Wayang,
            traits: vec![Traits::Wayang, Traits::Humanoid, Traits::Shadow],
            hp: 6,
            rarity: Rarity::Uncommon,
            adult_hood: 15.0,  // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 3.0, // todo unknown
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Charisma],
            ability_flaws: Some(vec![AbilityScores::Constitution]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Wayang, Languages::Shadowtonuge],
            free_languages: 0,
            specials: vec![Specials::Darkvision],
            common_languages: Some(vec![
                Languages::Dziriak,
                Languages::Diabolic,
                Languages::Minatan,
                Languages::Nagaji,
                Languages::Thalassic,
                Languages::Vudrani,
                Languages::Yaksha,
            ]),
        }
    }

    // Rare Ancestries
    pub fn anadi() -> Self {
        Self {
            ancestry: Ancestries::Anadi,
            traits: vec![Traits::Anadi, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Rare,
            adult_hood: 13.0,
            life_expectancy: 80,
            average_height: 5.5, // todo unknown
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Wisdom],
            ability_flaws: Some(vec![AbilityScores::Constitution]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Mwangi],
            free_languages: 0,
            specials: vec![Specials::ChangeShape, Specials::Fangs], // todo duplicate change shape with different effects
            common_languages: Some(vec![
                Languages::Draconic,
                Languages::Elven,
                Languages::Kholo,
                Languages::Iruxi,
                Languages::Orcish,
                Languages::Fey,
            ]),            
        }
    }
    pub fn android() -> Self {
        Self {
            ancestry: Ancestries::Android,
            traits: vec![Traits::Android, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Rare,
            adult_hood: 15.0, // toodo unknwon
            life_expectancy: 100, // todo renewal
            average_height: 5.5, // todo unknown
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Intelligence, AbilityScores::Dexterity],
            ability_flaws: Some(vec![AbilityScores::Charisma]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Androffan],
            free_languages: 0,
            specials: vec![Specials::Darkvision, Specials::Constructed, Specials::EmotionallyUnaware],
            common_languages: Some(vec![
                Languages::Chthonian,
                Languages::Empyrean,
                Languages::Draconic,
                Languages::Dwarven,
                Languages::Sakvroth,
                Languages::Utopian,
            ]),
        }
    }
    pub fn automaton() -> Self {
        Self {
            ancestry: Ancestries::Automaton,
            traits: vec![Traits::Automaton, Traits::Humanoid, Traits::Construct],
            hp: 8,
            rarity: Rarity::Rare,
            adult_hood: 15.0, // todo unknown
            life_expectancy: -1, 
            average_height: 5.5, // todo unknown
            size: Sizes::Medium, // todo medium or small
            speed: 25,
            ability_boosts: vec![AbilityScores::Strength],
            ability_flaws: None,
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Utopian],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::AutomationCore, Specials::ConstructedBody],
            common_languages: Some(vec![
                Languages::Chthonian,
                Languages::Thalassic,
                Languages::Sussuran,
                Languages::Empyrean,
                Languages::Elven,
                Languages::Pyric,
                Languages::Diabolic,
                Languages::Petran,
            ]),
        }
    }
    pub fn awakened_animal() -> Self {
        Self {
            ancestry: Ancestries::AwakenedAnimal,
            traits: vec![Traits::AwakenedAnimal, Traits::Humanoid, Traits::Beast],
            hp: 8, // todo 6 for tiny and small 8 for medium 10 for large
            rarity: Rarity::Rare,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 5.5, // todo unknown
            size: Sizes::Medium, // todo do tiny, small, medium, large
            speed: 25,  // determined by heritage
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Wisdom],
            ability_flaws: Some(vec![AbilityScores::Intelligence]),
            free_boosts: 1,
            languages: vec![Languages::Common],
            free_languages: 0, // why
            specials: vec![Specials::LowLightVision, Specials::AwakenedForm, Specials::AwakenedMind],
            common_languages: None,
        }
    }
    pub fn conrasu() -> Self {
        Self {
            ancestry: Ancestries::Conrasu,
            traits: vec![Traits::Conrasu, Traits::Plant, Traits::Aeon],
            hp: 10,
            rarity: Rarity::Rare,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 5.5, // todo unknown
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Charisma, AbilityScores::Wisdom],
            ability_flaws: Some(vec![AbilityScores::Charisma]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Rasu],
            free_languages: 0,
            specials: vec![Specials::SunlightHealing],
            common_languages: Some(vec![
                Languages::Empyrean,
                Languages::Elven,
                Languages::Iruxi,
                Languages::Fey,
                Languages::Petran,
                Languages::Utopian,
            ]),
        }
    }
    pub fn fleshwarp() -> Self {
        Self {
            ancestry: Ancestries::Fleshwarp,
            traits: vec![Traits::Fleshwarp, Traits::Humanoid, Traits::Aberration],
            hp: 10,
            rarity: Rarity::Rare,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 6.0, // todo 5 - 7
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Constitution],
            ability_flaws: None,
            free_boosts: 1,
            languages: vec![Languages::Common],
            free_languages: 0,
            specials: vec![Specials::Darkvision, Specials::UnusualAnatomy],
            common_languages: Some(vec![
                Languages::Aklo,
                Languages::Draconic,
                Languages::Dwarven,
                Languages::Goblin,
                Languages::Elven,
                Languages::Sakvroth,
            ]),
        }
    }
    fn ghoran() -> Self {
        Self {
            ancestry: Ancestries::Ghoran,
            traits: vec![Traits::Ghoran, Traits::Plant, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Rare,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 5.5, // todo unknown
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Constitution],
            ability_flaws: None,
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Fey],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::Photosynthesis],
            common_languages: Some(vec![
                Languages::Aklo,
                Languages::Draconic,
                Languages::Elven,
                Languages::Gnomish,
                Languages::Jotun,
            ]),
        }
    }
    fn goloma() -> Self {
        Self {
            ancestry: Ancestries::Goloma,
            traits: vec![Traits::Goloma, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Rare,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 5.5, // todo unknown
            size: Sizes::Medium,
            speed: 30,
            ability_boosts: vec![AbilityScores::Wisdom],
            ability_flaws: None,
            free_boosts: 1,
            languages: vec![Languages::Mwangi, Languages::Goloma],
            free_languages: 0,
            specials: vec![Specials::EyesInBack],
            common_languages: Some(vec![
                Languages::Chthonian,
                Languages::Aklo,
                Languages::Draconic,
                Languages::Elven,
                Languages::Halfling,
                Languages::Necril,
                Languages::Orcish,
                Languages::Fey,
            ]),
        }
    }
    fn kashrishi() -> Self {
        Self {
            ancestry: Ancestries::Kashrishi,
            traits: vec![Traits::Kashrishi, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Rare,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 5.5, // todo unknown
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Constitution],
            ability_flaws: None,
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Kashrishi],
            free_languages: 0,
            specials: vec![Specials::EmpathicSense, Specials::GlowingHorn],
            common_languages: Some(vec![
                Languages::Thalassic,
                Languages::Empyrean,
                Languages::Draconic,
                Languages::Fey,
                Languages::Petran,
            ]),
        }
    }
    fn poppet() -> Self {
        Self {
            ancestry: Ancestries::Poppet,
            traits: vec![Traits::Poppet, Traits::Humanoid, Traits::Construct],
            hp: 6,
            rarity: Rarity::Rare,
            adult_hood: 0.0, // todo unknown
            life_expectancy: 30,
            average_height: 2.0, //todo 1 - 3
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Charisma],
            ability_flaws: Some(vec![AbilityScores::Dexterity]),
            free_boosts: 1,
            languages: vec![Languages::Common],
            free_languages: 0,
            specials: vec![Specials::Darkvision, Specials::Constructed, Specials::Flammable],
            common_languages: Some(vec![
                Languages::Draconic,
                Languages::Dwarven,
                Languages::Elven,
                Languages::Gnomish,
                Languages::Goblin,
                Languages::Fey,
            ]),
        }
    }
    fn sarangay() -> Self {
        Self {
            ancestry: Ancestries::Sarangay,
            traits: vec![Traits::Sarangay, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Rare,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 6.5, // todo 6-7
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Charisma],
            ability_flaws: Some(vec![AbilityScores::Wisdom]),
            free_boosts: 1,
            languages: vec![Languages::Common],
            free_languages: 0,
            specials: vec![Specials::HeadGem, Specials::Horns],
            common_languages: Some(vec![
                Languages::Empyrean,
                Languages::Fey,
                Languages::Nagaji,
                Languages::Petran,
                Languages::Pyric,
                Languages::Sussuran,
                Languages::Thalassic,
                Languages::Yaksha,
            ]),
        }
    }
    fn shisk() -> Self {
        Self {
            ancestry: Ancestries::Shisk,
            traits: vec![Traits::Shisk, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Rare,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 5.5, // todo unknown
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Intelligence],
            ability_flaws: None,
            free_boosts: 1,
            languages: vec![Languages::Mwangi, Languages::Shisk],
            free_languages: 0,
            specials: vec![Specials::Darkvision],
            common_languages: None,
        }
    }
    fn shoony() -> Self {
        Self {
            ancestry: Ancestries::Shoony,
            traits: vec![Traits::Shoony, Traits::Humanoid],
            hp: 6,
            rarity: Rarity::Rare,
            adult_hood: 9.0, // todo unknown
            life_expectancy: 50, // todo unknown
            average_height: 3.5, // todo unknown
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Charisma],
            ability_flaws: Some(vec![AbilityScores::Constitution]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Shoony],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::BluntSnout],
            common_languages: Some(vec![
                Languages::Dwarven,
                Languages::Goblin,
                Languages::Gnomish,
                Languages::Halfling,
                Languages::Petran,
            ]),
        }
    }
    fn skeleton() -> Self {
        Self {
            ancestry: Ancestries::Skeleton,
            traits: vec![Traits::Skeleton, Traits::Undead],
            hp: 6,
            rarity: Rarity::Rare,
            adult_hood: 0.0, // todo unknown
            life_expectancy: -1, // todo unknown
            average_height: 5.5, // todo unknown
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Charisma],
            ability_flaws: Some(vec![AbilityScores::Intelligence]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Necril],
            free_languages: 0,
            specials: vec![Specials::Undeath],
            common_languages: Some(vec![
                Languages::Aklo,
                Languages::Dwarven,
                Languages::Elven,
                Languages::Diabolic,
                Languages::Orcish,
                Languages::Sakvroth,
            ]),
        }
    }
    fn sprite() -> Self {
        Self {
            ancestry: Ancestries::Sprite,
            traits: vec![Traits::Sprite, Traits::Fey],
            hp: 6,
            rarity: Rarity::Rare,
            adult_hood: 15.0,
            life_expectancy: 1000,
            average_height: 1.0,
            size: Sizes::Tiny,
            speed: 20,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Intelligence],
            ability_flaws: Some(vec![AbilityScores::Strength]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Fey],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::MagicStrikes],
            common_languages: Some(vec![
                Languages::Empyrean,
                Languages::Draconic,
                Languages::Elven,
                Languages::Gnomish,
                Languages::Goblin,
                Languages::Jotun,
            ]),
        }
    }
    fn strix() -> Self {
        Self {
            ancestry: Ancestries::Strix,
            traits: vec![Traits::Strix, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Rare,
            adult_hood: 14.0,
            life_expectancy: 40,
            average_height: 6.0,
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity],
            ability_flaws: None,
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Strix],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::Wings],
            common_languages: Some(vec![
                Languages::Draconic,
                Languages::Jotun,
                Languages::Gnomish,
                Languages::Diabolic,
            ]),
        }
    }
    fn surki() -> Self {
        Self {
            ancestry: Ancestries::Surki,
            traits: vec![Traits::Surki, Traits::Humanoid],
            hp: 6,
            rarity: Rarity::Rare,
            adult_hood: 50.0, // todo 10 - 100 years in eggs
            life_expectancy: 60, // todo unknown
            average_height: 5.5, // todo unknown
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Constitution],
            ability_flaws: None,
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Surki],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::Magiphage],
            common_languages: Some(vec![
                Languages::Elven,
                Languages::Fey,
                Languages::Sakvroth,
            ]),
        }
    }
    fn vishkanya() -> Self {
        Self {
            ancestry: Ancestries::Vishkanya,
            traits: vec![Traits::Vishkanya, Traits::Humanoid],
            hp: 6,
            rarity: Rarity::Rare,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 6.5, // old age 7 women taller than men
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity],
            ability_flaws: None,
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Vishkanya],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::InnateVenom],
            common_languages: Some(vec![
                Languages::Aklo,
                Languages::Thalassic,
                Languages::Draconic,
                Languages::Elven,
                Languages::Goblin,
                Languages::Sakvroth,
                Languages::Vanara,
            ]),
        }
    }
    fn yaksha() -> Self {
        Self {
            ancestry: Ancestries::Yaksha,
            traits: vec![Traits::Yaksha, Traits::Spirit],
            hp: 8,
            rarity: Rarity::Rare,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 100, // todo vows
            average_height: 5.0, // 4-7
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Charisma, AbilityScores::Constitution],
            ability_flaws: Some(vec![AbilityScores::Intelligence]),
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Yaksha, Languages::Fey],
            free_languages: 0,
            specials: vec![Specials::LowLightVision],
            common_languages: Some(vec![
                Languages::Empyrean,
                Languages::Diabolic,
                Languages::Nagaji,
                Languages::Tengu,
                Languages::Vudrani,
            ]),
        }
    }
    fn yaoguai() -> Self {
        Self {
            ancestry: Ancestries::Yaoguai,
            traits: vec![Traits::Yaoguai, Traits::Humanoid],
            hp: 8,
            rarity: Rarity::Rare,
            adult_hood: 15.0, // todo unknown
            life_expectancy: 90, // todo unknown
            average_height: 6.5, // todo unknown
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Charisma, AbilityScores::Constitution],
            ability_flaws: Some(vec![AbilityScores::Intelligence]),
            free_boosts: 1,
            languages: vec![Languages::Common],
            free_languages: 0,
            specials: vec![Specials::ChangeShape], // todo duplicate change shape with different effects
            common_languages: Some(vec![
                Languages::Aklo,
                Languages::Draconic,
                Languages::Elven,
                Languages::Fey,
                Languages::Kitsune,
                Languages::Nagaji,
                Languages::Sakvroth,
                Languages::Tengu,
                Languages::Ysoki,
            ]),
        }
    }

}

impl AncestryDescription {
    pub fn dwarf() -> Self {
        Self {
            description: "Orcs are proud, strong people with hardened physiques who value physical might and glory in combat.

Orcs are forged in the fires of violence and conflict, often from the moment they are born. As they live lives that are frequently cut brutally short, orcs revel in testing their strength against worthy foes, often by challenging a higher-ranking member of their community for dominance. Orcs often struggle to gain acceptance among other communities, who frequently see them as brutes. Those who earn the loyalty of an orc friend, however, soon learn that an orc's fidelity and honesty are unparalleled. Orc culture teaches that they are shaped by the challenges they survive, and the most worthy survive the most hardships. Orcs who attain both a long life and great triumphs command immense respect.",
                        you_might: vec!["Strive to uphold your personal honor and refuse to back down", "Appreciate quality craftsmanship in all forms and insist upon it for all your gear."],
                        others_probaly: vec!["See you as stubborn, though whether this is an asset or a detriment changes from moment to moment.", "Recognize the deep connection you have with your family, heritage, and friends."],
                        physical_description: "Dwarves are short and stocky, standing about a foot shorter than most humans. They have wide, compact bodies and burly frames. Dwarves of all genders pride themselves on the length of their hair and beards, which they often braid into intricate patterns, some of which represent specific clans. A long beard is a sign of maturity and honor among many dwarf clans.
            
Dwarves typically reach physical adulthood around the age of 25, though their traditionalist culture places more value on completing coming-of-age ceremonies unique to each clan than reaching a certain age. A typical dwarf can live to around 350 years old.",
                        society: "Though the ancient dwarven empire fell long ago, overwhelmed by orc and goblinoid enemies, dwarves today retain many of the qualities that once propelled them to greatness: fierceness, gumption, and stubbornness in their endeavors. Dwarves live within mountain Sky Citadels dotted over the surface, which can create vast cultural divides between dwarf clans. Yet nearly all dwarven peoples share a passion for stonework, metalwork, and family.
            
Few dwarves are seen without their clan dagger strapped to their belt. This dagger is forged just before a dwarf's birth and is distinctive to their clan. Dwarves honor their children with names of ancestors or dwarven heroes. When introducing themselves, dwarves tend to list their family and clan, plus any number of other familial connections and honorifics.",
                        beliefs: Some("Dwarves tend to value honor and closely follow the traditions of their clans and kingdoms. They have a strong sense of friendship and justice, though they are often very particular about who they consider a friend. They work hard and play harder—especially when strong ale is involved. Torag, god of dwarvenkind, is dwarves' primary deity, though worship of Torag's family members is also common.
            
Popular Edicts create art with utility, guard your community against those who would harm it, keep your clan dagger close
Popular Anathema leave an activity or promise uncompleted, forsake your family"),
        }
    }
    pub fn elf() -> Self {
        Self {
            description: "Elves are a tall, long-lived people with a strong tradition of art and magic.

As an ancient people, elves have seen great change and have the perspective that can come only from watching the arc of history. After leaving Golarion in ancient times, they returned to a changed land, and they still struggle to reclaim their ancestral homes. Elves value kindness, intellect, and beauty, with many elves striving to improve their manners, appearance, and culture. Their studies delve into a level of detail that most shorter-lived peoples find excessive or inefficient. Elves are often rather private people, steeped in the secrets of their groves and kinship groups. They're slow to build friendships outside their kinsfolk, as elves who spend their lives among shorter-lived peoples often become morose after watching generations of companions age and die. These elves are known as Forlorn among their fellow elves.",
                        you_might: vec!["Carefully curate your relationships with people with shorter lifespans.", "Adopt specialized or obscure interests simply for the sake of mastering them."],
                        others_probaly: vec!["Focus on your appearance, either admiring your grace or treating you as if you're physically fragile.", "Worry that you privately look down on them, or feel like you're condescending and aloof."],
                        physical_description: "While generally taller than humans, elves possess a fragile grace, accentuated by long features and sharply pointed ears. Their eyes are wide and rounded, featuring large and often vibrantly colored pupils that make up the entire visible portion of the eye. These pupils give them an alien look and allow them to see sharply even in very little light.
            
Elves gradually adapt to their environment and their companions, and they often take on physical traits reflecting their surroundings. An elf who has dwelled in primeval forests for centuries, for example, might exhibit verdant hair and gnarled fingers, while one who's lived in a desert might have golden pupils and skin. Elves reach physical adulthood around the age of 20, though they aren't considered to be fully emotionally mature by other elves until closer to the passing of their first century. A typical elf can live to around 600 years old.",
                        society: "he inborn patience and intellectual curiosity of elves make them excellent sages, philosophers, and wizards, and their societies are built upon their inherent sense of wonder and knowledge.
            
Elves hold deeply seated ideals of individualism, allowing each elf to explore multiple occupations before alighting on a particular pursuit or passion that suits her best. Elves bear notorious grudges against rivals, which elves call ‘ilduliel,' but these antagonistic relationships can sometimes blossom into friendships over time. An elf keeps their personal name secret among their family, while giving a nickname when meeting other people. This nickname can change over time, due to events in the elf's life or even on a whim. A single elf might be known by many names by associates of different ages and regions.
            
Elven names consist of multiple syllables and are meant to flow lyrically—at least in the Elven tongue.",
                        beliefs: Some("Elves are often emotional and capricious, yet they hold high ideals close to their hearts. They prefer deities who share their love of all things mystic and artistic. Desna and Shelyn are particular favorites, the former for her sense of wonder and the latter for her appreciation of artistry. Calistria is the most notorious of elven deities, as she represents many of the elven ideals taken to the extreme."),            
        }
    }
    pub fn gnome() -> Self {
        Self {
            description: "Gnomes are short and hardy folk, with an unquenchable curiosity and eccentric habits.

Long ago, early gnome ancestors emigrated from the First World, realm of the fey. While it's unclear why the first gnomes wandered to Golarion, this lineage manifests in modern gnomes as bizarre reasoning, eccentricity, obsessive tendencies, and what some see as naivete.
            
Always hungry for new experiences, gnomes constantly wander both mentally and physically, attempting to stave off a terrible ailment that threatens all of their people. This affliction, known as the Bleaching, strikes gnomes who fail to dream, innovate, and take in new experiences. The Bleaching slowly drains the color— literally—from gnomes, and it plunges those affected into states of deep depression that eventually claim their lives. Very few gnomes survive this scourge, becoming deeply morose and wise survivors known as bleachlings.",
                        you_might: vec!["Embrace learning and hop from one area of study to another without warning.", "Speak, think, and move quickly, and lose patience with those who can't keep up."],
                        others_probaly: vec!["Appreciate your enthusiasm and the energy with which you approach new situations.", "Struggle to understand your motivations or adapt to your rapid changes of direction."],
                        physical_description: "Most gnomes stand just over 3 feet in height and weigh little more than a human child. They exhibit a wide range of natural skin, hair, and eye colors. For gnomes that haven't begun the Bleaching, nearly any hair and eye color other than white is possible, with vibrant colors most frequent, while skin tones span a slightly narrower spectrum and tend toward earthy tones and pinkish hues, though occasionally green, black, or pale blue.
            
Gnomes typically reach physical maturity at the age of 18, though many gnomes maintain a childlike curiosity about the world even into adulthood. A gnome can theoretically live to any age if they can stave off the Bleaching indefinitely, but in practice gnomes rarely live longer than around 400 years.",
                        society: "While most gnomes adopt some of the cultural practices of the region in which they live, they tend to pick and choose, adjusting their communities to fit their own fey logic. This often leads to majority gnome communities eventually consisting almost entirely of gnomes, as other people, bewildered by gnomish political decisions, choose to move elsewhere. Gnomes have little culture that they would consider entirely their own. Exceptionally few gnome kingdoms or nations exist on the surface of Golarion, and most gnomes wouldn't know what to do with such a state if they had one.
            
By necessity, few gnomes marry for life, instead allowing relationships to run their course before amicably moving on, the better to stave off the Bleaching with new experiences. Though gnome families tend to be small, many gnome communities raise children communally, with fluid family boundaries. Gnome names can get quite complex and polysyllabic. Gnomes rarely concern themselves with how easy their names are to pronounce, and they often go by shorter nicknames. Among gnomes, the shorter the name, the more feminine it's considered to be.",
                        beliefs: Some("Though gnomes are impulsive tricksters with inscrutable motives and confusing methods, many at least attempt to make the world a better place. They are prone to fits of powerful emotion and are rarely shy about helping those they believe deserve it. Gnomes most commonly worship deities that value individuality and nature, such as Cayden Cailean, Desna, Gozreh, and Shelyn.",)
            
        }
    }
    pub fn goblin() -> Self {
        Self {
            description: "Goblins are a short, scrappy, energetic people who have spent millennia maligned and feared.

The convoluted histories other people cling to don't interest goblins. These small folk live in the moment, and they prefer tall tales over factual records. Goblin virtues are about being present, creative, and honest. They strive to lead fulfilled lives, rather than worrying about how their journeys will end. To tell stories, not nitpick the facts. To be small, but dream big. Many goblins enjoy simpler delights like songs, fire, and eating, and hate reading, dogs, and horses. Other goblins might have more complex pursuits, though, such as tinkering with scraps or concocting snacks and explosives from most anything.",
                        you_might: vec!["Strive to prove that you have a place among other civilized peoples, perhaps even to yourself.", "Lighten the heavy emotional burdens others carry (and amuse yourself) with antics and pranks."],
                        others_probaly: vec!["Work to ensure you don't accidentally (or intentionally) set too many things on fire.", "Wonder how you survive given your ancestry's typical gastronomic choices, reckless behavior, and love of fire."],
                        physical_description: "Goblins are stumpy humanoids with large bodies, scrawny limbs, and massively oversized heads with large ears and beady red eyes. Their skin ranges from green to gray to blue, and they often bear scars, boils, and rashes. Goblins average 3 feet tall. Most are bald, with little or no body hair. Their jagged teeth fall out and regrow constantly, and their fast metabolism means they eat constantly and nap frequently. Mutations are also more common among goblins than other peoples, and goblins usually view particularly salient mutations as a sign of power or fortune.
            
Goblins reach adolescence by the age of 3 and adulthood 4 or 5 years later. Goblins can live 50 years or more, but without anyone to protect them from each other or themselves, few live past 20 years of age.",
                        society: "Goblins tend to flock to strong leaders, forming small tribes. These tribes rarely number more than a hundred, though the larger a tribe is, the more diligent the leader must be to keep order—a notoriously difficult task. Play and creativity matter more to goblins than productivity or study, and their encampments erupt with songs and laughter.
            
Goblins bond closely with their allies, fiercely protecting those companions who have protected them or offered a sympathetic ear. Goblins tend to assume for their own protection that members of taller ancestries, which goblins often refer to colloquially as “longshanks,” won't treat them kindly. Learning to trust longshanks is difficult for a goblin, and it's been only in recent years that such a partnership has even been an option. Goblins keep their names simple. A good name should be easy to pronounce, short enough to shout without getting winded, and taste good to say. The namer often picks a word that rhymes with something they like so that writing songs is easier.",
                        beliefs: Some("Even the most well-intentioned goblins have trouble following the rules, meaning goblin adventurers are often unsure whether they're on the right side of the law. Organized worship likewise confounds goblins, and most of them would rather pick their own deities, choosing powerful monsters, natural wonders, or anything else they find fascinating— sometimes even attributing deific status to fellow goblins of note. Goblins who spend time around people of other ancestries might adopt some of their beliefs, though, and many goblin adventurers adopt the worship of Cayden Cailean."),
            
       }
    }
    pub fn halfling() -> Self {
        Self {
            description: "Halflings are a short, resilient people who exhibit remarkable curiosity and humor.

Claiming no place as their own, halflings control few settlements larger than villages. Instead, they frequently live among humans within larger cities, carving out small communities alongside taller folk. Optimistic, cheerful, and driven by powerful wanderlust, halflings make up for their short stature with an abundance of bravado. At once excitable and easygoing, halflings are the best kind of opportunists, and their passions favor joy over violence. While their curiosity sometimes drives them toward adventure, halflings also carry strong ties to house and home.",
                        you_might: vec!["Get along well with a wide variety of people and enjoy meeting new friends.", "Find it difficult to resist indulging your curiosity, even when you know it's going to lead to trouble."],
                        others_probaly: vec!["Appreciate your ability to always find a silver lining or something to laugh about, no matter how dire the situation.", "Think you bring good luck with you."],
                        physical_description: "Halflings are short humanoids who look vaguely like smaller humans. They rarely grow to be more than 3 feet in height. Halfling proportions vary, with some looking like shorter adult humans with slightly larger heads and others having proportions closer to those of a human child. Most halflings prefer to walk barefoot rather than wear shoes, and those who do so develop roughly calloused soles on their feet over time. Tufts of thick, often-curly hair warm the tops of their broad, tanned feet. Halfling skin tones tend toward rich, tawny shades like amber or oak, and their hair color ranges from a light golden blond to raven black.
            
Halflings reach physical adulthood around the age of 20. A typical halfling can live to be around 150 years old.",
                        society: "Despite their jovial and friendly nature, halflings don't usually tend to congregate. They have few cultural centers in the Inner Sea region, and they instead tend to weave themselves throughout the societies of the world. Halflings eke out whatever living they can manage, many performing menial labor or holding simple service jobs. Some halflings reject city life, instead turning to the open road and traveling from place to place in search of fortune and fame. These nomadic halflings often travel in small groups, sharing hardships and simple pleasures among close friends and family. Halfling names are usually two to three syllables, with a gentle sound that avoids hard consonants. Preferring their names to sound humble, halflings see overly long or complex names as a sign of arrogance for their people. However, they understand that elves and humans might have longer names to suit their own aesthetics.",
                        beliefs: Some("Halflings are loyal to their friends and their family, but they aren't afraid to do what needs to be done in order to survive. Wherever halflings go, they seamlessly blend into the society they find themselves in, adapting to the culture and beliefs of the predominant ancestry around them and adding their uniquely halfling twists, creating a blend of cultural diffusion that enriches both cultures. Halflings favor gods that either grant luck, like Desna, or encourage guile, like Norgorber, and many appreciate Cayden Cailean's role as a liberator, as well as any religions common among other ancestries around them."),
        }
    }
    pub fn human() -> Self {
        Self {
            description: "Humans are diverse and adaptable people with wide potential and deep ambitions.

As unpredictable and varied as any of Golarion's peoples, humans have exceptional drive and the capacity to endure and expand. Though many civilizations thrived before humanity rose to prominence, humans have built some of the greatest and the most terrible societies throughout the course of history, and today they are the most populous people in the realms around the Inner Sea.

Humans' ambition, versatility, and exceptional potential have led to their status as the world's predominant ancestry. Their empires and nations are vast, sprawling things, and their citizens carve names for themselves with the strength of their sword arms and the power of their spells. Humanity is diverse and tumultuous, running the gamut from nomadic to imperial, sinister to saintly. Many of them venture forth to explore, to map the expanse of the multiverse, to search for long-lost treasure, or to lead mighty armies to conquer their neighbors—for no better reason than because they can.",
            you_might: vec!["Strive to achieve greatness, either in your own right or on behalf of a cause.", "Seek to understand your purpose in the world.", "Cherish your relationships with family and friends."],
            others_probaly: vec!["Respect your flexibility, your adaptability, and—in most cases—your open-mindedness.", "Distrust your intentions, fearing you seek only power or wealth", "Aren't sure what to expect from you and are hesitant to assume your intentions."],
            physical_description: "Humans' physical characteristics are as varied as the world's climes. Humans have a wide variety of skin and hair colors, body types, and facial features. Generally speaking, their skin has a darker hue the closer to the equator they or their ancestors lived.

Humans reach physical adulthood around the age of 15, though mental maturity occurs a few years later. A typical human can live to be around 90 years old. Humans have exceptionally mutable physical characteristics compared to other ancestries, with greater variance in height, weight, and other physical parameters.",
            society: "Human variety also manifests in terms of their governments, attitudes, and social norms. Though the oldest of human cultures can trace their shared histories thousands of years into the past, when compared to the societies of the elves or dwarves, human civilizations seem in a state of constant flux as empires fragment and new kingdoms subsume the old.",
            beliefs: None,
        }
    }
    pub fn leshy() -> Self {
        Self {
            description: "Leshies are immortal nature spirits placed in small plant bodies, seeking to experience the world.

Leshies are immortal spirits of nature temporarily granted physical forms. As guardians and emissaries of the environment, leshies are “born” when a skilled druid or other master of primal magic conducts a ritual to create a suitable vessel, and then a spirit chooses that vessel to be their temporary home. Leshies are self-sufficient from the moment the ritual ends, though it isn't uncommon for leshies to maintain lifelong bonds with their creators. Many leshies relish the opportunity to interact with the physical world. While most leshy spirits are ancient, they rarely recall past lifetimes and see their new life as a chance to experience the wonders of the world once more.",
                        you_might: vec!["Act as a traveling agent for natural guardians who are unable to leave their territories.", "Encourage civilizations to cooperate with nature and build their cities in ecologically friendly ways."],
                        others_probaly: vec!["Think you are a curiosity due to your spiritual origins.", "Assume you know only about nature and are unfamiliar with civilization and society."],
                        physical_description: "Leshies are as varied as the material used to create their vessels, usually appearing as a bizarre mishmash of various plants or fungi. Their bodies are vaguely humanoid in shape, with numerous characteristics of the plant or fungus from which they were made. A typical leshy is about 3 feet tall. Leshies begin their lives as adults and don't age.",
                        society: "To most leshies, the concept of family is not a matter of birth, but rather determined by bonds of loyalty and friendship. Leshies are dedicated allies, but they have little tolerance for those who would despoil nature. As much as they are happy to accept someone who earns their trust into their family, they expect family members to look out for them and their natural wards in return.
Leshies are grouped into categories akin to ethnicities, but these are not connected to physical features; rather, they represent broad categories of characteristics of their spirits. Certain spirits are more likely to gravitate toward particular physical bodies, though this predisposition is far from absolute. Leshies' genders are determined by the spirits that inhabit their bodies. Some leshies are exclusively male or female, while many consider themselves both. Others, particularly fungus leshies, tend toward far more complex expressions of gender, or eschew the concept entirely. Leshies choose and change their names multiple times throughout their lives.",
                        beliefs: Some("Leshies' beliefs generally focus on the natural world. Those with a philosophical bent lean toward the Green Faith, and Gozreh is the most popular deity among faithful leshies. Some leshies also venerate green men, powerful spirits of nature."),
        }
    }    
    pub fn orc() -> Self {
        Self {
            description: "Orcs are proud, strong people with hardened physiques who value physical might and glory in combat.

Orcs are forged in the fires of violence and conflict, often from the moment they are born. As they live lives that are frequently cut brutally short, orcs revel in testing their strength against worthy foes, often by challenging a higher-ranking member of their community for dominance. Orcs often struggle to gain acceptance among other communities, who frequently see them as brutes. Those who earn the loyalty of an orc friend, however, soon learn that an orc's fidelity and honesty are unparalleled. Orc culture teaches that they are shaped by the challenges they survive, and the most worthy survive the most hardships. Orcs who attain both a long life and great triumphs command immense respect.",
            you_might: vec!["Eagerly meet any chance to prove your strength in a physical contest.", "View dying in glorious combat as preferable to a mundane death from old age or illness."],
            others_probaly: vec!["See you as violent or lacking in discipline.", "Admire your forthrightness and blunt honesty."],
            physical_description: "Orcs are tall and powerfully built, with long arms and stocky legs. Many orcs top 7 feet in height, though they tend to adopt broad, almost bow-legged stances and slouch forward at the shoulders. Orcs have rough skin, thick bones, and rock-hard muscles, making them suited to war and other physically demanding tasks. Orc skin color is typically some shade of green, though some orcs have other skin colors that reflect adaptations to their environments.

Orcs consider powerful builds, heavily scarred skin, large tusks, and tattoos attractive, regardless of gender. Orcs reach physical adulthood around the age of 17, with many orcs living to be up to 60 years old.",
            society: "Most orc communities—known as holds—define themselves through two things: pain and glory. Each earns respect in near equal measure, so long as the pain is borne with stoicism. An orc with many scars who walks uncomplaining with a broken leg draws as much admiration as one who wins a great victory on the battlefield. Additionally, power defines the dynamics among families and holds. Weaker orcs work at the behest of the strong, with power constantly shifting between orcs that prove their might. Orcs tend to share in familial duties, raising children as a community and sharing responsibilities among the entire hold.",
            beliefs: Some("A common orc saying is “you are the scars that shape you.” Violent, chaotic lives in violent, chaotic lands mean that most orcs tend to expect and accept violence. Lamashtu, and Rovagug are commonly worshiped among more war-minded orc communities, while less violent holds worship gods like Sarenrae, whose tenets of fire, redemption, and glory all hold some appeal to orc sensibilities.

While there are orc deities, their worship is surprisingly uncommon among orcs. Orcs believe that if a creature has a face and a name, it can be killed, and so their own deities are often targets, rather than objects of reverence. Some orc holds teach that the greatest members of the hold can earn a chance to challenge the orc deities for a place amid the pantheon."),
        }
    }
    pub fn athamaru() -> Self {
        Self {
            description: "Athamarus are fish-like humanoids who form tight-knit communities underseas, with villages of siblings led by a common matriarch. Athamarus engage in subsistence farming of seaweed, train eels to serve as mounts, and create elaborate works of coral art.

Athamarus are fish-like humanoids who form tight-knit undersea communities. In small settlements, they engage in the subsistence farming of seaweed, train eels to serve as mounts, and create elaborate works of coral art. Their interactions with other aquatic ancestries are strained, as athamarus have suffered mistreatment at their hands. However, they remain curious about potential connections and what new opportunities may offer.
            
Athamarus define themselves by their connections to the world around them. They live in communal settings among coral reefs and aquatic companion creatures that encourage large families and friend groups. Athamaru settlements established near river or sea trade routes usually maintain strong relationships with the sailors who regularly pass by, exchanging coral jewelry or information about nearby sea routes for surface goods, especially root foods like tubers, yams, and other vegetables, which they see as delicacies.
            
If you want to play a character who values community and survives the sea through effort alone, you should play an athamaru.",
                        you_might: vec!["Value your community above most everything else.", "Approach strangers with warm curiosity."],
                        others_probaly: vec!["Assume you're well versed in underwater survival.", "Believe you're to blame for foul smells."],
                        physical_description: "Athamarus have a distinctive appearance that resembles fish. Their brightly colored skin often matches the reefs where they build their communities. Frills, barbels, and crested fins add to their flamboyant appearance. The sheer variety of crest shapes, scale patterns, and fin styles make individuals distinct, even as communities share features. These similarities are often environmentally advantageous, such as scales that allow them to blend into seaweed or longer toes in areas with stony seabeds. Outsiders often note that athamarus have a distinctive smell, which comes from pheromones, used for both communication and defense. While the level of control varies, all athamarus can communicate basic emotions chemically, and individual settlements have unique variations to their pheromones that serve as a community fingerprint. Masters of pheromonal expression can communicate complex philosophical concepts purely chemically, occupying a role similar to master singers in other communities.
            
Athamarus value natural adornments that blur the lines between body modifications and jewelry. In areas where coral grows, tending reefs and their symbiotic species are highly valued tasks. Some fashion still-living specimens into earrings or cuffs, then gently guide the coral as it continues to grow. These pieces often stay in circulation for generations and are valued as living community history. Other uses of coral include integrating pieces in with the wearer's body in symbiosis, the coral providing nutrients and the athamaru ensuring access to sunlight and quality water.",
                        society: "The largest population of athamarus lives in the underwater nation of Xidao in Tian Xia. There, smaller city-states follow their own government and leaders, though they have vowed to protect one another from the terrors of the deep should the need arise. The largest city-state, Yashabaru, has no direct control over the other settlements, but its leader serves as a figurehead for the region. Outside of Xidao, communities are more isolated and independent, though population centers include the Shackles, a reclusive settlement near Sedeq, and the large gulfs of Arcadia.
            
The smallest individual athamaru settlements might have only a dozen members, while the largest house hundreds. Athamaru communities connect with other aquatic inhabitants to build strong bonds that improve the quality of life for everyone involved. Animals are integrated into their communities as partners, particularly the domesticated eels that athamarus use as steeds. The eels generally roam freely, and athamarus train mounts from newly hatched elvers they raise.
            
Each community consists of genetically related athamarus, typically all born from a single parent. In most settlements, the parent is known as the matriarch and serves as the leader. When this parent nears the end of their life or otherwise stops laying eggs, several athamarus in the community choose to undergo certain physical changes, including an increase in size, to announce their intention to become the next matriarch. The decision to be considered a matriarch isn't taken lightly, as the eventual leadership role is responsible for the community's survival and continuation. Each community selects their next matriarch from these “hopefuls” in their own way. Such traditions include democratic elections, trials of knowledge, and contests of physicality involving deep diving or riding untamed wildlife. In Xidao, citystate matriarch candidates must present themselves to the High Matriarch and their council to be judged worthy of completing the transformation.
            
Sometimes young adults split off to create new communities, especially if resources become scarce or internal conflict escalates. If a group struggles as they establish a community, their former settlement often agrees to take them back if they agree to respect the extant leadership.
            
Other groups have mistreated athamarus in the past, so their communities maintain strong internal bonds. Athamarus consider it less risky to deal with each other first. While the most common activities between them are trade, communities also share information. Warnings about danger flow quickly. Safe opportunities are also shared, such as trustworthy land-dwellers and open markets. Such trust with outsiders, however, is easily lost and hard to regain. Most athamarus would rather flee—sometimes even uprooting an entire community—than risk open hostilities.",
                        beliefs: Some("Due to their community-minded nature, athamarus tend to favor beliefs and philosophies that focus on connection and aiding others. They are also aware that they sometimes require protection from the dangers of the sea, so some athamarus take on the roles of staunch guardians or religious figures who plead for intercession from sympathetic gods. Worship within a single community tends to favor one deity, with multiple temples being rare. In the Inner Sea, athamaru settlements typically revere the nature god Gozreh to strengthen their bonds with their surroundings or Erastil for guidance on forming healthy communities. In Tian Xia, the Duke of Thunder Hei Feng's favor is sought as athamarus navigate fickle seasons and politics, while those athamarus who guard the seas against horrors from the deep pray to the triad war gods Srikalis, Sritaming, and Sribaril for unwavering strength."),
            
        }
    }
}
