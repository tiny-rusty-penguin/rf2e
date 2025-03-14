mod ancestry;
mod backgrounds;
mod deities;
mod language;
mod location;
mod spells;
mod animals;

use crate::ancestry::{Ancestries, Ancestry};
use crate::deities::Deities;
use crate::location::{Nations};
use clap::Parser;
use std::collections::HashMap;
use std::io;
use strum::{EnumIter, IntoEnumIterator};

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

#[derive(Debug)]
enum Proficiencies {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary,
}

#[derive(Debug, EnumIter)]
enum Skills {
    Acrobatics,
    Arcana,
    Athletics,
    Crafting,
    Deception,
    Diplomacy,
    Intimidation,
    Medicine,
    Nature,
    Occultism,
    Performance,
    Religion,
    Society,
    Stealth,
    Survival,
    Thievery,
}

struct Skill {
    ability_base: AbilityScores,
    proficiency: Proficiencies,
}

#[derive(Debug)]
enum LoreSkills {
    Art,
    Academia,
    Alcohol,
    City(Option<String>),
    Cooking,
    Circus,
    Deity(Option<Deities>),
    Engineering,
    Farming,
    FortuneTelling,
    Games,
    Genealogy,
    Gladiatorial,
    Guild,
    Heraldry,
    Herbalism,
    Labor,
    Legal,
    Mercantile,
    Mining,
    Sailing,
    Scribing,
    Tanning,
    Terrain(Option<Terrain>),
    Theater,
    Underworld,
    Warfare,
}

#[derive(Debug)]
enum Terrain {
    Aquatic,
    Arctic,
    Desert,
    Forest,
    Mountain,
    Plains,
    Sky,
    Swamp,
    Underground,
}

enum Traits {
    Dwarf,
    Humanoid,
    Elf,
    Gnome,
    Goblin,
    Halfling,
    Human,
    Leshy,
    Plant,
    Orc,
}

enum Rarity {
    Common,
    Uncommon,
    Rare,
    Unknown,
}

struct Size {
    space: u8,
    reach_tall: u8,
    reach_long: u8,
}

enum Holiness {
    Holy,
    Unholy,
}

enum Realm {
    FirstVault,
    Aktun,
    Axis,
    TheCatafalque,
    Nessus,
    Hell,
    TheGardensOfDeceitAndDelight,
    Elysium,
    CaydenCity,
    HerosHeart,
    ThePlaceSevenfoldCynosure,
    Summerlands,
    Requuius,
    Heaven,
    Universe,
    IomedaeRealm,
    SereneCircle,
}
enum People {
    Architects,
    Aristocrats,
    Bankers,
    Judges,
    Lawmakers,
    Lawyers,
    Merchants,
    Slavers,
    Bureaucrats,
    Tyrants,
    Diabolists,
    Hedonists,
    Performers,
    ScornedLovers,
    Thieves,
    Brewers,
    Vintners,
    Barkeeps,
    Innkeepers,
    Adventurers,
    Travelers,
    Astronomers,
    Gamblers,
    Varisians,
    Musicians,
    Farmers,
    Hunters,
    Tradesmen,
    Soldiers,
    Mercenaries,
    Brigands,
    Barbarians,
    Explorers,
    Fishers,
    Hermits,
    Survivalists,
    Sailors,
    Woodsmen,
    Knights,
    Warriors,
    Mystics,
    Ascetics,
    Historians,
    MartialArtists,
    Scholars,
}

enum Government {
    GrandCouncil,
    Vassal(Nations),
    Tribal,
    ConstitutionalMonarchy,
    ParliamentaryDemocracy,
    Monarchy,
    AutocraticCouncil,
    Mahajanapada,
    SecretSyndicate,
}
enum Individuals {
    Gyr,
    TerittaRicia,
    AndiraMarusek,
    EdrydArtume,
    PeiYaeMen,
    HarthwikBarzoni,
}





// todo ancestry


enum Color {
    Gold,
    Silver,
    Black,
    Red,
    Yellow,
    Tan,
    Blue,
    White,
    Brown,
    Green,
    Grey,
}

enum Books {
    TheOrderOfNumbers,
    TheManualOfCityBuilding,
    TheAsmodeanMonograph,
    TheBookOfJoy,
    BloodForWine,
    PlacardOfWisdom,
    TheEightScrolls,
    ShrineWallWritings,
    ParablesOfErastil,
    Gorumskagat,
    MythInIron,
    HymnsToTheWindsAndWaves,
    ActsOfIomedae,
    UnbindingTheFetters,
}

enum Months {
    Abadius,
    Calistril,
    Pharast,
    Gozran,
    Desnus,
    Sarenith,
    Erastus,
    Arodus,
    Rova,
    Lamashan,
    Neth,
    Kuthona,
}

struct Month {
    month: Months,
    name: &'static str,
    common_name: &'static str,
    days: u8,
    description: &'static str,
}

