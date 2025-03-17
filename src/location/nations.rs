use std::vec;

use crate::deities::Deities;
use crate::language::Languages;
use crate::location::areas::{Continents, MajorAreas};
use crate::location::cities::Settlements;
use crate::{Government, Individuals, NationType};

pub enum Nations {
    // Avistan
    Absalom,
    Alban,
    Andoran,
    Artume,
    Bakrakhan,
    HoldOfBelkzen,
    Brevoy,
    Cheliax,
    Cordelon,
    Corvenn,
    Cyrusian,
    Daggermark,
    Doggadth,
    DroskarsKingdom,
    Druma,
    EchoWood,
    Edasseril,
    Eurythnia,
    FiveKingsMountains,
    Galt,
    Gardadth,
    Gastash,
    Grakodan,
    Gralton,
    Gravelands,
    Haruka,
    Hermea,
    Hymbria,
    Irrisen,
    Isger,
    Issia,
    Jelheg,
    JistkaImperium,
    KingdomOfZog,
    Kragnaroth,
    Kyonin,
    Lambreth,
    LandsOfTheLinnormKings,
    Lastwall,
    Liberthane,
    Mendev,
    Mierani,
    Mivon,
    Molthune,
    MordantSpire,
    Narland,
    Narven,
    NewThassilon,
    Nidal,
    Nirmathas,
    Numeria,
    Oprak,
    Pitax,
    PrincipalitiesOfTaldaris,
    ProtectorateOfTheBlackMarquis,
    Qadira,
    Ravounel,
    Razmiran,
    RealmOfTheMammothLords,
    RiverKingdoms,
    Saggorak,
    Sarkoris,
    Sevenarches,
    Shalast,
    Taggoret,
    Taldor,
    TarKhadurrm,
    TarTaargadth,
    Thassilon,
    Touvette,
    Tymon,
    Ustalav,
    Varisia,
    Worldwound,

    // Garund
    Alijae,
    Alkenstar,
    Bloodcove,
    Chauxen,
    Dehrukani,
    Droon,
    Eihlona,
    Ekujae,
    Geb,
    GholGan,
    Holomog,
    Jalmeray,
    Kallijae,
    Katapesh,
    KazUlu,
    Kerdreg,
    Kibwe,
    Lirgen,
    MediogaltiIsland,
    Mualijae,
    Murraseth,
    MwangiExpanse,
    Mzali,
    Nantambu,
    Nex,
    NineWalls,
    Nurvatcha,
    Osirion,
    Rahadoum,
    Rastel,
    Sargava,
    Senghor,
    Shackles,
    ShoryEmpire,
    SoddenLands,
    StatueBuilder,
    TekritaninLeague,
    Thuvia,
    Tirakawhan,
    Usaro,
    Vidrian,
    Yamasa,

    // Arcadia
    Briarbough,
    Degasi,
    Halana,
    Innazpa,
    Nalmeras,
    PortValen,
    Razatlan,
    RazatlaniEmpire,
    Tazuni,
    Tomalan,
    Usclaeth,
    Xopatl,

    // Casmaron
    Ayyarad,
    Baghava,
    Danamsa,
    Dharget,
    Dhuraxilis,
    EmpireOfKelesh,
    Iobaria,
    Janvari,
    Japrini,
    Kaladay,
    Karazh,
    Kaskkari,
    Kaurata,
    Kelesh,
    Khoyadesh,
    Koloran,
    Mharana,
    Nayapul,
    NewIobaria,
    Ninshabur,
    Njalgard,
    Pandata,
    PolLiachora,
    PolPtirmeios,
    PolReanpharos,
    PolSylirica,
    Saman,
    Tanadesh,
    Vaktai,
    Vudra,
    Yenchabur,
    Zastel,
    Zelshabbar,

    // Crown of the World
    Almhult,
    Hasanaliat,
    KhorkiiClans,
    OsmanConfederation,
    UlaagorClans,
    Urjuk,
    Yumyzyl,
    ZavatenGura,

    // Tian Xia
    AgamazarRaj,
    Amanandar,
    Bachuan,
    ChuYe,
    DiguoDashu,
    Goka,
    Hongal,
    Hwanggot,
    JadeHegemonEmpire,
    Jinin,
    Kaoling,
    Kwanlai,
    Lingshen,
    Linvarre,
    LungWa,
    Minata,
    Minkai,
    Nagajor,
    Olehala,
    Ootoya,
    PoLi,
    Quain,
    RahkLo,
    Shenmen,
    Shu,
    Songbai,
    SuccessorStates,
    TangMai,
    Taumata,
    Teikoku,
    Tianjing,
    ValashRaj,
    ValashaiEmpire,
    Wanshou,
    XaHoi,
    Xidao,
    Yixing,
    ZiHa,
}

struct Nation {
    nation: Nations,
    nation_type: NationType,
    continent: Continents,
    major_area: Option<MajorAreas>,
    capital: Option<Settlements>,
    ruler: Option<Individuals>,
    government: Option<Government>,
    demonym: Option<Vec<&'static str>>,
    adjective: Option<Vec<&'static str>>,
    language: Option<Vec<Languages>>,
    religion: Option<Vec<Deities>>,
}

