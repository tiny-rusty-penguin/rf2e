use crate::ancestry::Ancestries;
use crate::spells::Spells;
use crate::{AbilityScores, BaseWeapons, Books, Color, Date, Domains, Fonts, Holiness, People, Player, Realm, Skills};
use crate::animals::{Animals, AnimalTraits};
#[derive(Debug)]
pub enum Deities {
    Abadar,
    Asmodeus,
    Calistria,
    CaydenCailean,
    Desna,
    Erastil,
    Gorum,
    Gozreh,
    Iomedae,
    Irori,
    Lamashtu,
    Nethys,
    Norgorber,
    Pharasma,
    Rovagug,
    Sarenrae,
    Shelyn,
    Torag,
    Urgathoa,
    ZonKuthon,
    Anubis,
    Bastet,
    Bes,
    Hathor,
    Horus,
    Isis,
    MaAt,
    Osiris,
    Ra,
    Sekhmet,
    Selket,
    Set,
    Sobek,
    Thoth,
    Wadjet,
    Baalzebul,
    Barbatos,
    Belial,
    Dispater,
    Geryon,
    Mammon,
    Mephistopheles,
    Moloch,
    Abraxas,
    Baphomet,
    CythVSug,
    Dagon,
    Gogunta,
    Haagenti,
    Kabriri,
    Kostchtchie,
    Nurgal,
    Orcus,
    Pazuzu,
    Shax,
    Sifkesh,
    Treerazer,
    Zevgavizeb,
    Zura,
    Angradd,
    Bolka,
    Dranngvit,
    Droskar,
    Folgrit,
    Grundinnar,
    Kols,
    Magrim,
    Trudd,
    CountRanalc,
    Imbrex,
    Magdh,
    Ng,
    Ragadahn,
    Shyka,
    TheGreenMother,
    TheLanternKing,
    TheLostPrince,
    Atreia,
    Ayrzul,
    Ferrumnestra,
    Hshurha,
    Kelizandri,
    Laudinmio,
    Lysianassa,
    Ranginori,
    Sairazul,
    Shumunue,
    Verilorn,
    Ymeri,
    Findeladlara,
    Ketephys,
    Yuelral,
    Andoletta,
    Arqueros,
    Arshea,
    Ashava,
    BlackButterfly,
    Cernunnos,
    Dammerich,
    Eritrice,
    Falayna,
    Halcamora,
    Irez,
    Jaidz,
    Korada,
    Lymnieris,
    Marishi,
    Pulura,
    Ragathiel,
    Shei,
    Soralyon,
    Tanagaar,
    Vildeis,
    Winlas,
    Ylimancha,
    Zohls,
    Apollyon,
    Charon,
    Szuriel,
    Trelmarixian,
    Bergelmir,
    Fandarra,
    Immonhiel,
    Rowdrosh,
    Skode,
    Barzahk,
    Imot,
    Kerkamoth,
    Monad,
    MotherVulture,
    Narakaas,
    Narriseminek,
    Otolmens,
    Saloc,
    Valmallos,
    Ydajisk,
    Adanye,
    Angazhan,
    Balumbdar,
    GrandmotherSpider,
    Kalekot,
    Lubaiko,
    Mazludeh,
    Uvuko,
    Walkena,
    Chohar,
    Dajermube,
    Luhar,
    Tlehar,
    Aakriti,
    Achaekek,
    Ahriman,
    Alseta,
    Apsu,
    Arazni,
    Besmara,
    Brigh,
    Casandalee,
    Chaldira,
    ConquerorWorm,
    Dahak,
    Erecura,
    FollowersOfFate,
    Gendowyn,
    Ghlaunder,
    GreenMan,
    Groetus,
    Gyronna,
    Hanspur,
    Jaidi,
    Kazutal,
    Kitumu,
    Kurgess,
    Lissala,
    Lorthact,
    Milani,
    Naderi,
    NiviRhombodazzle,
    Nocticula,
    Picoperi,
    Ravithra,
    Sivanah,
    Thamir,
    Thisamet,
    Vineshvakhi,
    Ydersius,
    Yelayne,
    Zyphus,
    Azathoth,
    Hastur,
    Nhimbaloth,
    Nyarlathotep,
    XhamenDor,
    YogSothoth,
    ArdadLili,
    Doloras,
    Eiseth,
    Mahathallah,
    Alglenweis,
    StagMotherOfTheForestOfStones,
    Sturovenen,
    Baekho,
    Daikitsu,
    Fumeiyoshi,
    GeneralSusumu,
    HeiFeng,
    JinLi,
    Kofusachi,
    LadyJingxi,
    LadyNanbyo,
    LaoShuPo,
    MuguraAndNrithu,
    Nalinivati,
    PhiDeva,
    QiZhong,
    Shizuru,
    Srikalis,
    SunWukong,
    TheLadyOfTheNorthStar,
    Tsukiyo,
    Yaezhing,
    Yamatsumi,
    Arundhat,
    Ashukharma,
    Chamidu,
    Dhalavei,
    Diomazul,
    Gruhastha,
    Lahkgya,
    Likha,
    Matravash,
    Ragdya,
    Raumya,
    Suyuddha,
    Atheism,
    EsotericOrderOfThePalatineEye,
    GodCalling,
    GreenFaith,
    LawsOfMortality,
    PropheciesOfKalistrade,
    Sangpotshi,
    ShoantiAnimism,
    WhisperingWay,
    Aroden,
    Gaelata,
    Lalaci,
    Lixiriltha,
    PeacockSpirit,
    Diabolism // todo check this one
}

enum DeityCategories {
    GodOfTheInnerSea,
}

enum DeityConcerns {
    Birth,
    Death,
    Fate,
    Prophecy,
    Time,
    Cities,
    Law,
    Merchants,
    Wealth,
    Contracts,
    Pride,
    Slavery,
    Tyranny,
    Lust,
    Revenge,
    Trickery,
    Ale,
    Bravery,
    Freedom,
    Wine,
    Luck,
    Stars,
    Travelers,
    Dreams,
    Family,
    Farming,
    Hunting,
    Trade,
    Battle,
    Strength,
    Weapons,
    Nature,
    Sea,
    Weather,
    Honor,
    Justice,
    Rulership,
    Valor,
    History,
    Knowledge,
    SelfPerfection,
}

enum Pantheons {
    TheDeliberateJourney,
    TheGodClaw,
    TheOfferingPlate,
    TheResplendentCourt,
    UrbanProsperity,
    ElvenPantheon,
    TheLastBreath,
    TheDivineDare,
    TheFreeingFlame,
    TheLaborersBastion,
    CosmicCaravan,
    ThePathOfTheHeavens,
    ThePrismaticRay,
    HearthAndHarvest,
    SeafarersHope,
    ThePillarsOfKnowledge,
}

enum SacredDays {
    MarketsDoor,
    Taxfest,
    Asmodeus1,
    Asmodeus2,
    Asmodeus3,
    FirstBrewing,
    AscensionDay,
    AscendanceDay,
    Merrymead,
    RitualOfStardust,
    SwallowtailFestival,
    PlantingWeek,
    HarvestFeast,
    Archerfeast,
    DayOfTheInheritor,
    FirstCrusaderDay,
    Armasse,
    RemembranceMoon,
    SelfDecided,

}

struct SacredDay {
    name: &'static str,
    description: &'static str,
    date: Date,
}

