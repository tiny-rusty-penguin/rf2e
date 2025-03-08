use clap::Parser;
use std::io;
use strum::{EnumIter, IntoEnumIterator};

struct Size {
    space: u8,
    reach_tall: u8,
    reach_long: u8,
}

impl Sizes {
    fn get_size_data(self) -> Size {
        match self {
            Sizes::Tiny => Size {
                space: 2,
                reach_tall: 0,
                reach_long: 0,
            },
            Sizes::Small => Size {
                space: 5,
                reach_tall: 5,
                reach_long: 5,
            },
            Sizes::Medium => Size {
                space: 5,
                reach_tall: 5,
                reach_long: 5,
            },
            Sizes::Large => Size {
                space: 10,
                reach_tall: 10,
                reach_long: 5,
            },
            Sizes::Huge => Size {
                space: 15,
                reach_tall: 15,
                reach_long: 10,
            },
            Sizes::Gargantuan => Size {
                space: 20,
                reach_tall: 20,
                reach_long: 15,
            },
        }
    }
}

#[derive(Debug)]
enum Sizes {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

#[derive(Debug, EnumIter)]
enum Languages {
    Common,
    Dwarven,
    Elven,
    Fey,
    Gnomish,
    Goblin,
    Halfling,
    Orcish,
}

#[derive(Debug, EnumIter, PartialEq, Clone)]
enum AbilityScores {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl AbilityScores {
    fn all() -> [AbilityScores; 6] {
        [
            AbilityScores::Strength,
            AbilityScores::Dexterity,
            AbilityScores::Constitution,
            AbilityScores::Intelligence,
            AbilityScores::Wisdom,
            AbilityScores::Charisma,
        ]
    }
}

#[derive(Debug, EnumIter)]
enum Ancestries {
    Dwarf,
    Elf,
    Gnome,
    Goblin,
    Halfling,
    Human,
    Leshy,
    Orc,
}

#[derive(Debug)]
enum Specials {
    Darkvision,
    ClanDagger,
    LowLightVision,
    KeenEyes,
    PlantNourishment,
}

struct Ancestry {
    ancestry: Ancestries,
    hp: u8,
    size: Sizes,
    speed: u8,
    ability_boosts: Vec<AbilityScores>,
    ability_flaws: Vec<AbilityScores>,
    free_boosts: u8,
    languages: Vec<Languages>,
    free_languages: u8,
    specials: Vec<Specials>,
}

impl Ancestry {
    fn dwarf() -> Self {
        Ancestry {
            ancestry: Ancestries::Dwarf,
            hp: 10,
            size: Sizes::Small,
            speed: 20,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Wisdom],
            ability_flaws: vec![AbilityScores::Charisma],
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Dwarven],
            free_languages: 0,
            specials: vec![Specials::Darkvision, Specials::ClanDagger],
        }
    }
    fn elf() -> Self {
        Ancestry {
            ancestry: Ancestries::Elf,
            hp: 6,
            size: Sizes::Medium,
            speed: 30,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Intelligence],
            ability_flaws: vec![AbilityScores::Constitution],
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Elven],
            free_languages: 0,
            specials: vec![Specials::LowLightVision],
        }
    }

    fn gnome() -> Self {
        Ancestry {
            ancestry: Ancestries::Gnome,
            hp: 8,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Charisma],
            ability_flaws: vec![AbilityScores::Strength],
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Fey, Languages::Gnomish],
            free_languages: 0,
            specials: vec![Specials::LowLightVision],
        }
    }

    fn goblin() -> Self {
        Ancestry {
            ancestry: Ancestries::Goblin,
            hp: 6,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Charisma],
            ability_flaws: vec![AbilityScores::Wisdom],
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Goblin],
            free_languages: 0,
            specials: vec![Specials::Darkvision],
        }
    }

    fn halfling() -> Self {
        Ancestry {
            ancestry: Ancestries::Halfling,
            hp: 6,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Wisdom],
            ability_flaws: vec![AbilityScores::Strength],
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Halfling],
            free_languages: 0,
            specials: vec![Specials::KeenEyes],
        }
    }

    fn human() -> Self {
        Ancestry {
            ancestry: Ancestries::Human,
            hp: 8,
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![],
            ability_flaws: vec![],
            free_boosts: 2,
            languages: vec![Languages::Common],
            free_languages: 1,
            specials: vec![],
        }
    }

    fn leshy() -> Self {
        Ancestry {
            ancestry: Ancestries::Leshy,
            hp: 8,
            size: Sizes::Small,
            speed: 25,
            ability_boosts: vec![AbilityScores::Wisdom, AbilityScores::Constitution],
            ability_flaws: vec![AbilityScores::Intelligence],
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Fey],
            free_languages: 0,
            specials: vec![Specials::LowLightVision, Specials::PlantNourishment],
        }
    }

    fn orc() -> Self {
        Ancestry {
            ancestry: Ancestries::Orc,
            hp: 10,
            size: Sizes::Medium,
            speed: 25,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Constitution],
            ability_flaws: vec![AbilityScores::Intelligence],
            free_boosts: 1,
            languages: vec![Languages::Common, Languages::Orcish],
            free_languages: 0,
            specials: vec![Specials::Darkvision],
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    create_character: bool,
}

