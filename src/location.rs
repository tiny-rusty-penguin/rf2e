use crate::deities::Deities;
use crate::language::Languages;
use crate::{Government, Individuals};

enum Continents {
    Avistan,
    Garund,
    TianXia,
    Arcadia,
    CrownOfTheWorld,
    Azlant,
    Casmaron,
    Darklands,
    Sarusan,
}

enum MajorAreas {
    // todo review
    SagaLands,
    EyeOfDread,
    ShiningKingdoms,
    OldCheliax,
    Absalom,
    GoldenRoad,
    HighSeas,
    ImpossibleLands,
    BrokenLands,
}

pub enum PlanarNations {
    Aralidae,
    Aylok,
    AzureEmpire,
    DiguoDashu,
    Egypt,
    Felgunn,
    HangingMarches,
    Hautansia,
}

pub enum Nations {
    Absalom,
    AgamazarRaj,
    Alban,
    Alijae,
    Alkenstar,
    Almhult,
    Andoran,
    Artume,
    Ayyarad,
    Bachuan,
    Baghava,
    HoldOfBelkzen,
    Bloodcove,
    Brevoy,
    Briarbough,
    Chauxen,
    Cheliax,
    ChuYe,
    Cordelon,
    Daggermark,
    Danamsa,
    Degasi,
    Dehrukani,
    Dharget,
    Dhuraxilis,
    Droon,
    DroskarsKingdom,
    Druma,
    EchoWood,
    Edasseril,
    Eihlona,
    Ekujae,
    Eurythnia,
    EmpireOfKelesh,
    FiveKingsMountains,
    Galt,
    Geb,
    GholGan,
    Gralton,
    Gravelands,
    Halana,
    Hasanaliat,
    Hermea,
    Holomog,
    Hwanggot,
    Hymbria,
    Innazpa,
    Iobaria,
    Irrisen,
    Isger,
    JaathoomEmpire,
    JadeHegemonEmpire,
    Jalmeray,
    Janvari,
    Japrini,
    Jelheg,
    Jinin,
    JistkaImperium,
    Jotungard,
    Kaladay,
    Kallijae,
    Kamora,
    Kaoling,
    Karazh,
    Kashak,
    Kaskkari,
    Katapesh,
    Kaurata,
    KazUlu,
    Kelizandrika,
    Kerdreg,
    KhorkiiClans,
    Khoyadesh,
    KingdomOfZog,
    Kragnaroth,
    Kyonin,
    Lambreth,
    LandsOfTheLinnormKings,
    Lastwall,
    Liberthane,
    Lingshen,
    Linvarre,
    MediogaltiIsland,
    Mendev,
    Mharana,
    Mierani,
    Minata,
    Minkai,
    Mivon,
    Molthune,
    MordantSpire,
    Mualijae,
    Murraseth,
    MwangiExpanse,
    Mzali,
    Nagajor,
    Nalmeras,
    Nantambu,
    Narland,
    Nayapul,
    NewIobaria,
    NewThassilon,
    Nex,
    Nidal,
    NineWalls,
    Ning,
    Nirmathas,
    Njalgard,
    Numeria,
    Nurvatcha,
    Olehala,
    Oprak,
    Osirion,
    OsmanConfederation,
    Pandata,
    PeerlessEmpire,
    Pitax,
    PoLi,
    PolLiachora,
    PolPtirmeios,
    PolReanpharos,
    PolSylirica,
    PortValen,
    Preita,
    ProtectorateOfTheBlackMarquis,
    Qadira,
    Quain,
    Rahadoum,
    RahkLo,
    Rastel,
    Ravounel,
    Razatlan,
    Razmiran,
    RealmOfTheMammothLords,
    RiverKingdoms,
    Russia,
    Saman,
    Sargava,
    Sarkoris,
    Senghor,
    Sevenarches,
    Shackles,
    Shenmen,
    SoddenLands,
    Songbai,
    Sovyrian,
    StatueBuilder,
    Stonetide,
    Taldor,
    Tanadesh,
    TangMai,
    TarTaargadth,
    Tazuni,
    TekritaninLeague,
    ThassilonHiuNuo,
    Thuvia,
    Tianjing,
    Tirakawhan,
    Tomalan,
    Touvette,
    Tymon,
    UlaagorClans,
    Urjuk,
    Usaro,
    Usclaeth,
    Ustalav,
    Vaktai,
    ValashRaj,
    ValashaiEmpire,
    Varisia,
    Vidrian,
    Voiporl,
    Vudra,
    Wanshou,
    XaHoi,
    Xibalba,
    Xidao,
    Xopatl,
    Yumyzyl,
    Zastel,
    ZavatenGura,
    Zelshabbar,
    ZiHa,
    Zo,
}

