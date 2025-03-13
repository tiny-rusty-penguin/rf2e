use strum::EnumIter;
use crate::{AbilityScores, LoreSkills, Skills, SkillFeats, Terrain};


#[derive(Debug, EnumIter)]
enum Backgrounds {
    Acolyte,
    Acrobat,
    AnimalWhisperer,
    Artisan,
    Artist,
    Bandit,
    Barkeep,
    Barrister,
    BountyHunter,
    Charlatan,
    Cook,
    Criminal,
    Cultist,
    Detective,
    Emissary,
    Entertainer,
    Farmhand,
    FieldMedic,
    FortuneTeller,
    Gambler,
    Gladiator,
    Guard,
    Herbalist,
    Hermit,
    Hunter,
    Laborer,
    MartialDisciple,
    Merchant,
    Miner,
    Noble,
    Nomad,
    Prisoner,
    Sailor,
    Scholar,
    Scout,
    StreetUrchin,
    Teacher,
    Tinker,
    Warrior,
}

struct Background {
    background: Backgrounds,
    ability_boosts: Vec<AbilityScores>,
    trained_abilities: Vec<Skills>,
    trained_lore: Vec<LoreSkills>,
    description: &'static str,
    skill_feat: SkillFeats,
}

impl Background {
    pub fn all() -> Vec<Self> {
        vec![
            Self::acolyte(),
            Self::acrobat(),
            Self::animal_whisperer(),
            Self::artisan(),
            Self::artist(),
            Self::bandit(),
            Self::barkeep(),
            Self::barrister(),
            Self::bounty_hunter(),
            Self::charlatan(),
            Self::cook(),
            Self::criminal(),
            Self::cultist(),
            Self::detective(),
            Self::emissary(),
            Self::entertainer(),
            Self::farmhand(),
            Self::field_medic(),
            Self::fortune_teller(),
            Self::gambler(),
            Self::gladiator(),
            Self::guard(),
            Self::herbalist(),
            Self::hermit(),
            Self::hunter(),
            Self::laborer(),
            Self::martial_disciple(),
            Self::merchant(),
            Self::miner(),
            Self::noble(),
            Self::nomad(),
            Self::prisoner(),
            Self::sailor(),
            Self::scholar(),
            Self::scout(),
            Self::street_urchin(),
            Self::teacher(),
            Self::tinker(),
            Self::warrior(),
        ]
    }