struct Deity {
    pub alive: bool,
    pub alternative_names: Vec<&'static str>,
    pub adjective: Vec<&'static str>,
    pub description: &'static str,
    pub category: DeityCategories,
    pub edicts: Vec<&'static str>,
    pub anathema: Vec<&'static str>,
    pub areas_of_concern: Vec<DeityConcerns>,
    pub pantheons: Option<Vec<Pantheons>>,
    pub realm: Vec<Realm>,
    pub worshippers: Vec<People>,
    pub ancestry_worshippers: Option<Vec<Ancestries>>,
    pub symbol: Vec<&'static str>,
    pub sacred_animal: Option<Vec<Animals>>,
    pub favoured_animal: Option<Vec<Animals>>,
    pub sacred_color: Vec<Color>,
    pub sacred_days: Option<Vec<SacredDays>>,
    pub sacred_texts: Vec<Books>,

    pub divine_ability: Vec<AbilityScores>,
    pub divine_font: Vec<Fonts>,
    pub divine_sanction: Option<Vec<Holiness>>,
    pub divine_skill: Skills,
    pub favored_weapon: BaseWeapons,
    pub domains: Vec<Domains>,
    pub alternate_domains: Option<Vec<Domains>>,
    pub cleric_spells: Vec<Spells>,

    pub divine_intercession: Option<&'static str>,
    pub minor_boon_description: Option<&'static str>,
    pub moderate_boon_description: Option<&'static str>,
    pub major_boon_description: Option<&'static str>,
    pub minor_curse_description: Option<&'static str>,
    pub moderate_curse_description: Option<&'static str>,
    pub major_curse_description: Option<&'static str>,
    pub minor_boon: Option<fn(&mut Player) -> anyhow::Result<()>>,
    pub moderate_boon: Option<fn(&mut Player) -> anyhow::Result<()>>,
    pub major_boon: Option<fn(&mut Player) -> anyhow::Result<()>>,
    pub minor_curse: Option<fn(&mut Player) -> anyhow::Result<()>>,
    pub moderate_curse: Option<fn(&mut Player) -> anyhow::Result<()>>,
    pub major_curse: Option<fn(&mut Player) -> anyhow::Result<()>>,
}

impl Deity {
    pub fn all() -> Vec<Deity> {
        vec![
            Self::abadar(),
            Self::asmodeus(),
            Self::calistria(),
            Self::desna(),
            Self::cayden_cailean(),
            Self::erastil(),
        ]
    }
    fn abadar() -> Self {
        Self {
             alive: true,
             alternative_names: vec![
                 "Master of the First Vault",
                 "God of Walls and Ditches",
                 "The Gold-Fisted",
                 "Judge of the Gods",
                 "God of the First Vault",
                 "Wealthy Father",
                 "Two-Headed Eagle",
                 "Azlanti period: The Scales and Streets",
             ],
             adjective: vec!["Abadaran", "Abadarian"],
             description: "Abadar is worshipped as the god of cities, law, merchants, and wealth. Abadar’s cathedral-banks are found in many cities and places where order thrives or is gaining a foothold. Aristocrats, city guards, merchants, and those working in legal practice or who have the well-being of their community on their mind are common worshippers of the god of cities, along with dwarves in general. Abadaran priests living in cities often serve as judges, lawyers, and clerks, while those who live on the frontier work as roving magistrates, acting as judge, jury, and executioners in the name of order.",
             category: DeityCategories::GodOfTheInnerSea,
             edicts: vec!["bring civilization of frontiers", "earn wealth through hard word and trade", "follow the rule of law"],
             anathema: vec!["engage in banditry or piracy", "steal", "undermine a law-abiding court"],
             areas_of_concern: vec![
                 DeityConcerns::Cities,
                 DeityConcerns::Law,
                 DeityConcerns::Merchants,
                 DeityConcerns::Wealth,
             ],
             pantheons: Some(vec![
                 Pantheons::TheDeliberateJourney,
                 Pantheons::TheOfferingPlate,
                 Pantheons::TheResplendentCourt,
                 Pantheons::UrbanProsperity
             ]),
             realm: vec![Realm::FirstVault, Realm::Aktun, Realm::Axis,],
             worshippers: vec![
                 People::Architects,
                 People::Aristocrats,
                 People::Bankers,
                 People::Judges,
                 People::Lawmakers,
                 People::Lawyers,
                 People::Merchants,
             ],
             ancestry_worshippers: None,
             symbol: vec!["Golden Key"],
             sacred_animal: Some(vec![Animals::Monkey]),
             favoured_animal: Some(
                vec![
                    Animals::GoldenEagle,
                    Animals::Beaver,
                    Animals::HouseCat,
                ]
                    .into_iter()
                    .chain(Animals::beasts_of_burden().into_iter())
                    .chain(Animals::mounts().into_iter())
                    .chain(Animals::watchdog().into_iter())
                    .collect::<Vec<_>>())
                .map(|mut v| { v.sort();v.dedup();v }),
             sacred_color: vec![Color::Gold, Color::Silver],
             sacred_days: Some(vec![SacredDays::MarketsDoor, SacredDays::Taxfest]),
             sacred_texts: vec![Books::TheOrderOfNumbers, Books::TheManualOfCityBuilding],
             divine_ability: vec![AbilityScores::Constitution, AbilityScores::Intelligence],
             divine_font: vec![Fonts::Harmful, Fonts::Healing],
             divine_sanction: Some(vec![Holiness::Holy, Holiness::Unholy]),
             divine_skill: Skills::Society,
             favored_weapon: BaseWeapons::Crossbow,
             domains: vec![Domains::Cities, Domains::Earth, Domains::Travel, Domains::Wealth,],
             alternate_domains: Some(vec![Domains::Duty]),
             cleric_spells: vec![Spells::IllusoryObject, Spells::Creation, Spells::PlanarPalace],
             divine_intercession: Some("Abadar’s gifts take the form of riches, while his ire tends to cause offenders to lose wealth."),
             minor_boon_description: Some("Abadar warns his favored against those who might unfairly take advantage. Once, when someone rolls a success on a Deception check to Lie maliciously to you and you alone, they get a critical failure instead. Abadar typically chooses to grant this boon in response to an extremely consequential lie."),
             moderate_boon_description: Some("Abadar blesses all your enterprises, leading to financial success as all your ventures always seems to work out. If you roll a critical failure at a check to Earn Income, you get a failure instead. If you roll a success on a check to Earn Income, you earn twice the usual amount of income."),
             major_boon_description: Some("Your silver tongue is infallible, allowing you to reach a compromise if one is even remotely possible. Once per day, you know just what to offer to make a deal or broker a negotiation, and if you offer your divinely inspired deal, you can automatically receive a result of 20 + your Diplomacy modifier on your Diplomacy check instead of rolling. This does not increase your degree of success like rolling a 20 would. If there is legitimately nothing you could offer to reach an agreement, you learn that, and you don’t expend your daily use of the boon."),
             minor_curse_description: Some("Any time you steal, illegally harm or kill another creature, or undermine a law-abiding officer or court, a symbol or word describing your crime appears on a visible spot on your skin. This symbol cannot be removed or hidden with makeup (though it can be covered with clothing) and it doesn’t vanish until you make legal restitution for the crime, such as by serving your sentence."),
             moderate_curse_description: Some("Abadar curses all your enterprises, leading to financial disaster as all your ventures always seem to fail. The result of your check to Earn Income is always one degree of success worse than the one you rolled."),
             major_curse_description: Some("You become honest to a fault. You constantly suffer the critical failure effect of zone of truth. Additionally, you are always offered the worst possible option in a bargain."),
             minor_boon: Some(abadar_minor_boon),
             moderate_boon: Some(abadar_moderate_boon),
             major_boon: Some(abadar_major_boon),
             minor_curse: Some(abadar_minor_curse),
             moderate_curse: Some(abadar_moderate_curse),
             major_curse: Some(abadar_major_curse),
         }
    }