enum Cities {
    Absalom,
    Nagisa,
    AlkenstarCity,
    Almas,
    Artume,
    Peijita,
    Urgir,
    Bloodcove,
    NewSteven,
    Egorian,
    Jyito,
    Novoboro,
    Daggermark,
    Radripal,
    Degasi,
    Dhuraxilis,
    HiuNuo,
    Thornkeep,
    XinEdasseril,
    XinEurythnia,
    XinShalast,
    Isfahel,
    Highhelm,
    Isarn,
    Mechitar,
    Tzaarban,
    Gralton,
    Rookery,
    Promise,
    Haseong,
    Mimere,
}

enum NationType {
    Normal,
    NonExistent,
    Vassal,
    Unlanded,
}

struct Nation {
    nation: Nations,
    nation_type: NationType,
    continent: Continents,
    major_area: Option<MajorAreas>,
    capital: Option<Cities>,
    ruler: Option<Individuals>,
    government: Option<Government>,
    demonym: Option<&'static str>,
    adjective: Option<Vec<&'static str>>,
    language: Option<Vec<Languages>>,
    religion: Option<Vec<Deities>>,
}

impl Nation {
    pub fn absalom() -> Self {
        Self {
            nation: Nations::Absalom,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::Absalom),
            capital: Some(Cities::Absalom),
            ruler: Some(Individuals::Gyr),
            government: Some(Government::GrandCouncil),
            demonym: Some("Absalomians"),
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
    pub fn agamazar_raj() -> Self {
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
    pub fn alban() -> Self {
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

    fn alijae() -> Self {
        // todo review technically this is an organization
        Self {
            nation: Nations::Alijae,
            nation_type: NationType::Unlanded,
            continent: Continents::Garund,
            major_area: None,
            capital: Some(Cities::Nagisa),
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
            capital: Some(Cities::AlkenstarCity),
            ruler: Some(Individuals::TerittaRicia),
            government: Some(Government::ConstitutionalMonarchy),
            demonym: Some("Alkenstarians"),
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
    fn andoran() -> Self {
        Self {
            nation: Nations::Andoran,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: Some(Cities::Almas),
            ruler: Some(Individuals::AndiraMarusek),
            government: Some(Government::ParliamentaryDemocracy),
            demonym: Some("Andorens"),
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
            capital: Some(Cities::Artume),
            ruler: Some(Individuals::EdrydArtume),
            government: Some(Government::Monarchy),
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
        }
    }
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
    fn bachuan() -> Self {
        Self {
            nation: Nations::Bachuan,
            nation_type: NationType::Normal,
            continent: Continents::TianXia,
            major_area: None,
            capital: Some(Cities::Peijita),
            ruler: Some(Individuals::PeiYaeMen),
            government: Some(Government::AutocraticCouncil),
            demonym: Some("Bachuan"),
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
    fn hold_of_belkzen() -> Self {
        Self {
            nation: Nations::HoldOfBelkzen,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::EyeOfDread),
            capital: Some(Cities::Urgir),
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
    fn bloodcove() -> Self {
        Self {
            nation: Nations::Bloodcove,
            nation_type: NationType::Normal,
            continent: Continents::Garund,
            major_area: None,
            capital: Some(Cities::Bloodcove),
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
    fn brevoy() -> Self {
        Self {
            nation: Nations::Brevoy,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Cities::NewSteven),
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
    fn cheliax() -> Self {
        Self {
            nation: Nations::Cheliax,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::OldCheliax),
            capital: Some(Cities::Egorian),
            ruler: Some(Individuals::AbrogailThrune),
            government: Some(Government::Monarchy),
            demonym: Some("Chekaxians"),
            adjective: Some(vec!["Chelaxian", "Chelish"]),
            language: Some(vec![Languages::Diabolic, Languages::Common]),
            religion: Some(vec![Deities::Erastil, Deities::Iomedae, Deities::ZonKuthon]),
        }
    }
    fn chu_ye() -> Self {
        Self {
            nation: Nations::ChuYe,
            nation_type: NationType::Normal,
            continent: Continents::TianXia,
            major_area: None,
            capital: Some(Cities::Jyito),
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
    fn cordelon() -> Self {
        Self {
            nation: Nations::Cordelon,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Cities::Novoboro),
            ruler: None,
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
            capital: Some(Cities::Daggermark),
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
    fn danamsa() -> Self {
        Self {
            nation: Nations::Danamsa,
            nation_type: NationType::Unlanded,
            continent: Continents::Casmaron,
            major_area: None,
            capital: Some(Cities::Radripal),
            ruler: None,
            government: Some(Government::Mahajanapada),
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
            capital: Some(Cities::Degasi),
            ruler: None,
            government: None,
            demonym: None,
            adjective: Some(vec!["Degasi"]),
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
            capital: Some(Cities::Dhuraxilis),
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: None,
            religion: None,
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
    fn droskars_kingdom() -> Self {
        Self {
            nation: Nations::DroskarsKingdom,
            nation_type: NationType::Normal,
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
            capital: None,
            ruler: Some(Individuals::Kelldor),
            government: Some(Government::MercantileOligarchy),
            demonym: Some("Drumans"),
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
            capital: Some(Cities::Thornkeep),
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
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::SagaLands),
            capital: Some(Cities::XinEdasseril),
            ruler: None,
            government: None,
            demonym: None,
            adjective: None,
            language: Some(vec![Languages::Thassilonian]),
            religion: None,
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
            demonym: Some("Eihlonan"),
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
            demonym: Some("Ekujae"),
            adjective: Some(vec!["Ekujae"]),
            language: None,
            religion: Some(vec![Deities::Yuelral]),
        }
    }
    fn eurythnia() -> Self {
        Self {
            nation: Nations::Eurythnia,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::SagaLands),
            capital: Some(Cities::XinShalast),
            ruler: Some(Individuals::Sorshen),
            government: Some(Government::Monarchy),
            demonym: None,
            adjective: None,
            language: None,
            religion: Some(vec![Deities::Nocticula]),
        }
    }
    fn empire_of_kelesh() -> Self {
        Self {
            nation: Nations::EmpireOfKelesh,
            nation_type: NationType::Normal,
            continent: Continents::Casmaron,
            major_area: None,
            capital: Some(Cities::Isfahel),
            ruler: Some(Individuals::Kalish),
            government: Some(Government::ImperialMonarchy),
            demonym: Some("Keleshites"),
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
    fn five_kings_mountains() -> Self {
        Self {
            nation: Nations::FiveKingsMountains,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::ShiningKingdoms),
            capital: Some(Cities::Highhelm),
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
            capital: Some(Cities::Isarn),
            ruler: Some(Individuals::CamiliaDrannoch),
            government: Some(Government::Council),
            adjective: Some(vec!["Galtan"]),
            demonym: Some("Galtans"),
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
    fn geb() -> Self {
        Self {
            nation: Nations::Geb,
            nation_type: NationType::Normal,
            continent: Continents::Garund,
            major_area: Some(MajorAreas::ImpossibleLands),
            capital: Some(Cities::Mechitar),
            ruler: Some(Individuals::Geb),
            government: Some(Government::UndeadDictatorship),
            demonym: Some("Gebbites"),
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
            capital: Some(Cities::Tzaarban),
            ruler: None,
            government: None,
            demonym: Some("Ghol-Gani"),
            adjective: Some(vec!["Ghol-Gan", "Ghol-Gani"]),
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
            capital: Some(Cities::Gralton),
            ruler: Some(Individuals::MarniusCherlorne),
            government: Some(Government::Oligarchy),
            demonym: Some("Graltoners"),
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
    fn halana() -> Self {
        Self {
            nation: Nations::Halana,
            nation_type: NationType::Normal,
            continent: Continents::Arcadia,
            major_area: None,
            capital: Some(Cities::Rookery),
            ruler: None,
            government: Some(Government::Theocracy),
            demonym: None,
            adjective: Some(vec!["Halana"]),
            language: None,
            religion: Some(vec![Deities::Pazuzu]),
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
    fn hermea() -> Self {
        Self {
            nation: Nations::Hermea,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::HighSeas),
            capital: Some(Cities::Promise),
            ruler: Some(Individuals::Mengkare),
            government: Some(Government::ContractualDictatorship),
            demonym: Some("Hermeans"),
            adjective: Some(vec!["Hermean"]),
            language: Some(vec![Languages::Common, Languages::Draconic]),
            religion: Some(vec![Deities::Atheism]),
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
    fn hwanggot() -> Self {
        Self {
            nation: Nations::Hwanggot,
            nation_type: NationType::Normal,
            continent: Continents::TianXia,
            major_area: None,
            capital: Some(Cities::Haseong),
            ruler: Some(Individuals::HyeonGeonJi),
            government: Some(Government::HereditaryMonarchy),
            demonym: Some("Hwans"),
            adjective: None,
            language: Some(vec![Languages::Hwan, Languages::Tien]),
            religion: Some(vec![
                Deities::Kofusachi,
                Deities::Shelyn,
                Deities::SunWukong,
            ]),
        }
    }
    fn hymbria() -> Self {
        Self {
            nation: Nations::Hymbria,
            nation_type: NationType::Normal,
            continent: Continents::Avistan,
            major_area: Some(MajorAreas::BrokenLands),
            capital: Some(Cities::Mimere),
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
}

    enum Oceans {
    Arcadian,
    Obari,
    Okaiyo,
    Antarkos,
    Embaral,
}