impl Month {
    fn all() -> Vec<Month> {
        vec![
            Self::abadius(),
            Self::calistril(),
            Self::pharast(),
            Self::gozran(),
            Self::desnus(),
            Self::sarenith(),
            Self::erastus(),
            Self::arodus(),
            Self::rova(),
            Self::lamashan(),
            Self::neth(),
            Self::kuthona(),
        ]
    }
    fn abadius() -> Self {
        Self {
            month: Months::Abadius,
            name: "Abadius",
            common_name: "Prima",
            days: 31,
            description: "The first month of the year, named in honor of Abadar.",
        }
    }
    fn calistril() -> Self {
        Self {
            month: Months::Calistril,
            name: "Calistril",
            common_name: "Snappe",
            days: 28,
            description: "A late winter month named for Calistria, goddess of revenge.",
        }
    }
    fn pharast() -> Self {
        Self {
            month: Months::Pharast,
            name: "Pharast",
            common_name: "Anu",
            days: 31,
            description:
                "An early spring month named after Pharasma, the goddess of birth and death.",
        }
    }
    fn gozran() -> Self {
        Self {
            month: Months::Gozran,
            name: "Gozran",
            common_name: "Rusanne",
            days: 30,
            description: "A stormy month named for the god of the wind, Gozreh.",
        }
    }
    fn desnus() -> Self {
        Self {
            month: Months::Desnus,
            name: "Desnus",
            common_name: "Farlong",
            days: 31,
            description: "A mild month named for the goddess Desna.",
        }
    }
    fn sarenith() -> Self {
        Self {
            month: Months::Sarenith,
            name: "Sarenith",
            common_name: "Sola",
            days: 30,
            description: "The sun goddess Sarenrae gives her name to this sun-blessed month.",
        }
    }
    fn erastus() -> Self {
        Self {
            month: Months::Erastus,
            name: "Erastus",
            common_name: "Fletch",
            days: 31,
            description: "Named in honor of Erastil.",
        }
    }
    fn arodus() -> Self {
        Self {
            month: Months::Arodus,
            name: "Arodus",
            common_name: "Hazen",
            days: 31,
            description: "Although he is no longer widely worshiped, the summer month is named for the dead god Aroden.",
        }
    }
    fn rova() -> Self {
        Self {
            month: Months::Rova,
            name: "Rova",
            common_name: "Nuvar",
            days: 30,
            description: "The beginning of autumn is named after the violent god Rovagug. (Dwarves dispute naming a month after such a destructive and evil deity and instead name the ninth month Torawsh after their creator god, Torag) ",
        }
    }
    fn lamashan() -> Self {
        Self {
            month: Months::Lamashan,
            name: "Lamashan",
            common_name: "Shaldo",
            days: 31,
            description: "Named for Lamashtu, the goddess of monsters.",
        }
    }
    fn neth() -> Self {
        Self {
            month: Months::Neth,
            name: "Neth",
            common_name: "Joya",
            days: 30,
            description: "Named for Nethys, the two-faced god of magic.",
        }
    }
    fn kuthona() -> Self {
        Self {
            month: Months::Kuthona,
            name: "Kuthona",
            common_name: "Kai",
            days: 31,
            description: "The shortest day of the year comes during the month named after the god of darkness, Zon-Kuthon.",
        }
    }
}

enum Days {
    Moonday,
    Toilday,
    Wealday,
    Oathday,
    Fireday,
    Starday,
    Sunday,
}