fn get_input(prompt: &str) -> anyhow::Result<String> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase())
}

fn print_separator() {
    println!("--------------------")
}

fn select_ancestary() -> anyhow::Result<Ancestry> {
    loop {
        print_separator();
        for ancestry in Ancestries::iter() {
            println!("{:?}", ancestry);
        }
        let ancestary = get_input("Please Select an Ancestry:")?;
        match ancestary.as_str() {
            "dwarf" => return Ok(Ancestry::dwarf()),
            "elf" => return Ok(Ancestry::elf()),
            "gnome" => return Ok(Ancestry::gnome()),
            "goblin" => return Ok(Ancestry::goblin()),
            "halfling" => return Ok(Ancestry::halfling()),
            "human" => return Ok(Ancestry::human()),
            "leshy" => return Ok(Ancestry::leshy()),
            "orc" => return Ok(Ancestry::orc()),
            _ => {
                println!("Invalid ancestry. Please try again.");
            }
        }
    }
}

fn select_free_boosts(mut ancestary: Ancestry, num_of_boosts: u8) -> anyhow::Result<Ancestry> {
    print_separator();
    let all_abilities = AbilityScores::all();
    let available_boosts: Vec<AbilityScores> = all_abilities
        .iter()
        .filter(|ability| {
            !ancestary.ability_boosts.contains(ability)
                && !ancestary.ability_flaws.contains(ability)
        })
        .cloned()
        .collect();
    for boost in available_boosts {
        println!("{:?}", boost);
    }
    loop {
        let new_boost = get_input(
            format!(
                "Please Select an Ability Boost ({} boosts avaliable)",
                num_of_boosts
            )
            .as_str(),
        )?;
        match new_boost.as_str() {
            "strength" | "str" => {
                ancestary.ability_boosts.push(AbilityScores::Strength);
                return Ok(ancestary);
            }
            "dexterity" | "dex" => {
                ancestary.ability_boosts.push(AbilityScores::Dexterity);
                return Ok(ancestary);
            }
            "constitution" | "con" => {
                ancestary.ability_boosts.push(AbilityScores::Constitution);
                return Ok(ancestary);
            }
            "intelligence" | "int" => {
                ancestary.ability_boosts.push(AbilityScores::Intelligence);
                return Ok(ancestary);
            }
            "wisdom" | "wis" => {
                ancestary.ability_boosts.push(AbilityScores::Wisdom);
                return Ok(ancestary);
            }
            "charisma" | "cha" => {
                ancestary.ability_boosts.push(AbilityScores::Charisma);
                return Ok(ancestary);
            }
            _ => {
                println!("Invalid Boost. Please try again.");
            }
        }
    }
    Ok(ancestary)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.create_character {
        let mut ancestary = select_ancestary()?;
        let num_of_boosts = ancestary.free_boosts;
        for i in 0..num_of_boosts {
            ancestary = select_free_boosts(ancestary, num_of_boosts - i)?;
        }
    }
    Ok(())
}