    fn asmodeus() -> Self {
        Self {
            alive: true,
            alternative_names: vec![
                "The Prince of Darkness",
                "The Archfiend",
                "The Dark Prince",
                "The First",
                "The God-Fiend",
                "King of Hell",
                "Lord of Darkness",
                "Lord of Darkness and Law",
                "Lord of Hell",
                "Lord of the Pit",
                "Master of Witches",
                "Prince of Devils",
                "Prince of Hell",
                "Prince of Law",
                "Ruler of Hell",
            ],
            adjective: vec!["Asmodean"],
            description: "Asmodeus is the First, the Dark Prince, the lord of darkness and law, and the ruler of the plane of Hell. If Asmodeus’s own scriptures are to believed—and they are corroborated by certain other accounts, like the angel-penned Book of the Damned—he is one of the oldest beings of the multiverse. These texts claim that in time before time, in a world not yet created, Asmodeus and his brother Ihys were among the first gods in existence. During these unnamed ages, the two gods quarreled over the fate of the souls of their creations, and Asmodeus slew his brother. Confident that Ihys’s act of granting mortals free will was folly, Asmodeus made his own convictions known: that existence is best served by absolute order and discipline. These claims contradict other popular creation myths, and both theologians and immortal agents of the gods doubt Asmodeus’s claims to varying degrees, but while there is no evidence to prove them, they are also difficult to refute.",
            category: DeityCategories::GodOfTheInnerSea,
            edicts: vec![
                "negotiate a contract to your best advantage",
                "rule tyrannically",
                "show subservience to your better"
            ],
            anathema: vec![
                "break a contract",
                "share power with the weak",
                "insult Asmodeus by showing mercy to your enemies"
            ],
            areas_of_concern: vec![
                DeityConcerns::Contracts,
                DeityConcerns::Pride,
                DeityConcerns::Slavery,
                DeityConcerns::Tyranny,
            ],
            pantheons: Some(vec![Pantheons::TheGodClaw]),
            realm: vec![Realm::TheCatafalque, Realm::Nessus, Realm::Hell],
            worshippers: vec![
                People::Slavers,
                People::Bureaucrats,
                People::Tyrants,
                People::Diabolists,
                People::Lawyers,
            ],
            ancestry_worshippers: None,
            symbol: vec!["Red pentagram", "Archstar"],
            sacred_animal: Some(Animals::snakes()),
            favoured_animal: Some(vec![
                Animals::Lion,
                Animals::Goat,
                Animals::Rooster,
            ]),
            sacred_color: vec![Color::Black, Color::Red],
            sacred_days: Some(vec![SacredDays::Asmodeus1, SacredDays::Asmodeus2, SacredDays::Asmodeus3]),
            sacred_texts: vec![Books::TheAsmodeanMonograph],
            divine_ability: vec![
                AbilityScores::Constitution,
                AbilityScores::Dexterity,
                AbilityScores::Charisma,
                AbilityScores::Wisdom,
                AbilityScores::Strength,
                AbilityScores::Intelligence
            ],
            divine_font: vec![Fonts::Harmful],
            divine_sanction: Some(vec![Holiness::Unholy]),
            divine_skill: Skills::Deception,
            favored_weapon: BaseWeapons::Mace,
            domains: vec![Domains::Confidence, Domains::Fire, Domains::Trickery, Domains::Tyranny,],
            alternate_domains: Some(vec![Domains::Duty, Domains::Glyph]),
            cleric_spells: vec![Spells::Charm, Spells::Suggestion, Spells::Mislead,],
            divine_intercession: Some("Asmodeus tends to offer his gifts to entice those on the precipice of yielding to his vile temptations. His curses come most often in response to those who break contracts in his name, or commit other personal insults."),
            minor_boon_description: Some("Pleased by your talent for manipulation, Asmodeus amplifies your skills. Once, when you fail at the Diplomacy check to make a significant or consequential Request, you can cast suggestion on the target of your Request, suggesting the same course of action. This is a divine innate spell."),
            moderate_boon_description: Some("Your eyes glow red like embers and your skin takes on a crimson tone. You gain darkvision and fire resistance 5."),
            major_boon_description: Some("Asmodeus helps enforce your bargains and contracts. When a creature enters a bargain or contract with you, uncoerced and of its own free will, it can’t voluntarily violate its side of the bargain as long as you uphold your side. You can always choose to violate the bargain yourself, but if you do, the creature is no longer bound to fulfill its part."),
            minor_curse_description: Some("Asmodeus’s flames burn you with great malice. You gain weakness 5 to fire."),
            moderate_curse_description: Some("Asmodeus forces your compliance. You can’t voluntarily back out of an agreement or contract or go back on your word, though you need follow only the letter of the agreement, not the spirit."),
            major_curse_description: Some("Asmodeus has taken note of the chaos you have wrought. You receive an ancient wound that feels older than time itself. You become permanently drained 4, and nothing short of another intercession can remove the condition. Your wound aches fiercely whenever you perform a particularly chaotic act, causing you to become sickened 1."),
            minor_boon: Some(asmodeus_minor_boon),
            moderate_boon: Some(asmodeus_moderate_boon),
            major_boon: Some(asmodeus_major_boon),
            minor_curse: Some(asmodeus_minor_curse),
            moderate_curse: Some(asmodeus_moderate_curse),
            major_curse: Some(asmodeus_major_curse),
        }
    }