    fn acolyte() -> Self {
        Self {
            background: Backgrounds::Acolyte,
            ability_boosts: vec![AbilityScores::Intelligence, AbilityScores::Wisdom],
            trained_abilities: vec![Skills::Religion],
            trained_lore: vec![LoreSkills::Scribing],
            description: "You spent your early days in a religious monastery or cloister. You may have traveled out into the world to spread the message of your religion or because you cast away the teachings of your faith, but deep down, you’ll always carry within you the lessons you learned",
            skill_feat: SkillFeats::StudentOfTheCanon,
        }
    }
    fn acrobat() -> Self {
        Self {
            background: Backgrounds::Acrobat,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Dexterity],
            trained_abilities: vec![Skills::Acrobatics],
            trained_lore: vec![LoreSkills::Circus],
            description: "In a circus or on the streets, you earned your pay by performing as an acrobat. You might have turned to adventuring when the money dried up, or simply decided to put your skills to better use.",
            skill_feat: SkillFeats::SteadyBalance,
        }
    }
    fn animal_whisperer() -> Self {
        Self {
            background: Backgrounds::AnimalWhisperer,
            ability_boosts: vec![AbilityScores::Wisdom, AbilityScores::Charisma],
            trained_abilities: vec![Skills::Nature],
            trained_lore: vec![LoreSkills::Terrain(None)],
            description: "You have always felt a connection to animals, and it was only a small leap to learn to train them. As you travel, you continuously encounter different creatures, befriending them along the way.",
            skill_feat: SkillFeats::TrainAnimal,
        }
    }
    fn artisan() -> Self {
        Self {
            background: Backgrounds::Artisan,
            ability_boosts: vec![AbilityScores::Intelligence, AbilityScores::Strength],
            trained_abilities: vec![Skills::Crafting],
            trained_lore: vec![LoreSkills::Guild],
            description: "As an apprentice, you practiced a particular form of building or crafting, developing a specialized skill. You might have been a blacksmith’s apprentice toiling over the forge for countless hours, a young tailor sewing garments of all kinds, or a shipwright shaping the hulls of ships.",
            skill_feat: SkillFeats::SpecialtyCrafting,
        }
    }
    fn artist() -> Self {
        Self {
            background: Backgrounds::Artist,
            ability_boosts: vec![AbilityScores::Charisma, AbilityScores::Dexterity],
            trained_abilities: vec![Skills::Crafting],
            trained_lore: vec![LoreSkills::Art],
            description: "Your art is your greatest passion, whatever form it takes. Adventuring might help you find inspiration, or simply be a way to survive until you become a world-famous artist.",
            skill_feat: SkillFeats::SpecialtyCrafting,
        }
    }
    fn bandit() -> Self {
        Self {
            background: Backgrounds::Bandit,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Charisma],
            trained_abilities: vec![Skills::Intimidation],
            trained_lore: vec![LoreSkills::Terrain(None)],
            description: "Your past includes no small amount of rural banditry, robbing travelers on the road and scraping by. Whether your robbery was sanctioned by a local noble or you did so of your own accord, you eventually got caught up in the adventuring life.",
            skill_feat: SkillFeats::GroupCoercion,
        }
    }
    fn barkeep() -> Self {
        Self {
            background: Backgrounds::Barkeep,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Charisma],
            trained_abilities: vec![Skills::Diplomacy],
            trained_lore: vec![LoreSkills::Alcohol],
            description: "You have five specialties: hefting barrels, drinking, polishing steins, drinking, and drinking. You worked in a bar, where you learned how to hold your liquor and rowdily socialize.",
            skill_feat: SkillFeats::Hobnobber,
        }
    }
    fn barrister() -> Self {
        Self {
            background: Backgrounds::Barrister,
            ability_boosts: vec![AbilityScores::Intelligence, AbilityScores::Charisma],
            trained_abilities: vec![Skills::Diplomacy],
            trained_lore: vec![LoreSkills::Legal],
            description: "Piles of legal manuals, stern teachers, and experience in the courtroom have instructed you in legal matters. You’re capable of mounting a prosecution or defense in court, and you tend to keep abreast of local laws, as you never know when you might need to know them on short notice.",
            skill_feat: SkillFeats::GroupImpression,
        }
    }
    fn bounty_hunter() -> Self {
        Self {
            background: Backgrounds::BountyHunter,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Wisdom],
            trained_abilities: vec![Skills::Survival],
            trained_lore: vec![LoreSkills::Legal],
            description: "Bringing in lawbreakers lined your pockets. Maybe you had an altruistic motive and sought to bring in criminals to make the streets safer, or maybe the coin was motivation enough. Your techniques for hunting down criminals transfer easily to the life of an adventurer.",
            skill_feat: SkillFeats::ExperiencedTracker,
        }
    }
    fn charlatan() -> Self {
        Self {
            background: Backgrounds::Charlatan,
            ability_boosts: vec![AbilityScores::Intelligence, AbilityScores::Charisma],
            trained_abilities: vec![Skills::Deception],
            trained_lore: vec![LoreSkills::Underworld],
            description: "You traveled from place to place, peddling false fortunes and snake oil in one town, while pretending to be royalty in exile to seduce a wealthy heir in the next. Becoming an adventurer might be your next big scam or an attempt to put your talents to use for a greater cause. Perhaps it’s a bit of both, as you realize that after pretending to be a hero, you’ve become the mask.",
            skill_feat: SkillFeats::CharmingLiar,
        }
    }
    fn cook() -> Self {
        Self {
            background: Backgrounds::Cook,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Intelligence],
            trained_abilities: vec![Skills::Survival],
            trained_lore: vec![LoreSkills::Cooking],
            description: "You grew up in the kitchens of a tavern or other dining establishment and excelled there, becoming an exceptional cook. Baking, cooking, a little brewing on the side—you’ve spent lots of time out of sight. It’s about time you went out into the world to catch some sights for yourself.",
            skill_feat: SkillFeats::Seasoned,
        }
    }
    fn criminal() -> Self {
        Self {
            background: Backgrounds::Criminal,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Intelligence],
            trained_abilities: vec![Skills::Stealth],
            trained_lore: vec![LoreSkills::Underworld],
            description: "As an unscrupulous independent or as a member of an underworld organization, you lived a life of crime. You might have become an adventurer to seek redemption, to escape the law, or simply to get access to bigger and better loot.",
            skill_feat: SkillFeats::ExperiencedSmuggler,
        }
    }
    fn cultist() -> Self {
        Self {
            background: Backgrounds::Cultist,
            ability_boosts: vec![AbilityScores::Intelligence, AbilityScores::Charisma],
            trained_abilities: vec![Skills::Occultism],
            trained_lore: vec![LoreSkills::Deity(None)],
            description: "You were (or still are) a member of a cult whose rites may involve sacred dances to ensure a strong harvest or dire rituals that call upon dark powers. You might have taken up adventuring to further your cult’s aims, to initiate yourself into the world’s grander mysteries, or to flee unsavory practices or strictures.",
            skill_feat: SkillFeats::SchooledInSecrets,
        }
    }
    fn detective() -> Self {
        Self {
            background: Backgrounds::Detective,
            ability_boosts: vec![AbilityScores::Intelligence, AbilityScores::Wisdom],
            trained_abilities: vec![Skills::Society],
            trained_lore: vec![LoreSkills::Underworld],
            description: "You solved crimes as a police inspector or took jobs for wealthy clients as a private investigator. You might have become an adventurer as part of your next big mystery, but likely it was due to the consequences or aftermath of a prior case.",
            skill_feat: SkillFeats::Streetwise,
        }
    }
    fn emissary() -> Self {
        Self {
            background: Backgrounds::Emissary,
            ability_boosts: vec![AbilityScores::Charisma, AbilityScores::Intelligence],
            trained_abilities: vec![Skills::Society],
            trained_lore: vec![LoreSkills::City(None)],
            description: "As a diplomat or messenger, you traveled to lands far and wide. Communicating with new people and forming alliances were your stock and trade.",
            skill_feat: SkillFeats::Multilingual,
        }
    }
    fn entertainer() -> Self {
        Self {
            background: Backgrounds::Entertainer,
            ability_boosts: vec![AbilityScores::Charisma, AbilityScores::Dexterity],
            trained_abilities: vec![Skills::Performance],
            trained_lore: vec![LoreSkills::Theater],
            description: "Through an education in the arts or sheer dogged practice, you learned to entertain crowds. You might have been an actor, a dancer, a musician, a street magician, or any other sort of performer.",
            skill_feat: SkillFeats::FascinatingPerformance,
        }
    }
    fn farmhand() -> Self {
        Self {
            background: Backgrounds::Farmhand,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Wisdom],
            trained_abilities: vec![Skills::Athletics],
            trained_lore: vec![LoreSkills::Farming],
            description: "With a strong back and an understanding of seasonal cycles, you tilled the land and tended crops. Your farm could have been razed by invaders, you could have lost the family tying you to the land, or you might have simply tired of the drudgery, but at some point, you became an adventurer.",
            skill_feat: SkillFeats::Assurance(Some(Skills::Athletics)),
        }
    }
    fn field_medic() -> Self {
        Self {
            background: Backgrounds::FieldMedic,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Wisdom],
            trained_abilities: vec![Skills::Medicine],
            trained_lore: vec![LoreSkills::Warfare],
            description: "In the chaotic rush of battle, you learned to adapt to rapidly changing conditions as you administered to battle casualties. You patched up soldiers, guards, or other combatants, and learned a fair amount about the logistics of war.",
            skill_feat: SkillFeats::BattleMedicine,
        }
    }
    fn fortune_teller() -> Self {
        Self {
            background: Backgrounds::FortuneTeller,
            ability_boosts: vec![AbilityScores::Intelligence, AbilityScores::Charisma],
            trained_abilities: vec![Skills::Occultism],
            trained_lore: vec![LoreSkills::FortuneTelling],
            description: "The strands of fate are clear to you, as you have learned many traditional forms by which laypeople can divine the future. You might have used these skills to guide your community, or simply to make money. But even the slightest peek into these practices connects you to the occult mysteries of the universe.",
            skill_feat: SkillFeats::OddityIdentification,
        }
    }
    fn gambler() -> Self {
        Self {
            background: Backgrounds::Gambler,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Charisma],
            trained_abilities: vec![Skills::Deception],
            trained_lore: vec![LoreSkills::Games],
            description: "The thrill of the win drew you into games of chance. This might have been a lucrative sideline that paled in comparison to the real risks of adventuring, or you might have fallen on hard times due to your gambling and pursued adventuring as a way out of a spiral.",
            skill_feat: SkillFeats::LieToMe,
        }
    }
    fn gladiator() -> Self {
        Self {
            background: Backgrounds::Gladiator,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Charisma],
            trained_abilities: vec![Skills::Performance],
            trained_lore: vec![LoreSkills::Gladiatorial],
            description: "The bloody games of the arena taught you the art of combat. Before you attained true fame, you departed—or escaped—the arena to explore the world. Your skill at drawing both blood and a crowd’s attention pay off in a new adventuring life.",
            skill_feat: SkillFeats::ImpressivePerformance,
        }
    }
    fn guard() -> Self {
        Self {
            background: Backgrounds::Guard,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Charisma],
            trained_abilities: vec![Skills::Intimidation],
            trained_lore: vec![LoreSkills::Legal, LoreSkills::Warfare],
            description: "You served in the guard, out of either patriotism or the need for coin. Either way, you know how to get a difficult suspect to talk. However you left the guard, you might think of adventuring as a way to use your skills on a wider stage.",
            skill_feat: SkillFeats::QuickCoercion,
        }
    }
    fn herbalist() -> Self {
        Self {
            background: Backgrounds::Herbalist,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Wisdom],
            trained_abilities: vec![Skills::Nature],
            trained_lore: vec![LoreSkills::Herbalism],
            description: "As a formally trained apothecary or a rural practitioner of folk medicine, you learned the healing properties of various herbs. You’re adept at collecting the right natural cures in all sorts of environments and preparing them properly.",
            skill_feat: SkillFeats::NaturalMedicine,
        }
    }
    fn hermit() -> Self {
        Self {
            background: Backgrounds::Hermit,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Intelligence],
            trained_abilities: vec![Skills::Nature, Skills::Occultism],
            trained_lore: vec![LoreSkills::Terrain(None)],
            description: "In an isolated place—like a cave, remote oasis, or secluded mansion—you lived a life of solitude. Adventuring might be a welcome reprieve from solitude or an unwanted change, but in either case, you’re likely still rough around the edges.",
            skill_feat: SkillFeats::DubiousKnowledge,
        }
    }
    fn hunter() -> Self {
        Self {
            background: Backgrounds::Hunter,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Wisdom],
            trained_abilities: vec![Skills::Survival],
            trained_lore: vec![LoreSkills::Tanning],
            description: "You stalked and took down animals and other creatures of the wild. Skinning animals, harvesting their flesh, and cooking them were also part of your training, all of which can give you useful resources while you adventure.",
            skill_feat: SkillFeats::SurveyWildlife,
        }
    }
    fn laborer() -> Self {
        Self {
            background: Backgrounds::Laborer,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Constitution],
            trained_abilities: vec![Skills::Athletics],
            trained_lore: vec![LoreSkills::Labor],
            description: "You’ve spent years performing arduous physical labor. It was a difficult life, but you somehow survived. You may have embraced adventuring as an easier method to make your way in the world, or you might adventure under someone else’s command.",
            skill_feat: SkillFeats::HeftyHauler,
        }
    }
    fn martial_disciple() -> Self {
        Self {
            background: Backgrounds::MartialDisciple,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Dexterity],
            trained_abilities: vec![Skills::Athletics, Skills::Acrobatics],
            trained_lore: vec![LoreSkills::Warfare],
            description: "You dedicated yourself to intense training and rigorous study to become a great warrior. The school you attended might have been a traditionalist monastery, an elite military academy, or the local branch of a prestigious mercenary organization.",
            // TODO: Multiple skill feats based on trained abilities
            skill_feat: SkillFeats::QuickJump,
        }
    }
    fn merchant() -> Self {
        Self {
            background: Backgrounds::Merchant,
            ability_boosts: vec![AbilityScores::Intelligence, AbilityScores::Charisma],
            trained_abilities: vec![Skills::Diplomacy],
            trained_lore: vec![LoreSkills::Mercantile],
            description: "In a dusty shop, market stall, or merchant caravan, you bartered wares for coin and trade goods. The skills you picked up still apply in the adventuring life, in which a good deal on a suit of armor could prevent your death.",
            skill_feat: SkillFeats::BargainHunter,
        }
    }
    fn miner() -> Self {
        Self {
            background: Backgrounds::Miner,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Wisdom],
            trained_abilities: vec![Skills::Survival],
            trained_lore: vec![LoreSkills::Mining],
            description: "You earned a living wrenching precious minerals from the lightless depths of the earth. Adventuring might have seemed lucrative or glamorous compared to this backbreaking labor— and if you have to head back underground, this time you plan to do so armed with a real weapon instead of a miner’s pick.",
            skill_feat: SkillFeats::TerrainExpertise(Some(Terrain::Underground)),
        }
    }
    fn noble() -> Self {
        Self {
            background: Backgrounds::Noble,
            ability_boosts: vec![AbilityScores::Intelligence, AbilityScores::Charisma],
            trained_abilities: vec![Skills::Society],
            trained_lore: vec![LoreSkills::Heraldry, LoreSkills::Genealogy],
            description: "To the common folk, the life of a noble seems one of idyllic luxury, but growing up as a noble or member of the aspiring gentry, you know the reality: a noble’s lot is obligation and intrigue. Whether you seek to escape your duties by adventuring or to better your station, you have traded silks and pageantry for an adventurer’s life.",
            skill_feat: SkillFeats::CourtlyGraces,
        }
    }
    fn nomad() -> Self {
        Self {
            background: Backgrounds::Nomad,
            ability_boosts: vec![AbilityScores::Constitution, AbilityScores::Wisdom],
            trained_abilities: vec![Skills::Survival],
            trained_lore: vec![LoreSkills::Terrain(None)],
            description: "Traveling far and wide, you picked up basic tactics for surviving on the road and in unknown lands, getting by with few supplies and even fewer comforts. As an adventurer, you travel still, often into even more dangerous places.",
            skill_feat: SkillFeats::Assurance(Some(Skills::Survival)),
        }
    }
    fn prisoner() -> Self {
        Self {
            background: Backgrounds::Prisoner,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Constitution],
            trained_abilities: vec![Skills::Stealth],
            trained_lore: vec![LoreSkills::Underworld],
            description: "You have been imprisoned or punished for crimes (whether you were guilty or not). Now that your sentence has ended or you’ve escaped, you take full advantage of the newfound freedom of your adventuring life.",
            skill_feat: SkillFeats::ExperiencedSmuggler,
        }
    }
    fn sailor() -> Self {
        Self {
            background: Backgrounds::Sailor,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Dexterity],
            trained_abilities: vec![Skills::Athletics],
            trained_lore: vec![LoreSkills::Sailing],
            description: "You heard the call of the sea from a young age. Perhaps you signed onto a merchant’s vessel, joined the navy, or even fell in with a crew of pirates and scalawags.",
            skill_feat: SkillFeats::UnderwaterMarauder,
        }
    }
    fn scholar() -> Self {
        Self {
            background: Backgrounds::Scholar,
            ability_boosts: vec![AbilityScores::Intelligence, AbilityScores::Wisdom],
            trained_abilities: vec![
                Skills::Arcana,
                Skills::Nature,
                Skills::Occultism,
                Skills::Religion,
            ],
            trained_lore: vec![LoreSkills::Academia],
            description: "You have a knack for learning and sequestered yourself from the outside world to learn all you could. You read about so many wondrous places and things in your books, always dreaming about one day seeing the real things. Eventually, that curiosity led you to leave your studies and become an adventurer.",
            // TODO: Multiple skill feats based on trained abilities
            skill_feat: SkillFeats::Assurance(None),
        }
    }
    fn scout() -> Self {
        Self {
            background: Backgrounds::Scout,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Wisdom],
            trained_abilities: vec![Skills::Survival],
            trained_lore: vec![LoreSkills::Terrain(None)],
            description: "You called the wilderness home as you found trails and guided travelers. Your wanderlust could have called you to the adventuring life, or perhaps you served as a scout for soldiers and found you liked battle.",
            skill_feat: SkillFeats::Forager,
        }
    }
    fn street_urchin() -> Self {
        Self {
            background: Backgrounds::StreetUrchin,
            ability_boosts: vec![AbilityScores::Dexterity, AbilityScores::Constitution],
            trained_abilities: vec![Skills::Thievery],
            trained_lore: vec![LoreSkills::City(None)],
            description: "You eked out a living by picking pockets on the streets of a major city, never knowing where you’d find your next meal. While some adventure for the glory, you do so to survive.",
            skill_feat: SkillFeats::Pickpocket,
        }
    }
    fn teacher() -> Self {
        Self {
            background: Backgrounds::Teacher,
            ability_boosts: vec![AbilityScores::Intelligence, AbilityScores::Wisdom],
            trained_abilities: vec![Skills::Society, Skills::Performance],
            trained_lore: vec![LoreSkills::Academia],
            description: "You are incredibly knowledgeable, skilled, and perhaps even trained to teach children and adults about the world and its wonders. From books to classes, you’re committed to imparting knowledge to all. Not everything can be taught or learned from a book, though, so you’ve become an adventurer to learn subjects more directly and bring that wisdom back to your students.",
            skill_feat: SkillFeats::ExperiencedProfessional,
        }
    }
    fn tinker() -> Self {
        Self {
            background: Backgrounds::Tinker,
            ability_boosts: vec![AbilityScores::Intelligence, AbilityScores::Dexterity],
            trained_abilities: vec![Skills::Crafting],
            trained_lore: vec![LoreSkills::Engineering],
            description: "Creating all sorts of minor inventions scratches your itch for problem-solving. Your engineering skills take a particularly creative bent, and no one knows what you’ll come up with next. It might be a genius device with tremendous potential or it might explode.",
            skill_feat: SkillFeats::SpecialtyCrafting,
        }
    }
    fn warrior() -> Self {
        Self {
            background: Backgrounds::Warrior,
            ability_boosts: vec![AbilityScores::Strength, AbilityScores::Constitution],
            trained_abilities: vec![Skills::Intimidation],
            trained_lore: vec![LoreSkills::Warfare],
            description: "In your younger days, you waded into battle as a mercenary, a warrior defending a nomadic people, or a member of a militia or army. You might have wanted to break away from the regimented structure of these forces, or you could have always been as independent a warrior as you are now.",
            skill_feat: SkillFeats::IntimidatingGlare,
        }
    }
}