struct Date {
    month: Months,
    day: Days,
    year: Option<u16>,
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

#[derive(Debug)]
enum Specials {
    Darkvision,
    ClanDagger,
    LowLightVision,
    KeenEyes,
    PlantNourishment,
}

#[derive(Debug)]
struct Player {}

enum SkillFeats {
    AdditionalLore,
    Assurance(Option<Skills>),
    DubiousKnowledge,
    ExperiencedProfessional,
    QuickIdentification,
    RecognizeSpell,
    SkillTraining,
    TrickMagicItem,
    AdvancedFirstAid,
    AlchemicalCrafting,
    ArcaneSense,
    AutomaticKnowledge,
    BargainHunter,
    BattleCry,
    BattleMedicine,
    BizarreMagic,
    BondedAnimal,
    BreakCurse,
    CatFall,
    CharmingLiar,
    CloudJump,
    CombatClimber,
    CommunalCrafting,
    Confabulator,
    ContinualRecovery,
    CourtlyGraces,
    CraftAnything,
    DivineGuidance,
    ExperiencedSmuggler,
    ExperiencedTracker,
    FascinatingPerformance,
    FoilSenses,
    Forager,
    GladHand,
    GroupCoercion,
    GroupImpression,
    HeftyHauler,
    Hobnobber,
    ImpeccableCrafting,
    ImpressivePerformance,
    IntimidatingGlare,
    IntimidatingProwess,
    Inventor,
    KipUp,
    LastingCoercion,
    LegendaryCodebreaker,
    LegendaryLinguist,
    LegendaryMedic,
    LegendaryNegotiation,
    LegendaryPerformer,
    LegendaryProfessional,
    LegendarySneak,
    LegendarySurvivalist,
    LegendaryThief,
    LengthyDiversion,
    LieToMe,
    MagicalCrafting,
    MagicalShorthand,
    MonsterCrafting,
    Multilingual,
    NaturalMedicine,
    NimbleCrawl,
    NoCauseForAlarm,
    OddityIdentification,
    Pickpocket,
    PlanarSurvival,
    PowerfulLeap,
    QuickClimb,
    QuickCoercion,
    QuickDisguise,
    QuickJump,
    QuickRecognition,
    QuickRepair,
    QuickSqueeze,
    QuickSwim,
    QuickUnlock,
    QuietAllies,
    RapidMantel,
    ReadLips,
    RobustRecovery,
    ScareToDeath,
    SchooledInSecrets,
    Seasoned,
    ShamelessRequest,
    SignLanguage,
    SlipperySecrets,
    SpecialtyCrafting,
    SteadyBalance,
    Streetwise,
    StudentOfTheCanon,
    SubtleTheft,
    SurveyWildlife,
    SwiftSneak,
    TerrainExpertise(Option<Terrain>),
    TerrainStalker,
    TerrifiedRetreat,
    TitanWrestler,
    TrainAnimal,
    UnderwaterMarauder,
    UnifiedTheory,
    UnmistakableLore,
    UnusualTreatment,
    VirtuosicPerformer,
    WallJump,
    WardMedic,
    WaryDisarmament,
}

enum GeneralFeat {
    AdditionalLore,
    AdoptedAncestry,
    ArcaneSense,
    ArmorProficiency,
    Assurance,
    BreathControl,
    CannyAcumen,
    Diehard,
    DubiousKnowledge,
    ExperiencedProfessional,
    Fleet,
    IncredibleInitiative,
    Pet,
    QuickIdentification,
    RecognizeSpell,
    Ride,
    ShieldBlock,
    SkillTraining,
    Toughness,
    TrickMagicItem,
    WeaponProficiency,
    AncestralParagon,
    PrescientPlanner,
    UntrainedImprovisation,
    AlchemicalCrafting,
    BargainHunter,
    BattleMedicine,
    CatFall,
    CharmingLiar,
    CombatClimber,
    CourtlyGraces,
    ExperiencedSmuggler,
    ExperiencedTracker,
    FascinatingPerformance,
    FastRecovery,
    FeatherStep,
    Forager,
    GroupCoercion,
    GroupImpression,
    HeftyHauler,
    Hobnobber,
    ImpressivePerformance,
    IntimidatingGlare,
    LengthyDiversion,
    LieToMe,
    Multilingual,
    NaturalMedicine,
    NoCauseForAlarm,
    OddityIdentification,
    Pickpocket,
    QuickCoercion,
    QuickJump,
    QuickRepair,
    QuickSqueeze,
    ReadLips,
    SchooledInSecrets,
    Seasoned,
    SignLanguage,
    SpecialtyCrafting,
    SteadyBalance,
    Streetwise,
    StudentOfTheCanon,
    SubtleTheft,
    SurveyWildlife,
    TerrainExpertise,
    TerrainStalker,
    TitanWrestler,
    TrainAnimal,
    UnderwaterMarauder,
    VirtuosicPerformer,
    AutomaticKnowledge,
    BondedAnimal,
    CommunalCrafting,
    Confabulator,
    ContinualRecovery,
    GladHand,
    IntimidatingProwess,
    Inventor,
    LastingCoercion,
    MagicalCrafting,
    MagicalShorthand,
    NimbleCrawl,
    PowerfulLeap,
    QuickDisguise,
    QuietAllies,
    RapidMantel,
    RobustRecovery,
    UnmistakableLore,
    UnusualTreatment,
    WardMedic,
    WaryDisarmament,
    AdvancedFirstAid,
    BattleCry,
    BizarreMagic,
    BreakCurse,
    ExpeditiousSearch,
    FoilSenses,
    ImpeccableCrafting,
    KipUp,
    MonsterCrafting,
    PlanarSurvival,
    PrescientConsumable,
    QuickClimb,
    QuickRecognition,
    QuickSwim,
    QuickUnlock,
    ShamelessRequest,
    SlipperySecrets,
    SwiftSneak,
    TerrifiedRetreat,
    WallJump,
    IncredibleInvestiture,
    CloudJump,
    CraftAnything,
    DivineGuidance,
    LegendaryCodebreaker,
    LegendaryLinguist,
    LegendaryMedic,
    LegendaryNegotiation,
    LegendaryPerformer,
    LegendaryProfessional,
    LegendarySneak,
    LegendarySurvivalist,
    LegendaryThief,
    ScareToDeath,
    UnifiedTheory,
}

enum BardClassFeat {
    HymnOfHealing,
    ReachSpell,
    WellVersed,
    CantripExpansion,
    DirectedAudience,
    EmotionalPush,
    MultifariousMuse,
    BardicLore,
    LingeringComposition,
    MartialPerformance,
    VersatilePerformance,
    EsotericPolymath,
    LoremastersEtude,
    SongOfStrength,
    UpliftingOverture,
    CombatReading,
    CourageousAdvance,
    InTune,
    MelodiousSpell,
    RallyingAnthem,
    RitualResearcher,
    TripleTime,
    VersatileSignature,
    AssuredKnowledge,
    DefensiveCoordination,
    DirgeOfDoom,
    EducateAllies,
    Harmonize,
    SongOfMarching,
    SteadySpellcasting,
    Accompany,
    CallAndResponse,
    EclecticSkill,
    FortissimoComposition,
    KnowItAll,
    ReflexiveCourage,
    Soulsight,
    AnnotateComposition,
    CourageousAssault,
    HouseOfImaginaryWalls,
    OdeToOuroboros,
    QuickenedCasting,
    SymphonyOfTheUnfetteredHeart,
    UnusualComposition,
    EclecticPolymath,
    EnigmasKnowledge,
    InspirationalFocus,
    Reverberate,
    SharedAssault,
    Allegro,
    Earworm,
    SoothingBallad,
    TriumphantInspiration,
    TrueHypercognition,
    VigorousAnthem,
    CourageousOnslaught,
    EffortlessConcentration,
    ResoundingFinale,
    StudiousCapacity,
    AllInMyHead,
    DeepLore,
    DiscordantVoice,
    EternalComposition,
    ImpossiblePolymath,
    FatalAria,
    PerfectEncore,
    PiedPiping,
    SymphonyOfTheMuse,
    UltimatePolymath,
}

enum ClericClassFeat {
    DeadlySimplicity,
    DivineCastigation,
    DomainInitiate,
    PremonitionOfAvoidance,
    ReachSpell,
    CantripExpansion,
    CommunalHealing,
    EmblazonArmament,
    PanicTheDead,
    RapidResponse,
    SapLife,
    VersatileFont,
    HarmingHands,
    HealingHands,
    WarpriestsArmor,
    ChannelSmite,
    DirectedChannel,
    DivineInfusion,
    RaiseSymbol,
    RestorativeStrike,
    SacredGround,
    CastDown,
    DivineRebuttal,
    DivineWeapon,
    MagicHands,
    SelectiveEnergy,
    SteadySpellcasting,
    AdvancedDomain,
    CremateUndead,
    EmblazonEnergy,
    Martyr,
    RestorativeChannel,
    SanctifyArmament,
    SurgingFocus,
    VoidSiphon,
    ZealousRush,
    CastigatingWeapon,
    HeroicRecovery,
    ReplenishmentOfWar,
    SharedAvoidance,
    ShieldOfFaith,
    DefensiveRecovery,
    DomainFocus,
    EmblazonAntimagic,
    FortunateRelief,
    SappingSymbol,
    SharedReplenishment,
    ChannelingBlock,
    DeitysProtection,
    EbbAndFlow,
    FastChannel,
    LastingArmament,
    PremonitionOfClarity,
    SwiftBanishment,
    EternalBane,
    EternalBlessing,
    ReboundingSmite,
    Remediate,
    Resurrectionist,
    DivineApex,
    EchoingChannel,
    ImprovedSwiftBanishment,
    Inviolable,
    MiraculousPossibility,
    SharedClarity,
    AvatarsAudience,
    AvatarsProtection,
    MakerOfMiracles,
    SpellshapeChannel,
}

enum DruidClassFeat {
    AnimalEmpathy,
    PlantEmpathy,
    ReachSpell,
    VerdantWeapon,
    WidenSpell,
    CallOfTheWild,
    OrderExplorer,
    PoisonResistance,
    AnimalCompanion,
    LeshyFamiliar,
    StormBorn,
    UntamedForm,
    EnhancedFamiliar,
    AnthropomorphicShape,
    ElementalSummons,
    ForestPassage,
    FormControl,
    LeshyFamiliarSecrets,
    MatureAnimalCompanion,
    OrderMagic,
    SnowdriftSpell,
    CurrentSpell,
    GrownOfOak,
    InsectShape,
    InstinctiveSupport,
    SteadySpellcasting,
    StormRetribution,
    DeimaticDisplay,
    FerociousShape,
    FeyCaller,
    FloralRestoration,
    IncredibleCompanion,
    RaiseMenhir,
    SoaringShape,
    WindCaller,
    ElementalShape,
    HealingTransformation,
    OverwhelmingEnergy,
    PlantShape,
    PrimalHowl,
    PristineWeapon,
    SideBySide,
    ThunderclapSpell,
    DragonShape,
    GarlandSpell,
    PrimalFocus,
    PrimalSummons,
    WanderingOasis,
    ReactiveTransformation,
    SowSpell,
    SpecializedCompanion,
    TimelessNature,
    VerdantMetamorphosis,
    EffortlessConcentration,
    ImpalingBriars,
    MonstrosityShape,
    UpliftingWinds,
    InvokeDisaster,
    PerfectFormControl,
    PrimalAegis,
    HierophantsPower,
    LeyLineConduit,
    TrueShapeshifter,
}

enum FighterClassFeat {
    CombatAssessment,
    DoubleSlice,
    ExactingStrike,
    PointBlankStance,
    ReactiveShield,
    SnaggingStrike,
    SuddenCharge,
    ViciousSwing,
    AggressiveBlock,
    AssistingShot,
    BladeBrake,
    BrutishShove,
    CombatGrab,
    DuelingParry,
    IntimidatingStrike,
    LightningSwap,
    Lunge,
    ReboundingToss,
    SleekReposition,
    BarrelingCharge,
    DoubleShot,
    DualHandedAssault,
    PartingShot,
    PowerfulShove,
    QuickReversal,
    ShieldedStride,
    SlamDown,
    Swipe,
    TwinParry,
    AdvancedWeaponTraining,
    AdvantageousAssault,
    DazingBlow,
    DisarmingStance,
    FuriousFocus,
    GuardiansDeflection,
    ReflexiveShield,
    RevealingStab,
    RicochetStance,
    ShatterDefenses,
    ShieldWarden,
    TripleShot,
    BlindFight,
    DisorientingOpening,
    DuelingRiposte,
    FellingStrike,
    IncredibleAim,
    MobileShotStance,
    PositioningAssault,
    QuickShieldBlock,
    ResoundingBravery,
    SuddenLeap,
    AgileGrace,
    CertainStrike,
    CrashingSlam,
    CutFromTheAir,
    DebilitatingShot,
    DisarmingTwist,
    DisruptiveStance,
    FearsomeBrute,
    FlingingCharge,
    MirrorShield,
    OverpoweringCharge,
    TacticalReflexes,
    TwinRiposte,
    BrutalFinish,
    DashingStrike,
    DuelingDance,
    FlingingShove,
    ImprovedDuelingRiposte,
    IncredibleRicochet,
    LungingStance,
    ParagonsGuard,
    DesperateFinisher,
    Determination,
    GuidingFinish,
    GuidingRiposte,
    ImprovedTwinRiposte,
    OpeningStance,
    TwoWeaponFlurry,
    WhirlwindStrike,
    GracefulPoise,
    ImprovedReflexiveShield,
    MasterOfManyStyles,
    MultishotStance,
    OverwhelmingBlow,
    TwinnedDefense,
    ImpossibleVolley,
    SavageCritical,
    SmashFromTheAir,
    BoundlessReprisals,
    UltimateFlexibility,
    WeaponSupremacy,
}

enum RangerClassFeat {
    AnimalCompanion,
    CrossbowAce,
    HuntedShot,
    InitiateWarden,
    MonsterHunter,
    TwinTakedown,
    AnimalEmpathy,
    FavoredTerrain,
    HuntersAim,
    MonsterWarden,
    QuickDraw,
    AdvancedWarden,
    CompanionsCry,
    DisruptPrey,
    FarShot,
    FavoredPrey,
    RunningReload,
    ScoutsWarning,
    TwinParry,
    AdditionalRecollection,
    MasterfulWarden,
    MatureAnimalCompanion,
    SkirmishStrike,
    SnapShot,
    SwiftTracker,
    BlindFight,
    DeadlyAim,
    HazardFinder,
    TerrainMaster,
    WardensBoon,
    Camouflage,
    IncredibleCompanion,
    MasterMonsterHunter,
    PeerlessWarden,
    PenetratingShot,
    TwinRiposte,
    WardensStep,
    DistractingShot,
    DoublePrey,
    SecondSting,
    SideBySide,
    WardensFocus,
    SenseTheUnseen,
    SharedPrey,
    StealthyCompanion,
    WardensGuidance,
    GreaterDistractingShot,
    ImprovedTwinRiposte,
    LegendaryMonsterHunter,
    SpecializedCompanion,
    WardensReload,
    ImpossibleFlurry,
    ImpossibleVolley,
    ManifoldEdge,
    MasterfulCompanion,
    PerfectShot,
    ShadowHunter,
    LegendaryShot,
    ToTheEndsOfTheEarth,
    TripleThreat,
    UltimateSkirmisher,
}

enum RougeClassFeat {
    NimbleDodge,
    TrapFinder,
    TumbleBehind,
    TwinFeint,
    OverextendingFeint,
    PlantEvidence,
    YoureNext,
    BrutalBeating,
    CleverGambit,
    DistractingFeint,
    Mobility,
    QuickDraw,
    StrongArm,
    UnbalancingBlow,
    UnderhandedAssault,
    DreadStriker,
    HeadStomp,
    Mug,
    PoisonWeapon,
    Predictable,
    ReactivePursuit,
    Sabotage,
    ScoundrelsSurprise,
    ScoutsWarning,
    TheHarderTheyFall,
    TwinDistraction,
    AnalyzeWeakness,
    AnticipateAmbush,
    FarThrow,
    GangUp,
    LightStep,
    ShoveDown,
    SkirmishStrike,
    SlyDisarm,
    TwistTheKnife,
    WatchYourBack,
    BlindFight,
    Bullseye,
    DelayTrap,
    ImprovedPoisonWeapon,
    InspiredStratagem,
    NimbleRoll,
    OpportuneBackstab,
    PredictivePurchase,
    RicochetStance,
    Sidestep,
    SlyStriker,
    SwipeSouvenir,
    TacticalEntry,
    MethodicalDebilitations,
    NimbleStrike,
    PreciseDebilitations,
    SneakAdept,
    TacticalDebilitations,
    ViciousDebilitations,
    BloodyDebilitation,
    CriticalDebilitation,
    FantasticLeap,
    FellingShot,
    Preparation,
    ReactiveInterference,
    RicochetFeint,
    SpringFromTheShadows,
    DefensiveRoll,
    InstantOpening,
    LeaveAnOpening,
    SenseTheUnseen,
    StayDown,
    BlankSlate,
    CloudStep,
    CognitiveLoophole,
    DispellingSlice,
    PerfectDistraction,
    ReconstructTheScene,
    SwiftElusion,
    ImplausibleInfiltration,
    ImplausiblePurchase,
    PowerfulSneak,
    HiddenParagon,
    ImpossibleStriker,
    ReactiveDistraction,
}

enum WitchClassFeat {
    Cackle,
    Cauldron,
    Counterspell,
    ReachSpell,
    WidenSpell,
    WitchsArmaments,
    BasicLesson,
    CantripExpansion,
    ConcealSpell,
    EnhancedFamiliar,
    FamiliarsLanguage,
    RitesOfConvocation,
    SympatheticStrike,
    CeremonialKnife,
    GreaterLesson,
    SteadySpellcasting,
    WitchsCharge,
    IncredibleFamiliar,
    Murksight,
    SpiritFamiliar,
    StitchedFamiliar,
    WitchsBottle,
    DoubleDouble,
    MajorLessonI,
    QuickenedCasting,
    WitchsCommunion,
    CovenSpell,
    HexFocus,
    WitchsBroom,
    MajorLessonII,
    PatronsPresence,
    ReflectSpell,
    RitesOfTransfiguration,
    EffortlessConcentration,
    SiphonPower,
    MajorLessonIII,
    PatronsClaim,
    SplitHex,
    HexMaster,
    PatronsTruth,
    WitchsHut,
}

enum WizardClassFeat {
    Counterspell,
    Familiar,
    ReachSpell,
    SpellbookProdigy,
    WidenSpell,
    CantripExpansion,
    ConcealSpell,
    EnergyAblation,
    NonlethalSpell,
    EnhancedFamiliar,
    BespellStrikes,
    CallWizardlyTools,
    LinkedFocus,
    SpellProtectionArray,
    ConvincingIllusion,
    ExplosiveArrival,
    IrresistibleMagic,
    SplitSlot,
    SteadySpellcasting,
    AdvancedSchoolSpell,
    BondConservation,
    FormRetention,
    KnowledgeIsPower,
    OverwhelmingEnergy,
    QuickenedCasting,
    ScrollAdept,
    CleverCounterspell,
    ForcibleEnergy,
    KeenMagicalDetection,
    MagicSense,
    BondedFocus,
    ReflectSpell,
    SecondaryDetonationArray,
    SuperiorBond,
    EffortlessConcentration,
    ScintillatingSpell,
    SpellTinker,
    InfinitePossibilities,
    ReprepareSpell,
    SecondThoughts,
    ArchwizardsMight,
    SpellCombination,
    SpellMastery,
    SpellshapeMastery,
}

enum DwarfAncestryFeats {
    DwarvenDoughtiness,
    DwarvenLore,
    DwarvenWeaponFamiliarity,
    MountainStrategy,
    RockRunner,
    StonemasonsEye,
    UnburdenedIron,
    BoulderRoll,
    DefyTheDarkness,
    DwarvenReinforcement,
    EchoesInStone,
    MountainsStoutness,
    StoneBones,
    Stonewalker,
    MarchTheMines,
    TelluricPower,
    Stonegate,
    Stonewall,
}
enum ElfAncestryFeats {
    AncestralLongevity,
    ElvenLore,
    ElvenWeaponFamiliarity,
    Forlorn,
    NimbleElf,
    OtherworldlyMagic,
    UnwaveringMien,
    AgelessPatience,
    AncestralSuspicion,
    MartialExperience,
    ElfStep,
    ExpertLongevity,
    OtherworldlyAcumen,
    TreeClimber,
    AvengeAlly,
    UniversalLongevity,
    MagicRider,
}
enum GnomeAncestryFeats {
    AnimalAccomplice,
    AnimalElocutionist,
    FeyFellowship,
    FirstWorldMagic,
    GnomeObsession,
    GnomeWeaponFamiliarity,
    IllusionSense,
    RazzleDazzle,
    EnergizedFont,
    ProjectPersona,
    CautiousCuriosity,
    FirstWorldAdept,
    LifeLeap,
    VivaciousConduit,
    InstinctiveObfuscation,
    HomewardBound,
}

enum GoblinAncestryFeats {
    BurnIt,
    CityScavenger,
    GoblinLore,
    GoblinScuttle,
    GoblinSong,
    GoblinWeaponFamiliarity,
    JunkTinker,
    RoughRider,
    VerySneaky,
    Kneecap,
    LoudSinger,
    Vandal,
    CaveClimber,
    Cling,
    SkitteringScuttle,
    VeryVerySneaky,
    RecklessAbandon,
}

enum HalflingAncestryFeats {
    DistractingShadows,
    FolksyPatter,
    HalflingLore,
    HalflingLuck,
    HalflingWeaponFamiliarity,
    PrairieRider,
    SureFeet,
    TitanSlinger,
    UnfetteredHalfling,
    WatchfulHalfling,
    CulturalAdaptability,
    StepLively,
    DanceUnderfoot,
    GuidingLuck,
    Irrepressible,
    UnhamperedPassage,
    CeaselessShadows,
    TopplingDance,
    ShadowSelf,
}

enum HumanAncestryFeats {
    AdaptedCantrip,
    CooperativeNature,
    GeneralTraining,
    HaughtyObstinacy,
    NaturalAmbition,
    NaturalSkill,
    UnconventionalWeaponry,
    AdaptiveAdept,
    CleverImproviser,
    SenseAllies,
    CooperativeSoul,
    GroupAid,
    HardyTraveler,
    IncredibleImprovisation,
    Multitalented,
    AdvancedGeneralTraining,
    BounceBack,
    StubbornPersistence,
    HeroicPresence,
}

enum LeshyAncestryFeats {
    GraspingReach,
    HarmlesslyCute,
    LeshyLore,
    LeshySuperstition,
    Seedpod,
    ShadowOfTheWilds,
    Undaunted,
    AnchoringRoots,
    LeshyGlide,
    RitualReversion,
    SpeakWithKindred,
    BarkAndTendril,
    LuckyKeepsake,
    SolarRejuvenation,
    ThornedSeedpod,
    CallOTheGreenMan,
    CloakOfPoison,
    FlourishAndRuin,
    Regrowth,
}

enum OrcAncestryFeats {
    BeastTrainer,
    HoldMark,
    IronFists,
    OrcFerocity,
    OrcLore,
    OrcSuperstition,
    OrcWeaponFamiliarity,
    Tusks,
    AthleticMight,
    BloodyBlows,
    DefyDeath,
    ScarThickSkin,
    PervasiveSuperstition,
    UndyingFerocity,
    FerociousBeasts,
    IncredibleFerocity,
    SpellDevourer,
    RampagingFerocity,
}

enum ChangelingAncestryFeats {
    BrineMay,
    CallowMay,
    ChangelingLore,
    DreamMay,
    HagClaws,
    HagsSight,
    SlagMay,
    Called,
    MistChild,
    AccursedClaws,
    OccultResistance,
    HagMagic,
}

enum NephilimAncestryFeats {
    AngelKin,
    BestialManifestation,
    Grimspawn,
    Halo,
    Hellspawn,
    Lawbringer,
    Musetouched,
    NephilimEyes,
    NephilimLore,
    NimbleHooves,
    Pitborn,
    BlessedBlood,
    ExtraplanarSupplication,
    NephilimResistance,
    ScionOfManyPlanes,
    SkillfulTail,
    CelestialMagic,
    DivineWings,
    FiendishMagic,
    CelestialMercy,
    SlipSideways,
    SummonNephilimKin,
    DivineDeclaration,
    EternalWings,
}

enum Classes {
    Bard,
    Cleric,
    Druid,
    Fighter,
    Ranger,
    Rogue,
    Witch,
    Wizard,
}

enum SavingThrows {
    Fortitude,
    Reflex,
    Will,
}

struct SavingThrow {
    saving_throw: SavingThrows,
    proficiency: Proficiencies,
}

enum WeaponProficiencies {
    Unarmed,
    Simple,
    Martial,
    Advanced,
}

struct WeaponProficiency {
    weapon_proficiency: WeaponProficiencies,
    proficiency: Proficiencies,
}

enum Weapons {
    AlchemicalBomb,
    Arbalest,
    BastardSword,
    BattleAxe,
    Blowgun,
    BoStaff,
    Bola,
    ClanDagger,
    Club,
    CompositeLongbow,
    CompositeShortbow,
    Crossbow,
    Dagger,
    Dart,
    Dogslicer,
    DwarvenWarAxe,
    ElvenCurveBlade,
    Falchion,
    FilchersFork,
    Fist,
    Flail,
    Gauntlet,
    Glaive,
    GnomeFlickmace,
    GnomeHookedHammer,
    Greataxe,
    Greatclub,
    Greatpick,
    Greatsword,
    Guisarme,
    Halberd,
    HalflingSlingStaff,
    HandCrossbow,
    Hatchet,
    HeavyCrossbow,
    Horsechopper,
    Javelin,
    Kama,
    Katana,
    Katar,
    Khakkara,
    Kukri,
    Lance,
    LightHammer,
    LightMace,
    LightPick,
    Longbow,
    Longspear,
    Longsword,
    Mace,
    MainGauche,
    Maul,
    Morningstar,
    Nunchaku,
    OrcKnuckleDagger,
    OrcNecksplitter,
    Pick,
    Ranseur,
    Rapier,
    Sai,
    Sap,
    SawtoothSaber,
    Scimitar,
    Scythe,
    Seedpod,
    ShieldBash,
    ShieldBoss,
    ShieldSpikes,
    Shortbow,
    Shortsword,
    Shuriken,
    Sickle,
    Sling,
    Spear,
    SpecialUnarmed,
    SpecialUnarmedCraneWing,
    SpecialUnarmedDragonTail,
    SpecialUnarmedFallingStone,
    SpecialUnarmedIronSweep,
    SpecialUnarmedLashingBranch,
    SpecialUnarmedTigerClaw,
    SpecialUnarmedWindCrash,
    SpecialUnarmedWolfJaw,
    SpikedChain,
    SpikedGauntlet,
    Staff,
    Starknife,
    TempleSword,
    Trident,
    Wakizashi,
    WarFlail,
    Warhammer,
    Whip,
    BloodlettingKukri,
    CaterwaulSling,
    ChaplainsCudgel,
    FightersFork,
    GloomBlade,
    HuntersAnthem,
    Icicle,
    RetributionAxe,
    SearingBlade,
    SearingBladeGreater,
    SerpentDagger,
    SkyHammer,
    StormFlash,
    StormFlashGreater,
    TwiningStaff,
}

enum BaseWeapons {
    AlchemicalBomb,
    Arbalest,
    BastardSword,
    BattleAxe,
    Blowgun,
    BoStaff,
    Bola,
    ClanDagger,
    Club,
    CompositeLongbow,
    CompositeShortbow,
    Crossbow,
    Dagger,
    Dart,
    Dogslicer,
    DwarvenWarAxe,
    ElvenCurveBlade,
    Falchion,
    FilchersFork,
    Fist,
    Flail,
    Gauntlet,
    Glaive,
    GnomeFlickmace,
    GnomeHookedHammer,
    Greataxe,
    Greatclub,
    Greatpick,
    Greatsword,
    Guisarme,
    Halberd,
    HalflingSlingStaff,
    HandCrossbow,
    Hatchet,
    HeavyCrossbow,
    Horsechopper,
    Javelin,
    Kama,
    Katana,
    Katar,
    Khakkara,
    Kukri,
    Lance,
    LightHammer,
    LightMace,
    LightPick,
    Longbow,
    Longspear,
    Longsword,
    Mace,
    MainGauche,
    Maul,
    Morningstar,
    Nunchaku,
    OrcKnuckleDagger,
    OrcNecksplitter,
    Pick,
    Ranseur,
    Rapier,
    Sai,
    Sap,
    SawtoothSaber,
    Scimitar,
    Scythe,
    Seedpod,
    ShieldBash,
    ShieldBoss,
    ShieldSpikes,
    Shortbow,
    Shortsword,
    Shuriken,
    Sickle,
    Sling,
    Spear,
    SpecialUnarmed,
    SpecialUnarmedCraneWing,
    SpecialUnarmedDragonTail,
    SpecialUnarmedFallingStone,
    SpecialUnarmedIronSweep,
    SpecialUnarmedLashingBranch,
    SpecialUnarmedTigerClaw,
    SpecialUnarmedWindCrash,
    SpecialUnarmedWolfJaw,
    SpikedChain,
    SpikedGauntlet,
    Staff,
    Starknife,
    TempleSword,
    Trident,
    Wakizashi,
    WarFlail,
    Warhammer,
    Whip,
    BloodlettingKukri,
    CaterwaulSling,
    ChaplainsCudgel,
    FightersFork,
    GloomBlade,
    HuntersAnthem,
    Icicle,
    RetributionAxe,
    SearingBlade,
    SearingBladeGreater,
    SerpentDagger,
    SkyHammer,
    StormFlash,
    StormFlashGreater,
    TwiningStaff,
}
enum ArmorProficiencies {
    Unarmored,
    Light,
    Medium,
    Heavy,
}

enum Armors {
    Breastplate,
    ChainMail,
    ChainShirt,
    ElvenChain,
    ExplorersClothing,
    FullPlate,
    HalfPlate,
    Hide,
    Leather,
    PaddedArmor,
    ScaleMail,
    SplintMail,
    StuddedLeather,
    Unarmored,
    BandsOfForce,
    BandsOfForceGreater,
    BandsOfForceMajor,
    ElectricEelskin,
    ImpenetrableScale,
    LifeSaverMail,
    LifeSaverMailGreater,
    LionsArmor,
    LionsArmorGreater,
    MoonlitChain,
    Tideplate,
}

enum Shields {
    Buckler,
    SteelShield,
    TowerShield,
    WoodenShield,
    ColdIronBucklerLowGrade,
    ColdIronShieldLowGrade,
    SilverBucklerLowGrade,
    SilverShieldLowGrade,
    SturdyShieldMinor,
    SpellguardShield,
    LionsShield,
    SturdyShieldLesser,
    SpinedShield,
    ColdIronBucklerStandardGrade,
    ColdIronShieldStandardGrade,
    SilverBucklerStandardGrade,
    SilverShieldStandardGrade,
    AdamantineBucklerStandardGrade,
    AdamantineShieldStandardGrade,
    DawnsilverBucklerStandardGrade,
    DawnsilverShieldStandardGrade,
    DuskwoodBucklerStandardGrade,
    DuskwoodShieldStandardGrade,
    DuskwoodTowerShieldStandardGrade,
    DragonslayersShield,
    ForceShield,
    ForgeWarden,
    SturdyShieldModerate,
    FloatingShield,
    LoadstoneShield,
    SturdyShieldGreater,
    ColdIronBucklerHighGrade,
    ColdIronShieldHighGrade,
    SilverBucklerHighGrade,
    SturdyShieldMajor,
    AdamantineBucklerHighGrade,
    AdamantineShieldHighGrade,
    DawnsilverBucklerHighGrade,
    DawnsilverShieldHighGrade,
    DuskwoodBucklerHighGrade,
    DuskwoodShieldHighGrade,
    DuskwoodTowerShieldHighGrade,
    OrichalcumBucklerHighGrade,
    OrichalcumShieldHighGrade,
    IndestructibleShield,
    ReflectingShield,
    SturdyShieldSupreme,
}

enum WeaponRunes {
    Potency,
    Striking,