    fn calistria() -> Self {
        Self {
            alive: true,
            alternative_names: vec![
                "The Savored Sting",
                "The Unquenchable Fire",
                "The Lady in the Room"
            ],
            adjective: vec!["Calistrian"],
            description: "As symbolized by the three daggers of her religious symbol, Calistria has three aspects: lust, revenge, and trickery. Silver-tongued and charming, she is a master of weaving insults into compliments and laying intricate groundwork for retribution at its finest. She is a goddess of vengeance, but it would be a mistake to assume that means she pursues justice. Calistria is fickle, shifting her loyalties and interests as her whims take her—though she never forgets a slight, and any who think she has forgiven will surely find it is only a matter of time before they are targeted by a long-term plot of revenge to lay them thoroughly low.",
            category: DeityCategories::GodOfTheInnerSea,
            edicts: vec!["pursue your personal freedom", "seek hedonistic thrills", "take revenge"],
            anathema: vec![
                "become too consumed by love or a need for revenge",
                "let a slight go unanswered"
            ],
            areas_of_concern: vec![
                DeityConcerns::Lust,
                DeityConcerns::Revenge,
                DeityConcerns::Trickery,
            ],
            pantheons: Some(vec![Pantheons::TheLastBreath, Pantheons::ElvenPantheon]),
            realm: vec![Realm::TheGardensOfDeceitAndDelight, Realm::Elysium],
            worshippers: vec![
                People::Hedonists,
                People::Performers,
                People::ScornedLovers,
                People::Thieves,
            ],
            ancestry_worshippers: Some(vec![Ancestries::Elf]),
            symbol: vec!["Three Daggers"],
            sacred_animal: Some(vec![Animals::Wasp]),
            favoured_animal: Some(Animals::poisonous_animals()),
            sacred_color: vec![Color::Black, Color::Yellow],
            sacred_days: None,
            sacred_texts: vec![Books::TheBookOfJoy, Books::BloodForWine],
            divine_ability: vec![AbilityScores::Dexterity, AbilityScores::Charisma],
            divine_font: vec![Fonts::Harmful, Fonts::Healing],
            divine_sanction: Some(vec![Holiness::Holy, Holiness::Unholy]),
            divine_skill: Skills::Deception,
            favored_weapon: BaseWeapons::Whip,
            domains: vec![Domains::Pain, Domains::Passion, Domains::Secrecy, Domains::Trickery,],
            alternate_domains: None,
            cleric_spells: vec![Spells::Charm, Spells::Enthrall, Spells::Mislead,],
            divine_intercession: Some("Signs of favor or displeasure from the Savored Sting are sometimes subtle and at other times incontrovertible. Calistria typically grants her boon to those on the path toward great vengeance and curses those who slight her followers, particularly if those followers are sex workers, though her fickle heart rarely commits to any absolute guidelines."),
            minor_boon_description: Some("Calistria smiles on the riskiest deceptions. Once, when you roll a failure on a check to Lie, you get a critical success instead. Calistria typically grants this boon for an extremely consequential lie."),
            moderate_boon_description: Some("A foot-long wasp finds and befriends you. It serves you as a familiar as long as you maintain Calistria’s grace. The wasp always has the burrower and flier familiar abilities."),
            major_boon_description: Some("Calistria guides you towards vengeance. You always know the direction and distance towards the nearest creature that has wronged you and thus far gone unpunished."),
            minor_curse_description: Some("Whenever a new person desires vengeance against you, you suffer a painful sting and are afflicted with giant wasp venom at stage 1."),
            moderate_curse_description: Some("People react as though you’re insulting them, even in normal conversation. Whenever you attempt to Make an Impression, the outcome is one degree of success worse than the result of your roll. If you converse with someone over a long enough period of time but don’t attempt to Make an Impression, you still insult them, and you suffer the effect of a critical failure to Make an Impression."),
            major_curse_description: Some("You have wronged those unable to obtain revenge for themselves, and Calistria’s curse grants their revenge its own life. Whenever another creature imagines vengeance upon you but can’t pursue that vengeance because you are too powerful, well connected, or otherwise untouchable, a creature of roughly your level manifests out of their imagination and performs their desired revenge. Once the revenge is complete or the manifestation is destroyed, the summoned creature vanishes from existence."),
            minor_boon: Some(calistria_minor_boon),
            moderate_boon: Some(calistria_moderate_boon),
            major_boon: Some(calistria_major_boon),
            minor_curse: Some(calistria_minor_curse),
            moderate_curse: Some(calistria_moderate_curse),
            major_curse: Some(calistria_major_curse),
        }
    }

    fn cayden_cailean() -> Self {
        Self {
            alive: true,
            alternative_names: vec![
                "The Accidental God",
                "The Drunken Hero",
                "The Lucky Drunk",
                "The Drunken God",
                "The Merry Drunk",
                "The Merry Insurgent",
            ],
            adjective: vec!["Caydenite"],
            description: "Once a mortal human, Cayden Cailean is now one the few deities known as the Ascended. In his mortal years, Cayden was a sellsword of no small fame, known for his boisterous manner, skill with a blade, and fearless resolve. During a particularly rowdy night of drinking, a series of escalating dares led the wandering mercenary to attempt the Test of the Starstone. He emerged from the Starstone Cathedral 3 days later, laughing, a fully realized god. Divine responsibility did little to change Cayden’s attitude from what it was in his mortal life. He continues to crave adventure, drink, and pleasurable company while abhorring bullies, tyrants, and cowards.",
            category: DeityCategories::GodOfTheInnerSea,
            edicts: vec!["drink", "aid the oppressed", "seek glory and adventure"],
            anathema: vec!["waste alcohol", "be mean or standoffish when drunk", "oppress the vulnerable"],
            areas_of_concern: vec![
                DeityConcerns::Ale,
                DeityConcerns::Bravery,
                DeityConcerns::Freedom,
                DeityConcerns::Wine,
            ],
            pantheons: Some(vec![
                Pantheons::TheDivineDare,
                Pantheons::TheFreeingFlame,
                Pantheons::TheLaborersBastion,
                Pantheons::TheOfferingPlate
            ]),
            realm: vec![Realm::CaydenCity, Realm::Elysium, Realm::HerosHeart],
            worshippers: vec![
                People::Brewers,
                People::Vintners,
                People::Barkeeps,
                People::Innkeepers,
                People::Adventurers,
            ],
            ancestry_worshippers: None,
            symbol: vec!["Tankard"],
            sacred_animal: Some(Animals::hounds()),
            favoured_animal: Some(
                Animals::eagles()
                    .into_iter()
                    .chain(std::iter::once(Animals::Animal(AnimalTraits::EscapedCaptivity)))
                    .collect::<Vec<_>>(),
            ),
            sacred_color: vec![Color::Silver, Color::Tan],
            sacred_days: Some(vec![
                SacredDays::FirstBrewing,
                SacredDays::AscensionDay,
                SacredDays::Merrymead
            ]),
            sacred_texts: vec![Books::PlacardOfWisdom],
            divine_ability: vec![AbilityScores::Constitution, AbilityScores::Charisma],
            divine_font: vec![Fonts::Healing],
            divine_sanction: Some(vec![Holiness::Holy]),
            divine_skill: Skills::Athletics,
            favored_weapon: BaseWeapons::Rapier,
            domains: vec![Domains::Cities, Domains::Freedom, Domains::Indulgence, Domains::Might,],
            alternate_domains: None,
            cleric_spells: vec![Spells::FleetStep, Spells::Stupefy, Spells::Hallucination,],
            divine_intercession: Some("Cayden sometimes hands out his blessings and communicates his ire at seemingly random opportunities based on his drunken whims. In particular, Cayden blesses the recently liberated to help secure their freedom. Cayden is also known to grant particularly trivial and harmless curses to those who disrupt revelry. These curses typically cause the disruptor to change in appearance, taking on a comical or farcical look. He is quick to lift these curses by dawn or for those who give in to the enjoyment of the evening and join the festivities."),
            minor_boon_description: Some("Cayden Cailean helps you recover from nights of carousing. While you still get drunk and otherwise experience the effects of alcohol normally, you are never hung over the next morning."),
            moderate_boon_description: Some("You share some of the bravery Cayden espoused during his ascension. When you roll a success on a saving throw against a fear effect, you get a critical success instead. If you have the fighter bravery class feature, when you roll a critical failure on a save against a fear effect, you get a failure instead."),
            major_boon_description: Some("An embodiment of freedom, you break fetters with ease. At the end of each of your turns, you can end one effect that is currently making you immobilized (including effects that make you immobilized by grabbing or restraining you) as a free action."),
            minor_curse_description: Some("Individuals who raise Cayden’s passing displeasure awake as if from a hard night of drinking and find that drink tastes foul"),
            moderate_curse_description: Some("Cayden has afflicted you with cowardice. Whenever you roll a critical success on a saving throw against a fear effect, you get a success instead, and each time you roll a failure on a saving throw against a fear effect, you get a critical failure instead."),
            major_curse_description: Some("Cayden curses you to suffer in the presence of drinking, making it hard for you to bear entering any tavern. Whenever any creature drinks alcohol within 100 feet of you, you suffer all negative effects from the alcohol (but only the alcohol) instead. This doesn’t apply if the creature gains any special effect from drinking alcohol."),
            minor_boon: Some(cayden_cailean_minor_boon),
            moderate_boon: Some(cayden_cailean_moderate_boon),
            major_boon: Some(cayden_cailean_major_boon),
            minor_curse: Some(cayden_cailean_minor_curse),
            moderate_curse: Some(cayden_cailean_moderate_curse),
            major_curse: Some(cayden_cailean_major_curse),
        }
    }