impl Nation {
    // Avistan
    fn absalom() -> Self {
        Self {
            nation: Nations::Absalom,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::Absalom),
            capital: Some(Settlements::Absalom),
            ruler: Some(Individuals::Gyr),
            government: Some(Government::GrandCouncil),
            demonym: Some(vec!["Absalomians"]),
            adjective: Some(vec!["Absalom"]),
            language: Some(Languages::all()),
            religion: Some(vec![
                Deities::Abadar,
                Deities::Aroden,
                Deities::Calistria,
                Deities::CaydenCailean,
                Deities::Iomedae,
                Deities::Irori,
                Deities::Nethys,
                Deities::Norgorber,
                Deities::Sarenrae,
                Deities::Shelyn,
            ]),
        }
    }
    fn alban() -> Self {
        Self {
            nation: Nations::Alijae,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: None,
            capital: None,
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn andoran() -> Self {
        Self {
            nation: Nations::Andoran,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: Some(Settlements::Almas),
            ruler: Some(Individuals::AndiraMarusek),
            government: Some(Government::ParliamentaryDemocracy),
            demonym: Some(vec!["Andorens"]),
            adjective: Some(vec!["Andoren"]),
            language: Some(vec![Languages::Common]),
            religion: Some(vec![
                Deities::Abadar,
                Deities::CaydenCailean,
                Deities::Erastil,
                Deities::Iomedae,
                Deities::Shelyn,
            ]),
        }
    }
    fn artume() -> Self {
        Self {
            nation: Nations::Artume,
            nation_type: NationType::Unlanded,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Artume),
            ruler: Some(Individuals::EdrydArtume),
            government: Some(Government::Monarchy),
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn bakrakhan() -> Self {
        Self {
            nation: Nations::Bakrakhan,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::SagaLands),
            capital: Some(Settlements::XinBakrakhan),
            ruler: Some(Individuals::Alaznist),
            government: None,
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn hold_of_belkzen() -> Self {
        Self {
            nation: Nations::HoldOfBelkzen,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::EyeOfDread),
            capital: Some(Settlements::Urgir),
            ruler: None,
            government: Some(Government::Tribal),
            demonym: None,
            adjective: Some(vec!["Belkzen"]),
            language: Some(vec![Languages::Orcish]),
            religion: Some(vec![
                Deities::Lamashtu,
                Deities::Rovagug,
                Deities::ZonKuthon,
            ]),
        }
    }
    fn brevoy() -> Self {
        Self {
            nation: Nations::Brevoy,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::NewStetven),
            ruler: Some(Individuals::NoleskiSurtova),
            government: Some(Government::HereditaryMonarchy),
            demonym: None,
            adjective: Some(vec!["Brevic"]),
            language: Some(vec![
                Languages::Common,
                Languages::Draconic,
                Languages::Hallit,
                Languages::Skald,
                Languages::Varisian,
            ]),
            religion: Some(vec![Deities::Abadar, Deities::Gorum, Deities::Pharasma]),
        }
    }
    fn cheliax() -> Self {
        Self {
            nation: Nations::Cheliax,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::OldCheliax),
            capital: Some(Settlements::Egorian),
            ruler: Some(Individuals::AbrogailThrune),
            government: Some(Government::Monarchy),
            demonym: Some(vec!["Chekaxians"]),
            adjective: Some(vec!["Chelaxian", "Chelish"]),
            language: Some(vec![Languages::Diabolic, Languages::Common]),
            religion: Some(vec![Deities::Erastil, Deities::Iomedae, Deities::ZonKuthon]),
        }
    }
    fn cordelon() -> Self {
        Self {
            nation: Nations::Cordelon,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Novoboro),
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn corvenn() -> Self {
        Self {
            nation: Nations::Corvenn,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: None,
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn cyrusian() -> Self {
        Self {
            nation: Nations::Cyrusian,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::SagaLands),
            capital: Some(Settlements::XinCyrusian),
            ruler: Some(Individuals::Xanderghul),
            government: None,
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn daggermark() -> Self {
        Self {
            nation: Nations::Daggermark,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Daggermark),
            ruler: Some(Individuals::MartroLivondar),
            government: Some(Government::Anarchy),
            demonym: None,
            adjective: None,
            language: Some(vec![
                Languages::Common,
                Languages::Hallit,
                Languages::Kelish,
            ]),
            religion: Some(vec![Deities::CaydenCailean, Deities::Norgorber]),
        }
    }
    fn doggadth() -> Self {
        Self {
            nation: Nations::Doggadth,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: None,
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: Some(vec![Languages::Dwarven]),
            religion: None,
        }
    }
    fn droskars_kingdom() -> Self {
        Self {
            nation: Nations::DroskarsKingdom,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: None,
            ruler: Some(Individuals::OrdrikTalhirk),
            government: Some(Government::Theocracy),
            demonym: None,
            adjective: None,
            language: Some(vec![Languages::Dwarven]),
            religion: Some(vec![Deities::Droskar]),
        }
    }
    fn druma() -> Self {
        Self {
            nation: Nations::Druma,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: Some(Settlements::Kerse),
            ruler: Some(Individuals::Kelldor),
            government: Some(Government::MercantileOligarchy),
            demonym: Some(vec!["Drumans"]),
            adjective: Some(vec!["Drumish", "Druman"]),
            language: Some(vec![Languages::Common, Languages::Dwarven]),
            religion: Some(vec![
                Deities::Abadar,
                Deities::Torag,
                Deities::PropheciesOfKalistrade,
            ]),
        }
    }
    fn echo_wood() -> Self {
        Self {
            nation: Nations::EchoWood,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Thornkeep),
            ruler: Some(Individuals::TervinBlackshield),
            government: Some(Government::Dictatorship),
            demonym: None,
            adjective: None,
            language: Some(vec![
                Languages::Common,
                Languages::Hallit,
                Languages::Varisian,
            ]),
            religion: Some(vec![
                Deities::Gorum,
                Deities::GreenFaith,
                Deities::Hanspur,
                Deities::Iomedae,
            ]),
        }
    }
    fn edasseril() -> Self {
        Self {
            nation: Nations::Edasseril,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::SagaLands),
            capital: Some(Settlements::XinEdasseril),
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: Some(vec![Languages::Thassilonian]),
            religion: None,
        }
    }
    fn eurythnia() -> Self {
        Self {
            nation: Nations::Eurythnia,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::SagaLands),
            capital: Some(Settlements::XinShalast),
            ruler: Some(Individuals::Sorshen),
            government: Some(Government::Monarchy),
            demonym: None,
            adjective: None,
            language: None,
            religion: Some(vec![Deities::Nocticula]),
        }
    }
    fn five_kings_mountains() -> Self {
        Self {
            nation: Nations::FiveKingsMountains,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: Some(Settlements::Highhelm),
            ruler: None,
            government: Some(Government::GatheringCouncil),
            demonym: None,
            adjective: Some(vec!["FiveKings"]),
            language: Some(vec![Languages::Dwarven]),
            religion: Some(vec![Deities::Torag]),
        }
    }
    fn galt() -> Self {
        Self {
            nation: Nations::Galt,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: Some(Settlements::Isarn),
            ruler: Some(Individuals::CamiliaDrannoch),
            government: Some(Government::Council),
            adjective: Some(vec!["Galtan"]),
            demonym: Some(vec!["Galtans"]),
            language: Some(vec![Languages::Common, Languages::Hallit]),
            religion: Some(vec![
                Deities::Calistria,
                Deities::CaydenCailean,
                Deities::Erastil,
                Deities::Iomedae,
                Deities::Norgorber,
                Deities::Shelyn,
            ]),
        }
    }
    fn gardadth() -> Self {
        Self {
            nation: Nations::Gardadth,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: None,
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn gastash() -> Self {
        Self {
            nation: Nations::Gastash,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::EyeOfDread),
            capital: Some(Settlements::XinGastash),
            ruler: Some(Individuals::Zutha),
            government: None,
            demonym: None,
            adjective: None,
            language: None,
            religion: Some(vec![Deities::PeacockSpirit]),
        }
    }
    fn grakodan() -> Self {
        Self {
            nation: Nations::Grakodan,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: None,
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn gralton() -> Self {
        Self {
            nation: Nations::Gralton,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Gralton),
            ruler: Some(Individuals::MarniusCherlorne),
            government: Some(Government::Oligarchy),
            demonym: Some(vec!["Graltoners"]),
            adjective: Some(vec!["Gralton"]),
            language: Some(vec![Languages::Common]),
            religion: Some(vec![
                Deities::Calistria,
                Deities::CaydenCailean,
                Deities::Erastil,
                Deities::GreenFaith,
                Deities::Norgorber,
            ]),
        }
    }
    fn gravelands() -> Self {
        Self {
            nation: Nations::Gravelands,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::EyeOfDread),
            capital: None,
            ruler: None,
            government: Some(Government::Anarchy),
            demonym: None,
            adjective: None,
            language: Some(vec![Languages::Varisian]),
            religion: Some(vec![Deities::Urgathoa]),
        }
    }
    fn haruka() -> Self {
        Self {
            nation: Nations::Haruka,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::SagaLands),
            capital: Some(Settlements::XinHaruka),
            ruler: Some(Individuals::Krune),
            government: None,
            demonym: None,
            adjective: None,
            language: None,
            religion: Some(vec![Deities::Lissala, Deities::PeacockSpirit]),
        }
    }
    fn hermea() -> Self {
        Self {
            nation: Nations::Hermea,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::HighSeas),
            capital: Some(Settlements::Promise),
            ruler: Some(Individuals::Mengkare),
            government: Some(Government::ContractualDictatorship),
            demonym: Some(vec!["Hermeans"]),
            adjective: Some(vec!["Hermean"]),
            language: Some(vec![Languages::Common, Languages::Draconic]),
            religion: Some(vec![Deities::Atheism]),
        }
    }
    fn hymbria() -> Self {
        Self {
            nation: Nations::Hymbria,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Mimere),
            ruler: Some(Individuals::FerielNellmyr),
            government: None,
            demonym: None,
            adjective: Some(vec!["Hymbrian"]),
            language: None,
            religion: Some(vec![
                Deities::Calistria,
                Deities::Desna,
                Deities::Gozreh,
                Deities::Nethys,
            ]),
        }
    }
    fn irrisen() -> Self {
        Self {
            nation: Nations::Irrisen,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::SagaLands),
            capital: Some(Settlements::Whitethrone),
            ruler: Some(Individuals::AnastasiaNikolaevna),
            government: Some(Government::Monarchy),
            demonym: Some(vec!["Irrisen", "Irrisens"]),
            adjective: Some(vec!["Irrisen", "Irriseni"]),
            language: Some(vec![Languages::Hallit, Languages::Skald]),
            religion: Some(vec![Deities::Lamashtu, Deities::ZonKuthon]),
        }
    }
    fn isger() -> Self {
        Self {
            nation: Nations::Isger,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::OldCheliax),
            capital: Some(Settlements::Elidir),
            ruler: Some(Individuals::Hedvend),
            government: Some(Government::Vassal(Nations::Cheliax)),
            demonym: Some(vec!["Isgeri"]),
            adjective: Some(vec!["Isgeri"]),
            language: Some(vec![Languages::Common]),
            religion: Some(vec![
                Deities::Asmodeus,
                Deities::Erastil,
                Deities::Diabolism,
            ]),
        }
    }
    fn issia() -> Self {
        Self {
            nation: Nations::Issia,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: None,
            ruler: None,
            government: None,
            demonym: Some(vec!["Issians"]),
            adjective: Some(vec!["Issian"]),
            language: None,
            religion: None,
        }
    }
    fn jelheg() -> Self {
        Self {
            nation: Nations::Jelheg,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: Some(Settlements::Rhuvasi),
            ruler: Some(Individuals::Jelheg),
            government: Some(Government::Meritocracy),
            demonym: Some(vec!["Jelhegi"]),
            adjective: Some(vec!["Jelhegi"]),
            language: Some(vec![Languages::Hallit]),
            religion: None,
        }
    }
    fn jistka_imperium() -> Self {
        Self {
            nation: Nations::JistkaImperium,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::GoldenRoad),
            capital: Some(Settlements::Mirnura),
            ruler: None,
            government: Some(Government::Monarchy),
            demonym: Some(vec!["Jistkans"]),
            adjective: Some(vec!["Jistkan"]),
            language: Some(vec![Languages::Jistka]),
            religion: None,
        }
    }
    fn kingdom_of_zog() -> Self {
        Self {
            nation: Nations::KingdomOfZog,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: None,
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn kragnaroth() -> Self {
        Self {
            nation: Nations::Kragnaroth,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::EyeOfDread),
            capital: Some(Settlements::Kragnaroth),
            ruler: Some(Individuals::OrynoxMarchelin),
            government: Some(Government::Monarchy),
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn kyonin() -> Self {
        Self {
            nation: Nations::Kyonin,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: Some(Settlements::Iadara),
            ruler: Some(Individuals::TelandiaEdasseril),
            government: Some(Government::Monarchy),
            demonym: Some(vec!["Kyoni"]),
            adjective: Some(vec!["Kyonin"]),
            language: Some(vec![Languages::Elven]),
            religion: Some(vec![Deities::Calistria, Deities::Desna, Deities::Nethys]),
            // todo elven pantheon
        }
    }
    fn lambreth() -> Self {
        Self {
            nation: Nations::Lambreth,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Maashinelle),
            ruler: Some(Individuals::KamdynArnefax),
            government: Some(Government::Dictatorship),
            demonym: None,
            adjective: None,
            language: Some(vec![
                Languages::Common,
                Languages::Hallit,
                Languages::Varisian,
            ]),
            religion: Some(vec![
                Deities::Calistria,
                Deities::Desna,
                Deities::Pharasma,
                Deities::Urgathoa,
            ]),
        }
    }
    fn lands_of_the_linnorm_kings() -> Self {
        Self {
            nation: Nations::LandsOfTheLinnormKings,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::SagaLands),
            capital: Some(Settlements::Kalsgard),
            ruler: None,
            government: Some(Government::Tribal),
            demonym: Some(vec!["Ulfen"]),
            adjective: None,
            language: Some(vec![Languages::Skald]),
            religion: Some(vec![
                Deities::Desna,
                Deities::Erastil,
                Deities::Gorum,
                Deities::Torag,
            ]),
        }
    }
    fn lastwall() -> Self {
        Self {
            nation: Nations::Lastwall,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::EyeOfDread),
            capital: Some(Settlements::Vigil),
            ruler: Some(Individuals::Ulthun),
            government: Some(Government::MilitaryDictatorship),
            demonym: Some(vec!["Lastmen", "Lastwomen", "Lastfolk"]),
            adjective: Some(vec!["Lastwall"]),
            language: Some(vec![Languages::Common, Languages::Varisian]),
            religion: Some(vec![Deities::Iomedae, Deities::Gorum]),
        }
    }
    fn liberthane() -> Self {
        Self {
            nation: Nations::Liberthane,
            nation_type: NationType::Unlanded,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::FortLiberthane),
            ruler: Some(Individuals::AchilleParsall),
            government: Some(Government::MilitaryProtectorate),
            demonym: None,
            adjective: Some(vec!["Liberthane"]),
            language: None,
            religion: Some(vec![Deities::Iomedae, Deities::Milani, Deities::Sarenrae]),
        }
    }
    fn mendev() -> Self {
        Self {
            nation: Nations::Mendev,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Nerosyan),
            ruler: Some(Individuals::Irahai),
            government: Some(Government::Monarchy),
            demonym: Some(vec!["Mendevians"]),
            adjective: Some(vec!["Mendevian"]),
            language: Some(vec![Languages::Common, Languages::Hallit]),
            religion: Some(vec![Deities::Iomedae]),
        }
    }
    fn mierani() -> Self {
        Self {
            nation: Nations::Mierani,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::SagaLands),
            capital: None,
            ruler: None,
            government: None,
            demonym: Some(vec!["Mierani"]),
            adjective: Some(vec!["Mierani"]),
            language: Some(vec![Languages::Elven]),
            religion: None,
        }
    }
    fn mivon() -> Self {
        Self {
            nation: Nations::Mivon,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Mivon),
            ruler: Some(Individuals::RastonSelline),
            government: Some(Government::Meritocracy),
            demonym: Some(vec!["Mivoni"]),
            adjective: Some(vec!["Mivoni"]),
            language: Some(vec![Languages::Common, Languages::Hallit]),
            religion: Some(vec![Deities::Abadar, Deities::Gorum, Deities::Pharasma]),
        }
    }
    fn molthune() -> Self {
        Self {
            nation: Nations::Molthune,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::EyeOfDread),
            capital: Some(Settlements::Canorate),
            ruler: Some(Individuals::MarkwinTeldas),
            government: Some(Government::MilitaryOligarchy),
            demonym: Some(vec!["Molthuni"]),
            adjective: Some(vec!["Molthuni"]),
            language: Some(vec![Languages::Common, Languages::Varisian]),
            religion: Some(vec![Deities::Abadar, Deities::Erastil, Deities::Iomedae]),
        }
    }
    fn mordant_spire() -> Self {
        Self {
            nation: Nations::MordantSpire,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::HighSeas),
            capital: Some(Settlements::MordantSpire),
            ruler: None,
            government: Some(Government::SecretCouncilOfElders),
            demonym: Some(vec!["Spiresworn"]),
            adjective: Some(vec!["Mordan Spire"]),
            language: Some(vec![Languages::AncientAzlanti]),
            religion: Some(vec![Deities::Gozreh]), // todo elven pantheon
        }
    }
    fn narven() -> Self {
        Self {
            nation: Nations::Narven,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: None,
            ruler: Some(Individuals::NarvenFeathereyes),
            government: Some(Government::Theocracy),
            demonym: None,
            adjective: None,
            language: None,
            religion: Some(vec![Deities::GreenFaith]),
        }
    }
    fn new_thassilon() -> Self {
        Self {
            nation: Nations::NewThassilon,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::SagaLands),
            capital: Some(Settlements::XinShalast),
            ruler: Some(Individuals::Sorshen),
            government: Some(Government::ArcaneMeritocracy),
            demonym: None,
            adjective: Some(vec!["New Thassilonian"]),
            language: Some(vec![Languages::Thassilonian, Languages::Common]),
            religion: Some(vec![Deities::Nocticula]),
        }
    }
    fn nidal() -> Self {
        Self {
            nation: Nations::Nidal,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::OldCheliax),
            capital: Some(Settlements::Pangolais),
            ruler: Some(Individuals::BlackTriune),
            government: Some(Government::MilitaristicTheocracy),
            demonym: Some(vec!["Nidalese"]),
            adjective: Some(vec!["Nidalese"]),
            language: Some(vec![
                Languages::Varisian,
                Languages::Common,
                Languages::Shadowtonuge,
            ]),
            religion: Some(vec![Deities::ZonKuthon, Deities::Asmodeus, Deities::Desna]),
        }
    }
    fn nirmathas() -> Self {
        Self {
            nation: Nations::Nirmathas,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::EyeOfDread),
            capital: Some(Settlements::Tamran),
            ruler: Some(Individuals::WeslenGavirk),
            government: Some(Government::Meritocracy),
            demonym: Some(vec!["Nirmathi"]),
            adjective: Some(vec!["Nirmathi"]),
            language: Some(vec![Languages::Common, Languages::Varisian]),
            religion: Some(vec![Deities::Erastil, Deities::Gorum, Deities::Iomedae]),
        }
    }
    fn numeria() -> Self {
        Self {
            nation: Nations::Numeria,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Starfall),
            ruler: Some(Individuals::KevothKul),
            government: Some(Government::BarbaricMonarchy),
            demonym: Some(vec!["Numerians"]),
            adjective: Some(vec!["Numerian"]),
            language: Some(vec![Languages::Hallit]),
            religion: Some(vec![
                Deities::Abadar,
                Deities::Brigh,
                Deities::Desna,
                Deities::Erastil,
                Deities::Gorum,
                Deities::Iomedae,
                Deities::Baphomet,
                Deities::Deskari,
                Deities::IronGods,
            ]),
        }
    }
    fn oprak() -> Self {
        Self {
            nation: Nations::Oprak,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Hunthul),
            ruler: Some(Individuals::Azaersi),
            government: Some(Government::MilitaryDictatorship),
            demonym: None,
            adjective: Some(vec!["Oprakan"]),
            language: Some(vec![Languages::Goblin]),
            religion: Some(vec![Deities::Hadregash]),
        }
    }
    fn pitax() -> Self {
        Self {
            nation: Nations::Pitax,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Pitax),
            ruler: Some(Individuals::CastruccioIrovetti),
            government: Some(Government::Autocracy),
            demonym: Some(vec!["Pitaxians"]),
            adjective: Some(vec!["Pitaxian"]),
            language: Some(vec![Languages::Common]),
            religion: Some(vec![Deities::Calistria, Deities::Desna]),
        }
    }

    fn protectorate_of_the_black_marquis() -> Self {
        Self {
            nation: Nations::ProtectorateOfTheBlackMarquis,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Deadbridge),
            ruler: Some(Individuals::UrdulBazzak),
            government: Some(Government::PirateDespotAndCouncil),
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn qadira() -> Self {
        Self {
            nation: Nations::Qadira,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::GoldenRoad),
            capital: Some(Settlements::Katheer),
            ruler: Some(Individuals::Xerbystes),
            government: Some(Government::Satrapy(Nations::Kelesh)),
            demonym: Some(vec!["Qadirans"]),
            adjective: Some(vec!["Qadiran", "Qadiri"]),
            language: Some(vec![Languages::Kelish]),
            religion: Some(vec![
                Deities::Irori,
                Deities::Nethys,
                Deities::Pharasma,
                Deities::Rovagug,
                Deities::Sarenrae,
                // todo elemental lords
            ]),
        }
    }
    fn ravounel() -> Self {
        Self {
            nation: Nations::Ravounel,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::OldCheliax),
            capital: Some(Settlements::Kintargo),
            ruler: Some(Individuals::JiliaBainilus),
            government: Some(Government::Confederation),
            demonym: None,
            adjective: None,
            language: Some(vec![Languages::Common]),
            religion: Some(vec![Deities::Calistria, Deities::Shelyn]),
        }
    }
    fn razmiran() -> Self {
        Self {
            nation: Nations::Razmiran,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Thronestep),
            ruler: Some(Individuals::Razmir),
            government: Some(Government::TheocraticDictatorship),
            demonym: Some(vec!["Razmiri"]),
            adjective: Some(vec!["Razmiri"]),
            language: None,
            religion: Some(vec![Deities::Razmir]),
        }
    }
    fn realm_of_the_mammoth_lords() -> Self {
        Self {
            nation: Nations::RealmOfTheMammothLords,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::SagaLands),
            capital: None,
            ruler: None,
            government: Some(Government::MammothLordsTribal),
            demonym: Some(vec!["MammothLords"]),
            adjective: None,
            language: Some(vec![Languages::Hallit, Languages::Jotun]),
            religion: Some(vec![Deities::Gorum, Deities::Desna, Deities::Rovagug]),
        }
    }
    fn river_kingdoms() -> Self {
        Self {
            nation: Nations::RiverKingdoms,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: None,
            ruler: None,
            government: Some(Government::SquabblingMonarchies),
            demonym: Some(vec!["Riverfolk"]),
            adjective: Some(vec!["Riverfolk", "River Kingdoms"]),
            language: Some(vec![Languages::Common, Languages::Hallit]),
            religion: Some(vec![
                Deities::Calistria,
                Deities::CaydenCailean,
                Deities::Desna,
                Deities::Erastil,
                Deities::Gorum,
                Deities::Gyronna,
                Deities::Hanspur,
                Deities::Iomedae,
                Deities::Lamashtu,
                Deities::Norgorber,
            ]),
        }
    }
    fn saggorak() -> Self {
        Self {
            nation: Nations::Saggorak,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: None,
            ruler: Some(Individuals::Harral),
            government: Some(Government::Monarchy),
            demonym: None,
            adjective: None,
            language: Some(vec![Languages::Dwarven]),
            religion: None,
        }
    }
    fn sarkoris() -> Self {
        Self {
            nation: Nations::Sarkoris,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Gundrun),
            ruler: None,
            government: None,
            demonym: None,
            adjective: Some(vec!["Sarkorian"]),
            language: Some(vec![Languages::Hallit]),
            religion: Some(vec![Deities::GreenFaith, Deities::Pulura]), // todo druidism
        }
    }
    fn sevenarches() -> Self {
        Self {
            nation: Nations::Sevenarches,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Sevenarches),
            ruler: Some(Individuals::Oakstewards),
            government: Some(Government::Oligarchy),
            demonym: Some(vec!["Sevenarchers"]),
            adjective: Some(vec!["Sevenarches"]),
            language: Some(vec![
                Languages::Fey,
                Languages::Gnomish,
                Languages::Common,
                Languages::Hallit,
                Languages::Wildsong,
            ]),
            religion: Some(vec![
                Deities::eldest(),
                Deities::Erastil,
                Deities::GreenFaith,
                Deities::Hanspur,
            ]),
        }
    }
    fn shalast() -> Self {
        Self {
            nation: Nations::Shalast,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::SagaLands),
            capital: Some(Settlements::XinShalast),
            ruler: Some(Individuals::Karzoug),
            government: Some(Government::Monarchy),
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn taggoret() -> Self {
        Self {
            nation: Nations::Taggoret,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: None,
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn taldor() -> Self {
        Self {
            nation: Nations::Taldor,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: Some(Settlements::Oppara),
            ruler: Some(Individuals::EutropiaStavian),
            government: Some(Government::BureaucraticMonarchy),
            demonym: Some(vec!["Taldans"]),
            adjective: Some(vec!["Taldan"]),
            language: Some(vec![Languages::Common, Languages::Kelish]),
            religion: Some(vec![
                Deities::Abadar,
                Deities::Aroden,
                Deities::Calistria,
                Deities::CaydenCailean,
                Deities::Norgorber,
                Deities::Sarenrae,
                Deities::Shelyn,
            ]),
        }
    }
    fn tar_khadurrm() -> Self {
        Self {
            nation: Nations::TarKhadurrm,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: None,
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: Some(vec![Languages::Dwarven]),
            religion: None,
        }
    }
    fn tar_taargadth() -> Self {
        Self {
            nation: Nations::TarTaargadth,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: None,
            ruler: None,
            government: None,
            demonym: Some(vec!["Gadths"]),
            adjective: Some(vec!["Gadth"]),
            language: Some(vec![Languages::Dwarven]),
            religion: None,
        }
    }
    fn thassilon() -> Self {
        Self {
            nation: Nations::Thassilon,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::SagaLands),
            capital: None,
            ruler: Some(Individuals::Xin),
            government: Some(Government::Imperial),
            demonym: Some(vec!["Thassilonians"]),
            adjective: Some(vec!["Thassilonian"]),
            language: None,
            religion: Some(vec![Deities::Lissala]),
        }
    }
    fn touvette() -> Self {
        Self {
            nation: Nations::Touvette,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Avendale),
            ruler: Some(Individuals::CabolVoran),
            government: Some(Government::MilitaryDictatorship),
            demonym: None,
            adjective: Some(vec!["Touvette"]),
            language: None,
            religion: Some(vec![Deities::Atheism]),
        }
    }
    fn tymon() -> Self {
        Self {
            nation: Nations::Tymon,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Settlements::Tymon),
            ruler: Some(Individuals::UllorthUngin),
            government: Some(Government::Dictatorship),
            demonym: Some(vec!["Tymons"]),
            adjective: Some(vec!["Tymon"]),
            language: Some(vec![Languages::Common]),
            religion: Some(vec![Deities::Gorum]),
        }
    }
    fn ustalav() -> Self {
        Self {
            nation: Nations::Ustalav,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::EyeOfDread),
            capital: Some(Settlements::Caliphas),
            ruler: Some(Individuals::AduardOrdranti),
            government: Some(Government::LooseConfederacy),
            demonym: Some(vec!["Ustalavs"]),
            adjective: Some(vec!["Ustalavic"]),
            language: Some(vec![Languages::Varisian, Languages::Common, Languages::Skald]),
            religion: Some(vec![Deities::Pharasma, Deities::Desna, Deities::Urgathoa]),
        }
    }
    fn varisia() -> Self {
        Self {
            nation: Nations::Varisia,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::SagaLands),
            capital: None,
            ruler: None,
            government: Some(Government::UnaffiliatedCityStates),
            demonym: Some(vec!["Varisians"]),
            adjective: Some(vec!["Varisian"]),
            language: Some(vec![Languages::Varisian, Languages::Common, Languages::Shoanti]),
            religion: Some(vec![
                Deities::Abadar,
                Deities::Calistria,
                Deities::CaydenCailean,
                Deities::Desna,
                Deities::Erastil,
                Deities::Gozreh,
                Deities::Lamashtu,
                Deities::Norgorber,
                Deities::Pharasma,
                Deities::Urgathoa,
                Deities::ZonKuthon,
            ]),
        }
    }

    // Avistan Non-Existent
    fn worldwound() -> Self {
        Self {
            nation: Nations::Worldwound,
            nation_type: NationType::NonExistent,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: None,
            ruler: None,
            government: Some(Government::LooseCollectionOfDemonicWarlords),
            demonym: None,
            adjective: None,
            language : Some(vec![Languages::Chthonian, Languages::Hallit]),
            religion: Some(vec![Deities::Deskari, Deities::Lamashtu]), // todo devil worship

    // Garund
    fn alijae() -> Self {
        // todo review technically this is an organization
        Self {
            nation: Nations::Alijae,
            nation_type: NationType::Unlanded,
            continent: Continents::Garund,
            major_area: None,
            capital: Some(Settlements::Nagisa),
            ruler: None,
            government: Some(Government::Tribal),
            demonym: None,
            adjective: None,
            language: None,
            religion: Some(vec![
                Deities::Nocticula,
                Deities::Desna,
                Deities::Findeladlara,
                Deities::Irori,
                Deities::Nethys,
            ]),
        }
    }
    fn alkenstar() -> Self {
        Self {
            nation: Nations::Alkenstar,
            nation_type: NationType::Normal,
            continent: Continents::Garund,
            major_area: Some(MajorAreas::ImpossibleLands),
            capital: Some(Settlements::AlkenstarCity),
            ruler: Some(Individuals::TerittaRicia),
            government: Some(Government::ConstitutionalMonarchy),
            demonym: Some(vec!["Alkenstarians"]),
            adjective: Some(vec!["Alkenstar", "Alkenstari"]),
            language: Some(vec![
                Languages::Common,
                Languages::Dwarven,
                Languages::Kelish,
                Languages::Osiriani,
            ]),
            religion: Some(vec![
                Deities::Abadar,
                Deities::Erastil,
                Deities::Irori,
                Deities::Torag,
            ]),
        }
    }
    fn bloodcove() -> Self {
        Self {
            nation: Nations::Bloodcove,
            nation_type: NationType::Normal,
            continent: Continents::Garund,
            major_area: None,
            capital: Some(Settlements::Bloodcove),
            ruler: Some(Individuals::HarthwikBarzoni),
            government: Some(Government::SecretSyndicate),
            demonym: None,
            adjective: None,
            language: Some(vec![
                Languages::Common,
                Languages::Elven,
                Languages::Dwarven,
                Languages::Gnomish,
                Languages::Goblin,
                Languages::Mwangi,
            ]),
            religion: Some(vec![
                Deities::Abadar,
                Deities::Besmara,
                Deities::Erastil,
                Deities::Gozreh,
                Deities::HeiFeng,
                Deities::Norgorber,
            ]),
        }
    }
    fn chauxen() -> Self {
        Self {
            nation: Nations::Chauxen,
            nation_type: NationType::Unlanded,
            continent: Continents::Garund,
            major_area: None,
            capital: None,
            ruler: None,
            government: Some(Government::Colony(Nations::Vudra)),
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn dehrukani() -> Self {
        Self {
            nation: Nations::Dehrukani,
            nation_type: NationType::Normal,
            continent: Continents::Garund,
            major_area: None,
            capital: None,
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: None,
            religion: Some(vec![Deities::Ashava, Deities::Cernunnos, Deities::Lalaci]),
        }
    }
    fn droon() -> Self {
        Self {
            nation: Nations::Droon,
            nation_type: NationType::Unlanded,
            continent: Continents::Garund,
            major_area: None,
            capital: None,
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: None,
            religion: Some(vec![Deities::GrandmotherSpider, Deities::Lixiriltha]),
        }
    }
    fn eihlona() -> Self {
        Self {
            nation: Nations::Eihlona,
            nation_type: NationType::Normal,
            continent: Continents::Garund,
            major_area: None,
            capital: None,
            ruler: None,
            government: None,
            demonym: Some(vec!["Eihlonan"]),
            adjective: Some(vec!["Eihlonan"]),
            language: None,
            religion: None,
        }
    }
    fn ekujae() -> Self {
        Self {
            nation: Nations::Ekujae,
            nation_type: NationType::Unlanded,
            continent: Continents::Garund,
            major_area: None,
            capital: None,
            ruler: None,
            government: Some(Government::Democracy),
            demonym: Some(vec!["Ekujae"]),
            adjective: Some(vec!["Ekujae"]),
            language: None,
            religion: Some(vec![Deities::Yuelral]),
        }
    }
    fn geb() -> Self {
        Self {
            nation: Nations::Geb,
            nation_type: NationType::Normal,
            continent: Continents::Garund,
            major_area: Some(MajorAreas::ImpossibleLands),
            capital: Some(Settlements::Mechitar),
            ruler: Some(Individuals::Geb),
            government: Some(Government::UndeadDictatorship),
            demonym: Some(vec!["Gebbites"]),
            adjective: Some(vec!["Gebbite"]),
            language: Some(vec![
                Languages::Kelish,
                Languages::Osiriani,
                Languages::Necril,
            ]),
            religion: Some(vec![
                Deities::Arazni,
                Deities::Mahathallah,
                Deities::Nethys,
                Deities::Urgathoa,
                Deities::ZonKuthon,
            ]),
        }
    }
    fn ghol_gan() -> Self {
        Self {
            nation: Nations::GholGan,
            nation_type: NationType::NonExistent,
            continent: Continents::Garund,
            major_area: Some(MajorAreas::HighSeas),
            capital: Some(Settlements::Tzaarban),
            ruler: None,
            government: None,
            demonym: Some(vec!["Ghol-Gani"]),
            adjective: Some(vec!["Ghol-Gan", "Ghol-Gani"]),
            language: None,
            religion: None,
        }
    }
    fn holomog() -> Self {
        Self {
            nation: Nations::Holomog,
            nation_type: NationType::Normal,
            continent: Continents::Garund,
            major_area: None,
            capital: None,
            ruler: Some(Individuals::HoloEnyana),
            government: Some(Government::Matriarchy),
            demonym: None,
            adjective: Some(vec!["Holoma"]),
            language: Some(vec![Languages::Drooni, Languages::Empyrean]),
            religion: Some(vec![Deities::Mazludeh]),
        }
    }

    // Arcadia
    fn briarbough() -> Self {
        Self {
            nation: Nations::Briarbough,
            nation_type: NationType::Normal,
            continent: Continents::Arcadia,
            major_area: None,
            capital: None,
            ruler: None,
            government: Some(Government::RowenRifle),
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn degasi() -> Self {
        Self {
            nation: Nations::Degasi,
            nation_type: NationType::Normal,
            continent: Continents::Arcadia,
            major_area: None,
            capital: Some(Settlements::Degasi),
            ruler: None,
            government: None,
            demonym: None,
            adjective: Some(vec!["Degasi"]),
            language: None,
            religion: None,
        }
    }
    fn halana() -> Self {
        Self {
            nation: Nations::Halana,
            nation_type: NationType::Normal,
            continent: Continents::Arcadia,
            major_area: None,
            capital: Some(Settlements::Rookery),
            ruler: None,
            government: Some(Government::Theocracy),
            demonym: None,
            adjective: Some(vec!["Halana"]),
            language: None,
            religion: Some(vec![Deities::Pazuzu]),
        }
    }

    // Crown of the World
    fn almhult() -> Self {
        Self {
            nation: Nations::Almhult,
            nation_type: NationType::Unlanded,
            continent: Continents::CrownOfTheWorld,
            major_area: None,
            capital: None,
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: Some(vec![Languages::Skald]),
            religion: None,
        }
    }
    fn hasanaliat() -> Self {
        Self {
            nation: Nations::Hasanaliat,
            nation_type: NationType::Normal,
            continent: Continents::CrownOfTheWorld,
            major_area: None,
            capital: None,
            ruler: None,
            government: Some(Government::Mahajanapada),
            demonym: None,
            adjective: None,
            language: Some(vec![Languages::Erutaki]),
            religion: None,
        }
    }

    // Casmaron
    fn ayyarad() -> Self {
        Self {
            nation: Nations::Ayyarad,
            nation_type: NationType::Vassal,
            continent: Continents::Casmaron,
            major_area: None,
            capital: None,
            ruler: None,
            government: Some(Government::Vassal(Nations::EmpireOfKelesh)),
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn baghava() -> Self {
        Self {
            nation: Nations::Baghava,
            nation_type: NationType::Unlanded,
            continent: Continents::Casmaron,
            major_area: None,
            capital: None,
            ruler: None,
            government: Some(Government::Mahajanapada),
            demonym: None,
            adjective: None,
            language: None,
            religion: Some(vec![Deities::Gaelata]),
        }
    }
    fn danamsa() -> Self {
        Self {
            nation: Nations::Danamsa,
            nation_type: NationType::Unlanded,
            continent: Continents::Casmaron,
            major_area: None,
            capital: Some(Settlements::Radripal),
            ruler: None,
            government: Some(Government::Mahajanapada),
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn dharget() -> Self {
        Self {
            nation: Nations::Dharget,
            nation_type: NationType::Unlanded,
            continent: Continents::Casmaron,
            major_area: None,
            capital: None,
            ruler: None,
            government: Some(Government::Mahajanapada),
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn dhuraxilis() -> Self {
        Self {
            nation: Nations::Dhuraxilis,
            nation_type: NationType::Normal,
            continent: Continents::Casmaron,
            major_area: None,
            capital: Some(Settlements::Dhuraxilis),
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn empire_of_kelesh() -> Self {
        Self {
            nation: Nations::EmpireOfKelesh,
            nation_type: NationType::Normal,
            continent: Continents::Casmaron,
            major_area: None,
            capital: Some(Settlements::Isfahel),
            ruler: Some(Individuals::Kalish),
            government: Some(Government::ImperialMonarchy),
            demonym: Some(vec!["Keleshites"]),
            adjective: Some(vec!["Keleshite", "Kelesh"]),
            language: Some(vec![Languages::Kelish]),
            religion: Some(vec![
                Deities::Abadar,
                Deities::Sarenrae,
                Deities::Irori,
                Deities::Rovagug,
            ]),
        }
    }

    // Tian Xia
    fn agamazar_raj() -> Self {
        Self {
            nation: Nations::AgamazarRaj,
            nation_type: NationType::Vassal,
            continent: Continents::TianXia,
            major_area: None,
            capital: None,
            ruler: None,
            government: Some(Government::Vassal(Nations::ValashRaj)),
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
    fn bachuan() -> Self {
        Self {
            nation: Nations::Bachuan,
            nation_type: NationType::Normal,
            continent: Continents::TianXia,
            major_area: None,
            capital: Some(Settlements::Peijita),
            ruler: Some(Individuals::PeiYaeMen),
            government: Some(Government::AutocraticCouncil),
            demonym: Some(vec!["Bachuan"]),
            adjective: Some(vec!["Bachuan"]),
            language: Some(vec![Languages::Tien]),
            religion: Some(vec![
                Deities::Atheism,
                Deities::Kofusachi,
                Deities::HeiFeng,
                Deities::QiZhong,
            ]),
        }
    }
    fn chu_ye() -> Self {
        Self {
            nation: Nations::ChuYe,
            nation_type: NationType::Normal,
            continent: Continents::TianXia,
            major_area: None,
            capital: Some(Settlements::Jyito),
            ruler: Some(Individuals::Tsuneni),
            government: Some(Government::OniShogunate),
            demonym: None,
            adjective: None,
            language: Some(vec![
                Languages::Jotun,
                Languages::Minkaian,
                Languages::Samsaran,
                Languages::Senzar,
                Languages::Tien,
            ]),
            religion: Some(vec![Deities::Fumeiyoshi, Deities::GeneralSusumu]),
        }
    }
    fn hwanggot() -> Self {
        Self {
            nation: Nations::Hwanggot,
            nation_type: NationType::Normal,
            continent: Continents::TianXia,
            major_area: None,
            capital: Some(Settlements::Haseong),
            ruler: Some(Individuals::HyeonGeonJi),
            government: Some(Government::HereditaryMonarchy),
            demonym: Some(vec!["Hwans"]),
            adjective: None,
            language: Some(vec![Languages::Hwan, Languages::Tien]),
            religion: Some(vec![
                Deities::Kofusachi,
                Deities::Shelyn,
                Deities::SunWukong,
            ]),
        }
    }
}