    Returning,
    GhostTouch,
    Fearsome,
    Vitalizing,
    Shifting,
    Wounding,
    Corrosive,
    Flaming,
    Frost,
    Shock,
    Thundering,
    Astral,
    Decaying,
    Grievous,
    Extending,
    Holy,
    Unholy,
    FearsomeGreater,
    Brilliant,
    Keen,
    ExtendingGreater,
    Animated,
    Shockwave,
    SpellReservoir,
    VitalizingGreater,
    CorrosiveGreater,
    FlamingGreater,
    FrostGreater,
    ShockGreater,
    ThunderingGreater,
    AstralGreater,
    DecayingGreater,
    Quickstrike,
    Vorpal,
    BrilliantGreater,
}

enum ArmorRunes {
    Potency,
    Resilient,

    Shadow,
    Slick,
    Raiment,
    Ready,
    SizeChanging,
    Invisibility,
    SlickGreater,
    EnergyResistantAcid,
    EnergyResistantCold,
    EnergyResistantElectricity,
    EnergyResistantFire,
    ShadowGreater,
    InvisibilityGreater,
    ReadyGreater,
    Fortification,
    EnergyResistantGreaterAcid,
    EnergyResistantGreaterCold,
    EnergyResistantGreaterElectricity,
    EnergyResistantGreaterFire,
    Winged,
    Antimagic,
    SlickMajor,
    ShadowMajor,
    FortificationGreater,
    WingedGreater,
}

enum ShieldRunes {
    ReinforcingMinor,
    ReinforcingLesser,
    ReinforcingModerate,
    ReinforcingGreater,
    ReinforcingMajor,
    ReinforcingSupreme,
}

struct ClassLevelInfo {
    saving_throws: Vec<SavingThrow>,
    skills: HashMap<Option<Skills>, Option<Proficiencies>>,
    weapon_proficiencies: HashMap<WeaponProficiency, Proficiencies>,
    class_dc_proficiency: Proficiencies,
    spell_dc_proficiency: Proficiencies,
}

struct Class {
    class: Classes,
    key_ability: AbilityScores,
    hit_points: u8,
    level_progression: HashMap<u8, ClassLevelInfo>,
}

enum DwarfHeritages {
    AncientBlooded,
    DeathWardenDwarf,
    ForgeDwarf,
    RockDwarf,
    StrongBloodedDwarf,
}

enum ElfHeritages {
    AncientElf,
    ArcticElf,
    CavernElf,
    SeerElf,
    WhisperElf,
    WoodlandElf,
}

enum GnomeHeritages {
    ChameleonGnome,
    FeyTouchedGnome,
    SensateGnome,
    UmbralGnome,
    WellspringGnome,
}

enum GoblinHeritages {
    CharhideGoblin,
    IrongutGoblin,
    RazortoothGoblin,
    SnowGoblin,
    UnbreakableGoblin,
}

enum HalflingHeritages {
    GutsyHalfling,
    HillockHalfling,
    NomadicHalfling,
    TwilightHalfling,
    WildwoodHalfling,
}

enum HumanHeritages {
    SkilledHuman,
    VersatileHuman,
}

enum LeshyHeritages {
    CactusLeshy,
    FruitLeshy,
    FungusLeshy,
    GourdLeshy,
    LeafLeshy,
    LotusLeshy,
    RootLeshy,
    SeaweedLeshy,
    VineLeshy,
}

enum OrcHeritages {
    BadlandsOrc,
    BattleReady,
    DeepOrc,
    GraveOrc,
    HoldScarredOrc,
    RainfallOrc,
    WinterOrc,
}

enum UniversalHeritages {
    Changeling,
    Nephilim,
}

enum HalfHeritages {
    Dwarf,
    Aiuvarin,
    Gnome,
    Goblin,
    Halfling,
    Human,
    Leshy,
    Dromaar,
}

enum HeritageType {
    DwarfHeritage(DwarfHeritages),
    ElfHeritage(ElfHeritages),
    GnomeHeritage(GnomeHeritages),
    GoblinHeritage(GoblinHeritages),
    HalflingHeritage(HalflingHeritages),
    HumanHeritage(HumanHeritages),
    LeshyHeritage(LeshyHeritages),
    OrcHeritage(OrcHeritages),
    UniversalHeritage(UniversalHeritages),
    HalfHeritage(HalfHeritages),
}

enum HeritageFeat {}

enum AncestryFeat {}

struct Heritage {
    heritage: HeritageType,
    heritage_feat: HeritageFeat,
    description: &'static str,
}

enum Domains {
    Abomination,
    Air,
    Ambition,
    Change,
    Cities,
    Cold,
    Confidence,
    Creation,
    Darkness,
    Death,
    Decay,
    Delirium,
    Destruction,
    Dreams,
    Dust,
    Duty,
    Earth,
    Family,
    Fate,
    Fire,
    Freedom,
    Glyph,
    Healing,
    Indulgence,
    Introspection,
    Knowledge,
    Lightning,
    Luck,
    Magic,
    Metal,
    Might,
    Moon,
    Naga,
    Nature,
    Nightmares,
    Pain,
    Passion,
    Perfection,
    Plague,
    Protection,
    Repose,
    Secrecy,
    Sorrow,
    Soul,
    Star,
    Sun,
    Swarm,
    Time,
    Toil,
    Travel,
    Trickery,
    Truth,
    Tyranny,
    Undeath,
    Vigil,
    Void,
    Water,
    Wealth,
    Wood,
    Wyrmkin,
    Zeal,
}

enum ClassSpecials {
    Muse,
    Font,
    Doctrine,
    Order,
    Edge,
    Racket,
    HexSpell,
    Patron,
    ArcaneSchool,
    ArcaneThesis,
}

enum Muses {
    Enigma,
    Maestro,
    Polymath,
    Warrior,
}

enum Fonts {
    Harmful,
    Healing,
}

enum Doctrines {
    CloisteredCleric,
    WarPriest,
}

enum Orders {
    Animal,
    Leaf,
    Storm,
    Untamed,
}

enum Edges {
    Flurry,
    Outwit,
    Precision,
}

enum Rackets {
    Mastermind,
    Ruffian,
    Scoundrel,
    Thief,
}

enum HexSpells {
    PatronsPuppet,
    PhaseFamiliar,
}

enum Patrons {
    FaithsFlamekeeper,
    SilenceInSnow,
    SpinnerOfThreads,
    StarlessShadow,
    TheInscribedOne,
    TheResentment,
    WildingSteward,
}
enum ArcaneSchools {
    SchoolOfArsGrammatica,
    SchoolOfBattleMagic,
    SchoolOfCivicWizardry,
    SchoolOfMentalism,
    SchoolOfProteanForm,
    SchoolOfUnifiedMagicalTheory,
    SchoolOfTheBoundary,
}

enum ArcaneThese {
    ExperimentalSpellshaping,
    ImprovedFamiliarAttunement,
    SpellBlending,
    SpellSubstitution,
    StaffNexus,
}

impl ClassSpecials {
    fn associated_class(class: Classes) -> Option<Vec<ClassSpecials>> {
        match class {
            Classes::Bard => Some(vec![ClassSpecials::Muse]),
            Classes::Cleric => Some(vec![ClassSpecials::Font, ClassSpecials::Doctrine]),
            Classes::Druid => Some(vec![ClassSpecials::Order]),
            Classes::Fighter => None,
            Classes::Ranger => Some(vec![ClassSpecials::Edge]),
            Classes::Rogue => Some(vec![ClassSpecials::Racket]),
            Classes::Witch => Some(vec![ClassSpecials::HexSpell, ClassSpecials::Patron]),
            Classes::Wizard => Some(vec![
                ClassSpecials::ArcaneSchool,
                ClassSpecials::ArcaneThesis,
            ]),
        }
    }
}

struct ClassSpecial {}

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

fn select_ancestry() -> anyhow::Result<Ancestry> {
    loop {
        print_separator();
        for ancestry in Ancestries::iter() {
            println!("{:?}", ancestry);
        }
        let ancestry = Ancestry::str_to_ancestry(
            get_input("Please Select an Ancestry:")?
                .to_lowercase()
                .as_str(),
        );
        match ancestry {
            Some(_) => return Ok(ancestry.unwrap()),
            None => {
                println!("Invalid ancestry. Please try again.");
            }
        }
    }
}

fn select_free_boosts(mut ancestry: Ancestry, num_of_boosts: u8) -> anyhow::Result<Ancestry> {
    print_separator();
    let all_abilities = AbilityScores::all();
    let available_boosts: Vec<AbilityScores> = all_abilities
        .iter()
        .filter(|ability| {
            !ancestry.ability_boosts.contains(ability) && !ancestry.ability_flaws.contains(ability)
        })
        .cloned()
        .collect();
    for boost in available_boosts {
        println!("{:?}", boost);
    }
    loop {
        let new_boost = get_input(
            format!(
                "Please Select an Ability Boost ({} boosts available)",
                num_of_boosts
            )
            .as_str(),
        )?;
        match new_boost.as_str() {
            "strength" | "str" => {
                ancestry.ability_boosts.push(AbilityScores::Strength);
                return Ok(ancestry);
            }
            "dexterity" | "dex" => {
                ancestry.ability_boosts.push(AbilityScores::Dexterity);
                return Ok(ancestry);
            }
            "constitution" | "con" => {
                ancestry.ability_boosts.push(AbilityScores::Constitution);
                return Ok(ancestry);
            }
            "intelligence" | "int" => {
                ancestry.ability_boosts.push(AbilityScores::Intelligence);
                return Ok(ancestry);
            }
            "wisdom" | "wis" => {
                ancestry.ability_boosts.push(AbilityScores::Wisdom);
                return Ok(ancestry);
            }
            "charisma" | "cha" => {
                ancestry.ability_boosts.push(AbilityScores::Charisma);
                return Ok(ancestry);
            }
            _ => {
                println!("Invalid Boost. Please try again.");
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.create_character {
        let mut ancestry = select_ancestry()?;
        let num_of_boosts = ancestry.free_boosts;
        for i in 0..num_of_boosts {
            ancestry = select_free_boosts(ancestry, num_of_boosts - i)?;
        }
    }
    Ok(())
}