    fn desna() -> Self {
        Self {
            alive: true,
            alternative_names: vec![
                "The Song of the Spheres",
                "The Great Dreamer",
                "Starsong",
                "The Tender of Dreams",
                "Lady Luck",
                "Resplendent Goddess of Fortune",
                "Goddess of the North Star",
                "Queen of the North Star",
                "Mother Moon",
                "The Wandering Star"
            ],
            adjective: vec!["Desnan"],
            description: "The night didn’t know beauty until Desna came into existence. While the other gods toiled away to create the world, she set her sights on the heavens, placing each star in the sky. After surveying her artistry, she hung the brightest star high in the north and made it her home. Her first gift to mortals was this beacon of hope, a twinkling sign in the dark sky that they could turn to when lost or unsure of themselves. Desna provides safe passage through the darkness to all, should they choose to follow.",
            category: DeityCategories::GodOfTheInnerSea,
            edicts: vec![
                "aid fellow travelers",
                "explore new places",
                "express yourself through art and song",
                "find what life has to offer"],
            anathema: vec![
                "foster despair or terror in the innocent",
                "cast nightmare or use similar magic to corrupt dreams",
                "engage in bigoted behavior"
            ],
            areas_of_concern: vec![
                DeityConcerns::Luck,
                DeityConcerns::Stars,
                DeityConcerns::Travelers,
                DeityConcerns::Dreams,
            ],
            pantheons: Some(vec![
                Pantheons::CosmicCaravan,
                Pantheons::ElvenPantheon,
                Pantheons::TheDeliberateJourney,
                Pantheons::ThePathOfTheHeavens,
                Pantheons::ThePrismaticRay
            ]),
            realm: vec![Realm::ThePlaceSevenfoldCynosure],
            worshippers: vec![
                People::Travelers,
                People::Astronomers,
                People::Gamblers,
                People::Varisians,
                People::Musicians,
            ],
            ancestry_worshippers: None,
            symbol: vec!["Butterfly"],
            sacred_animal: Some(vec![Animals::Butterfly]),
            favoured_animal: Some(vec![
                Animals::Moth,
                Animals::Caterpillar,
                Animals::Owl,
                Animals::Sparrow,
                Animals::Dragonfly,
            ]
                .into_iter()
                .chain(Animals::messenger_birds().into_iter())
                .collect::<Vec<_>>())
                .map(|mut v| { v.sort();v.dedup();v }),
            sacred_color: vec![Color::Blue, Color::White],
            sacred_days: Some(vec![SacredDays::RitualOfStardust, SacredDays::SwallowtailFestival]),
            sacred_texts: vec![Books::TheEightScrolls, Books::ShrineWallWritings],
            divine_ability: vec![AbilityScores::Dexterity, AbilityScores::Charisma],
            divine_font: vec![Fonts::Healing],
            divine_sanction: Some(vec![Holiness::Holy]),
            divine_skill: Skills::Acrobatics,
            favored_weapon: BaseWeapons::Starknife,
            domains: vec![Domains::Dreams, Domains::Luck, Domains::Moon, Domains::Travel,],
            alternate_domains: Some(vec![Domains::Star, Domains::Void]),
            cleric_spells: vec![Spells::Sleep, Spells::Translocate, Spells::DreamingPotential,],
            divine_intercession: Some("Desna favors those who follow their hearts and whims without bringing harm to others."),
            minor_boon_description: Some("For those at the end of a journey, Desna gifts a deep sleep. Once, after you rest, you completely recover all Hit Points, remove all negative conditions, and become free of any curses or diseases."),
            moderate_boon_description: Some("Desna twists fortune in your favor. Once per day, after determining the result of a check, you can reroll the check and take the new result."),
            major_boon_description: Some("Desna bestows you with a swirling cloud of lights that forms a pair of wondrous butterfly wings. These wings grant you a fly Speed of 40 feet and shine dim light to a range of 20 feet."),
            minor_curse_description: Some("You always seem to lose your way or have strange mishaps on the road that delay your travel. You (and thus any group that travels with you) travel at only 3/4 normal exploration Speed, before taking into account the terrain and other features that might slow you down further."),
            moderate_curse_description: Some("Misfortune follows you in your travels and requires acts of contrition or benevolence to keep at bay. You must always roll twice and take the worst result when attempting a check. This is a misfortune effect. You can express your remorse and ignore this effect for 1d4 rounds by spending an action, which has the concentrate trait. If you perform a truly selfless act of compassion, you ignore this effect for 1 day."),
            major_curse_description: Some("The stars rebuke you for your ways. You are sickened 4 and slowed 1 whenever you are exposed to starlight."),
            minor_boon: Some(desna_minor_boon),
            moderate_boon: Some(desna_moderate_boon),
            major_boon: Some(desna_major_boon),
            minor_curse: Some(desna_minor_curse),
            moderate_curse: Some(desna_moderate_curse),
            major_curse: Some(desna_major_curse),
        }
    }
    fn erastil() -> Self {
        Self {
            alive: true,
            alternative_names: vec![
                "Old Deadeye",
                "Stag God",
                "Estig the Hunter",
                "Elk Father"
            ],
            description: "Unlike many other good deities, Erastil does not send his followers out into the world to fight and crush evil. Eschewing crusades and other ventures that take his followers away from their homes, Erastil watches over those who devote their lives to family and community. He is primarily an agricultural deity, specifically focusing on those aspects of nature that either can be tamed or are of use to his followers. His domain encompasses the plants and animals that farmers, hunters, and ranchers deal with in their everyday lives. While he is a protective deity, Erastil steps in only when quiet, pastoral lives are threatened. He desires his followers to live their lives in peace, with no risk of being conscripted into armies, devoured by monsters, or destroyed by magic.",
            adjective: vec!["Erastilian"],
            category: DeityCategories::GodOfTheInnerSea,
            edicts: vec![
                "care for your home and family",
                "fulfill your duties",
                "keep the peace",
                "protect the community"
            ],
            anathema: vec![
                "abandon your home in its time of need",
                "choose yourself over your community",
                "tarnish your reputation",
                "tell lies"
            ],
            areas_of_concern: vec![
                DeityConcerns::Family,
                DeityConcerns::Farming,
                DeityConcerns::Hunting,
                DeityConcerns::Trade,
            ],
            pantheons: Some(vec![Pantheons::HearthAndHarvest]),
            realm: vec![Realm::Summerlands, Realm::Requuius, Realm::Heaven,],
            worshippers: vec![
                People::Hunters,
                People::Farmers,
                People::Tradesmen,
            ],
            ancestry_worshippers: None,
            symbol: vec!["Bow and Arrow"],
            sacred_animal: Some(vec![Animals::Stag]),
            favoured_animal: Some(
                     Animals::hoofed_animals()
                    .into_iter()
                    .chain(Animals::farm_animals().into_iter())
                    .collect::<Vec<_>>())
                .map(|mut v| { v.sort();v.dedup();v }),
            sacred_color: vec![Color::Brown, Color::Green],
            sacred_days: Some(vec![
                SacredDays::PlantingWeek,
                SacredDays::HarvestFeast,
                SacredDays::Archerfeast
            ]),
            sacred_texts: vec![Books::ParablesOfErastil],
            divine_ability: vec![AbilityScores:: Constitution, AbilityScores::Wisdom],
            divine_font: vec![Fonts::Healing],
            divine_sanction: Some(vec![Holiness::Holy]),
            divine_skill: Skills::Survival,
            favored_weapon: BaseWeapons::Longbow,
            domains: vec![Domains::Earth, Domains::Family, Domains::Nature, Domains::Wealth,],
            alternate_domains: Some(vec![Domains::Duty]),
            cleric_spells: vec![Spells::SureStrike, Spells::WallOfThorns, Spells::NaturesPathway,],
            divine_intercession: Some("Erastil favors those who commit themselves to their communities and detests those who disrupt these families."),
            minor_boon_description: Some("Erastil shares in his bounty as long as you work for it. Whenever you roll a critical failure at a check to Subsist in the wild, you get a failure instead."),
            moderate_boon_description: Some("You share Erastil’s sharp eye, allowing you to use a longbow in any situation. When you attack with a longbow, you can ignore the longbow’s volley trait, and longbows have double the normal range increment for you."),
            major_boon_description: Some("You become a friend to the land. You can cast speak with plants as a divine innate spell. When you are in a healthy natural environment, the land sustains you without need for food or drink. You can cast the commune with nature ritual without any secondary casters or secondary checks."),
            minor_curse_description: Some("Erastil curses your table and punishes you for relying on others’ hard work. Any food you didn’t grow, gather, or otherwise harvest yourself tastes like ash and leaves you fatigued."),
            moderate_curse_description: Some("Erastil’s beasts take a dislike to you. Animals’ attitudes toward you are one category worse (friendly instead of helpful, indifferent instead of friendly, and so on)."),
            major_curse_description: Some("Erastil leaves you unable to have children (or otherwise reproduce or propagate if you do so in a different manner). This curse also affects your livestock, pets, and crops."),
            minor_boon: Some(erastil_minor_boon),
            moderate_boon: Some(erastil_moderate_boon),
            major_boon: Some(erastil_major_boon),
            minor_curse: Some(erastil_minor_curse),
            moderate_curse: Some(erastil_moderate_curse),
            major_curse: Some(erastil_major_curse),
        }
    }

    fn gorum() -> Self {
        Self {
            alive: false,
            alternative_names: vec!["Our Lord in Iron"],
            description: "The clash of steel, the cry of victory, the gasping denial of death: these are the sound of prayers to Our Lord in Iron, for to follow Gorum is to fight. Gorum does not care the reason for battle—a village’s desperate stand against raiders is no less worthwhile to him than a crusader army marching against demons in the Sarkoris Scar—nor does he choose sides in such clashes. Good or evil, law or chaos, the reason for the fight is irrelevant. It is the thrill of battle that finds his favor, the crucible of struggle in which victory is there for the taking.",
            adjective: vec!["Gorumite"],
            category: DeityCategories::GodOfTheInnerSea,
            edicts: vec![
                "attain victory in fair combat",
                "push your limits",
                "wear armor in combat"
            ],
            anathema: vec![
                "kill prisoners or surrendering foes",
                "prevent conflict through negotiation",
                "win a battle through underhanded tactics or indirect magic"
            ],
            areas_of_concern: vec![
                DeityConcerns::Battle,
                DeityConcerns::Strength,
                DeityConcerns::Weapons,
            ],
            pantheons: None,
            realm: vec![Realm::Elysium],
            worshippers: vec![
                People::Soldiers,
                People::Mercenaries,
                People::Brigands,
                People::Barbarians,
            ],
            // dromaars
            ancestry_worshippers: None,
            symbol: vec!["Sword in mountain"],
            sacred_animal: Some(vec![Animals::Rhinoceros]),
            favoured_animal: None,
            sacred_color: vec![Color::Red, Color::Grey],
            sacred_days: None,
            sacred_texts: vec![Books::Gorumskagat, Books::MythInIron],
            divine_ability: vec![AbilityScores::Strength, AbilityScores::Constitution],
            divine_font: vec![Fonts::Healing, Fonts::Harmful],
            divine_sanction: Some(vec![Holiness::Holy, Holiness::Unholy]),
            divine_skill: Skills::Athletics,
            favored_weapon: BaseWeapons::Greatsword,
            domains: vec![Domains::Confidence, Domains::Destruction, Domains::Might, Domains::Zeal],
            alternate_domains: None,
            cleric_spells: vec![Spells::SureStrike, Spells::Enlarge, Spells::WeaponStorm,],
            divine_intercession: Some("Gorum views things very simply: one either fights and earns his favor, or one is a coward and receives only scorn."),
            minor_boon_description: Some("Gorum grants you a weapon whenever you need one. You can use an Interact action to draw a 0-level non-magical iron weapon, even if you have no weapons on your person. Such a weapon lasts only as long as you continue using it to attack, and it can’t be sold, given away, melted for scrap iron, or the like."),
            moderate_boon_description: Some("Your blows become unstoppable, carrying the momentum of Gorum’s thrill for battle. Your greatsword Strikes gain the forceful trait."),
            major_boon_description: Some("Gorum feeds you the zeal of his undying warriors, allowing you to draw upon your own life force to fight on and on without falling. Whenever you would be reduced to 0 Hit Points, you are instead healed to half your maximum Hit Points and become doomed 1 (or increase your doomed condition by 1)."),
            minor_curse_description: Some("Gorum rewards cowardice with frailty. Any armor you wear and shield you wield reduces its item bonus to AC by 2 (minimum 0) and its Hardness, Hit Points, and Break Threshold by half."),
            moderate_curse_description: Some("You’ve lost the glory of slaying a worthy opponent. All of your weapon and unarmed attacks decrease their damage dice by one step, and all your attacks are nonlethal."),
            major_curse_description: Some("You are unable to keep up with the rigors of combat. The moment a combat breaks out, you become fatigued and slowed 1. At the end of each of your turns, your slowed condition increases by 1. These conditions end only when you are no longer in combat."),
            minor_boon: Some(gorum_minor_boon),
            moderate_boon: Some(gorum_moderate_boon),
            major_boon: Some(gorum_major_boon),
            minor_curse: Some(gorum_minor_curse),
            moderate_curse: Some(gorum_moderate_curse),
            major_curse: Some(gorum_major_curse),
        }
    }
    fn gozreh() -> Self {
        Self {
            alive: true,
            alternative_names: vec![
                "The Wind and the Waves",
                "Ioz'om, the Sky Father",
                "Hyjarth & Tourithia",
                "She Who Guides the Wind and the Waves",
                "Shimye-Magalla",
            ],
            description: "A timeless entity birthed from the first wind to stir the vast oceans, Gozreh wanders the world in the air and the seas. Sailors drop boxes of cargo as offerings to avoid a fatal storm, hoping to please the Wind and the Waves, even though they know that such pleas are far more likely to go unnoticed as they are to draw their deity’s attention. The deity’s temperament is fickle and their fury swift, hurling bolts of lightning and dragging to the crushing depths those who dare befoul the natural world. Gozreh is the sea that encapsulates the land and the wind that moves its surface, the birds that traverse the sky and the clouds that shield them.",
            adjective: vec!["Gorzen"],
            category: DeityCategories::GodOfTheInnerSea,
            edicts: vec!["cherish, protect, and respect nature in all its forms"],
            anathema: vec![
                "bring civilization to intrude on the wild",
                "create undead",
                "despoil areas of natural beauty"
            ],
            areas_of_concern: vec![
                DeityConcerns::Nature,
                DeityConcerns::Sea,
                DeityConcerns::Weather,
            ],
            pantheons: Some(vec![Pantheons::SeafarersHope]),
            realm: vec![Realm::Universe],
            worshippers: vec![
                People::Explorers,
                People::Fishers,
                People::Hermits,
                People::Survivalists,
                People::Sailors,
                People::Woodsmen,
                People::Farmers
            ],
            // todo druids
            ancestry_worshippers: None,
            symbol: vec!["Dripping leaf"],
            sacred_animal: Some(Animals::all()),
            favoured_animal: None,
            sacred_color: vec![Color::Blue, Color::Green],
            sacred_days: None,
            sacred_texts: vec![Books::HymnsToTheWindsAndWaves],
            divine_ability: vec![AbilityScores::Constitution, AbilityScores::Wisdom],
            divine_font: vec![Fonts::Healing],
            divine_sanction: None,
            divine_skill: Skills::Survival,
            favored_weapon: BaseWeapons::Trident,
            domains: vec![Domains::Air, Domains::Nature, Domains::Travel, Domains::Water],
            alternate_domains: Some(vec![Domains::Cold, Domains::Lightning]),
            cleric_spells: vec![Spells::GustOfWind, Spells::LightningBolt, Spells::ControlWater,],
            divine_intercession: Some("Gozreh is pleased when their creatures and waterways are treated with respect but quick to show their displeasure."),
            minor_boon_description: Some("Gozreh grants their guidance while at sea. You are under the constant effects of know direction and become trained in Sailing Lore (or another Lore skill if you are already trained in Sailing Lore)."),
            moderate_boon_description: Some("You gain the touch of the sea. You can breathe underwater and gain a swim Speed equal to your land Speed."),
            major_boon_description: Some("Gozreh has blessed you with their wind in your stride. You gain a +30-foot status bonus to your Speeds, and you gain a fly Speed equal to your land Speed."),
            minor_curse_description: Some("Lightning begins to strike twice. You gain weakness 5 to electricity, and any natural or magical bolts of lightning always target you rather than the other creatures around you."),
            moderate_curse_description: Some("The current of the waves constantly fights against you, putting you at risk of drowning any time you must Swim. You lose any swim Speed you have, unless it’s your only Speed, in which case you take a –20-foot status penalty to your swim Speed. When you roll an Athletics check to Swim, you always use the outcome for one degree of success worse than the result of your roll."),
            major_curse_description: Some("Those who displease Gozreh are shunned by nature itself. All animals and nonsapient plant creatures are hostile to you, and any animal companion or familiar abandons you. Sapient plants’ attitude toward you begins two categories worse than normal, as something about you seems repugnant to them. While plants and animals might not attack you outright if it’s not normally in their nature to do so when they are hostile, dogs growl at you, cats hiss, and so on."),
            minor_boon: Some(gozreh_minor_boon),
            moderate_boon: Some(gozreh_moderate_boon),
            major_boon: Some(gozreh_major_boon),
            minor_curse: Some(gozreh_minor_curse),
            moderate_curse: Some(gozreh_moderate_curse),
            major_curse: Some(gozreh_major_curse),
        }
    }
    fn iomedae() -> Self {
        Self {
            alive: true,
            alternative_names: vec!["The Inheritor", "Light of the Sword", "Lady of Valor"],
            description: "Iomedae, the youngest among the prominent deities of the Inner Sea region, had already proven herself worthy of divinity before her ascension. Born in Cheliax, she followed the path of the sword and fought evil, eventually becoming a paladin of Aroden’s herald Arazni. She became a legend among the Shining Crusade, leading the Knights of Ozem in a series of victories over the Whispering Tyrant. Iomedae became the third known mortal to pass the Test of the Starstone when she ascended to divinity in 3832 AR. As Arazni had been slain during the Shining Crusade, Aroden elevated the newly ascended goddess to be his new herald. When Aroden himself died, Iomedae inherited most of his worshippers and became a major deity of honor and justice.",
            adjective: vec!["Iomedaean"],
            category: DeityCategories::GodOfTheInnerSea,
            edicts: vec!["be temperate", "fight for justice and honor", "hold valor in your heart"],
            anathema: vec![
                "abandon a companion in need",
                "dishonor yourself",
                "refuse a challenge from an equal"
            ],
            areas_of_concern: vec![
                DeityConcerns::Honor,
                DeityConcerns::Justice,
                DeityConcerns::Rulership,
                DeityConcerns::Valor,
            ],
            pantheons: Some(vec![Pantheons::TheGodClaw]),
            realm: vec![Realm::Heaven, Realm::IomedaeRealm],
            worshippers: vec![
                People::Knights,
                People::Warriors,
            ],
            // todo paladins
            ancestry_worshippers: None,
            symbol: vec!["Sword and sun"],
            sacred_animal: Some(vec![Animals::Lion]),
            favoured_animal: Some(vec![
                Animals::Lion,
            ]
                .into_iter()
                .chain(Animals::eagles().into_iter())
                .chain(Animals::migratory_birds().into_iter())
                .collect::<Vec<_>>())
                .map(|mut v| { v.sort();v.dedup();v }),
            sacred_color: vec![Color::Red, Color::White],
            sacred_days: Some(vec![
                SacredDays::AscendanceDay,
                SacredDays::DayOfTheInheritor,
                SacredDays::FirstCrusaderDay,
                SacredDays::Armasse,
                SacredDays::RemembranceMoon,
            ]),
            sacred_texts: vec![Books::ActsOfIomedae],
            divine_ability: vec![AbilityScores::Strength, AbilityScores::Constitution],
            divine_font: vec![Fonts::Healing],
            divine_sanction: Some(vec![Holiness::Holy]),
            divine_skill: Skills::Intimidation,
            favored_weapon: BaseWeapons::Longsword,
            domains: vec![Domains::Confidence, Domains::Might, Domains::Truth, Domains::Zeal,],
            alternate_domains: Some(vec![Domains::Duty]),
            cleric_spells: vec![Spells::SureStrike, Spells::Enlarge, Spells::FireShield,],
            divine_intercession: Some("Iomedae grants her blessings to those who show valor in trying times."),
            minor_boon_description: Some("You always present yourself at your best. Your clothing and person are always clean and unrumpled, the metal of your blade and armor shining and unblemished. This doesn’t prevent you from being exposed to diseases and other afflictions via filth, but it protects you as well as if you had washed thoroughly right away."),
            moderate_boon_description: Some("Your heart beats with a determined valor. Once, Iomedae ends all negative effects affecting you, unless they are from an artifact, deity, or similarly powerful source; she also restores all lost Hit Points and replenishes your spells, Focus Points, and other daily resources."),
            major_boon_description: Some("Iomedae imbues your blade with great power. Longswords you wield gain the axiomatic, holy, and major striking runes while you hold them. These property runes count toward the number of runes you can have on your weapon, and if this would cause your weapon to exceed its limit, choose which ones to keep each day when you prepare. You gain a +2 status bonus to attack rolls with longswords."),
            minor_curse_description: Some("Your blade and armor have dulled like lead. Your weapons, armor, and shields have half their usual Hit Points and Break Threshold."),
            moderate_curse_description: Some("Tactical advantages never seem to work the way you planned. You can’t receive a circumstance bonus to your attack rolls, and enemies don’t take any circumstance penalties to their AC against your attacks."),
            major_curse_description: Some("All blades reject your wicked heart. Any weapon you wield automatically becomes broken after you Strike with it. This curse can’t break artifacts or similarly powerful weapons."),
            minor_boon: Some(iomedae_minor_boon),
            moderate_boon: Some(iomedae_moderate_boon),
            major_boon: Some(iomedae_major_boon),
            minor_curse: Some(iomedae_minor_curse),
            moderate_curse: Some(iomedae_moderate_curse),
            major_curse: Some(iomedae_major_curse),
        }
    }
    fn irori() -> Self {
        Self {
            alive: true,
            alternative_names: vec![
                "Master of Masters",
                "Iro-Shu",
                "The Enlightened One",
                "The Perfect Human",
                "The Perfect Man"
            ],
            description: "Irori exemplifies the concepts of self-perfection. His dogma states that he was a mortal who gained godhood through achieving a physical and mental state that surpassed mortal limitations. His followers seek to emulate their god’s divine state by perfecting themselves using the words of the Unbinding of Fetters, Irori’s sacred text. The illuminated pages of the tome detail numerous physical, spiritual, and mental exercises, as well as methods of learning and remembering.",
            adjective: vec!["Iroran"],
            category: DeityCategories::GodOfTheInnerSea,
            edicts: vec!["be humble",
                         "help others perfect themselves",
                         "hone your body, mind, and spirit to a more perfect state",
                         "practice discipline",
            ],
            anathema: vec![
                "engage in overly unhealthy or self-destructive behaviors",
                "destroy an important historical text",
                "repeatedly fail to maintain self-control"
            ],
            areas_of_concern: vec![
                DeityConcerns::Knowledge,
                DeityConcerns::History,
                DeityConcerns::SelfPerfection,
            ],
            pantheons: Some(vec![Pantheons::ThePillarsOfKnowledge, Pantheons::TheGodClaw]),
            realm: vec![Realm::Axis, Realm::SereneCircle],
            worshippers: vec![
                People::Mystics,
                People::Ascetics,
                People::Hermits,
                People::Historians,
                People::MartialArtists,
                People::Scholars,
            ],
            // todo vudrani
            // todo green dragons
            // todo monks
            ancestry_worshippers: Some(vec![Ancestries::Dwarf]),
            symbol: vec!["Open blue palm", "Master's Rebus"],
            sacred_animal: Some(vec![Animals::Snail]),
            favoured_animal: Some(vec![Animals::WhiteTiger, Animals::Mantis, Animals::Octopus]),
            sacred_color: vec![Color::Blue, Color::White],
            sacred_days: Some(vec![SacredDays::SelfDecided]),
            sacred_texts: vec![Books::UnbindingTheFetters],
            divine_ability: vec![AbilityScores::Intelligence, AbilityScores::Wisdom],
            divine_font: vec![Fonts::Healing, Fonts::Harmful],
            divine_sanction: Some(vec![Holiness::Holy, Holiness::Unholy]),
            divine_skill: Skills::Athletics,
            favored_weapon: BaseWeapons::Fist,
            domains: vec![Domains::Knowledge, Domains::Might, Domains::Perfection, Domains::Truth,],
            alternate_domains: Some(vec![Domains::Change, Domains::Vigil]),
            cleric_spells: vec![Spells::Jump, Spells::Haste, Spells::MountainResilience,],
            divine_intercession: Some("Irori grants his boons to those making progress on their paths toward perfection. He avoids bestowing misfortune as punishment, preferring to do so only to give an individual a challenging obstacle to overcome to help them progress in their quest for self-perfection."),
            minor_boon_description: Some("Irori grants you great insight and knowledge. Once, when you roll a failure at a check to Recall Knowledge, you get a critical success instead. Furthermore, the check loses the secret trait, so you know for sure that the result was a critical success. Irori typically grants this boon for an extremely consequential check to Recall Knowledge."),
            moderate_boon_description: Some("Your body rebuilds after adversity, becoming stronger. You can cast wholeness of body as an occult ki spell. If you didn't have one already, you gain a focus pool with 1 Focus Point and are trained in occult spell attack rolls and spell DCs (the sidebar on the monk class page has full information on what happens when you gain your first ki spell)."),
            major_boon_description: Some("You can temporarily ascend to a greater form. You can cast 8th-level righteous might once per day as an occult innate spell. When you do, your form appears unremarkable, rather than clad in powerful armaments."),
            minor_curse_description: Some("Irori challenges you to seek another path, rather than treat every problem as a nail just because you have a hammer. If you select a check for which you have a higher bonus when another method would have been more appropriate for the situation, such as using Deception to lie through life because it’s your highest modifier, even if it would be better to reach a compromise through Diplomacy, you must roll twice and take the lower result."),
            moderate_curse_description: Some("Irori challenges you to adapt to adversity in order to perfect yourself. You become weighed down as if under heavy weights, becoming clumsy 2 and encumbered until you accomplish a challenging task of Irori’s choosing."),
            major_curse_description: Some("Irori makes all living creatures forget your existence. This can be further compounded by raising Irori’s ire, resulting in your name being obliterated from all written records. In both cases, memories and writings rearrange themselves to omit you smoothly, rather than leaving obvious gaps."),
            minor_boon: Some(irori_minor_boon),
            moderate_boon: Some(irori_moderate_boon),
            major_boon: Some(irori_major_boon),
            minor_curse: Some(irori_minor_curse),
            moderate_curse: Some(irori_moderate_curse),
            major_curse: Some(irori_major_curse),
        }
    }
}

fn abadar_minor_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn abadar_moderate_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn abadar_major_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn abadar_minor_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn abadar_moderate_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn abadar_major_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn asmodeus_minor_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn asmodeus_moderate_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn asmodeus_major_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn asmodeus_minor_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn asmodeus_moderate_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn asmodeus_major_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn calistria_minor_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn calistria_moderate_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn calistria_major_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn calistria_minor_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn calistria_moderate_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn calistria_major_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn cayden_cailean_minor_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn cayden_cailean_moderate_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn cayden_cailean_major_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn cayden_cailean_minor_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn cayden_cailean_moderate_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn cayden_cailean_major_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn desna_minor_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn desna_moderate_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn desna_major_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn desna_minor_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn desna_moderate_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn desna_major_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn erastil_minor_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn erastil_moderate_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn erastil_major_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn erastil_minor_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn erastil_moderate_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn erastil_major_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn gorum_minor_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn gorum_moderate_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn gorum_major_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn gorum_minor_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn gorum_moderate_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn gorum_major_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn gozreh_minor_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn gozreh_moderate_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn gozreh_major_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn gozreh_minor_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn gozreh_moderate_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn gozreh_major_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn iomedae_minor_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn iomedae_moderate_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn iomedae_major_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn iomedae_minor_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn iomedae_moderate_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn iomedae_major_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn irori_minor_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn irori_moderate_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn irori_major_boon(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn irori_minor_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn irori_moderate_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}

fn irori_major_curse(player: &mut Player) -> anyhow::Result<()> {
    todo!("{:?}", player)
}
