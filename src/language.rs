use strum::EnumIter;
use crate::Rarity;

#[derive(Debug, EnumIter)]
pub enum Languages {
    Adlet,
    Alghollthu,
    Amurrun,
    Arboreal,
    Boggard,
    Ceratioidi,
    Common,
    Cyclops,
    Draconic,
    Drooni,
    Dwarven,
    Elven,
    Ekujae,
    Fey,
    Garuda,
    Jotun,
    Girtablilu,
    Gnomish,
    Goblin,
    Goloma,
    Halfling,
    Iruxi,
    Kasatha,
    Kashrishi,
    Kech,
    Kholo,
    Kuru,
    Munavri,
    Nagaji,
    Orcish,
    PlantSpeech,
    Ysoki,
    Rougarou,
    Sahuagin,
    Samsaran,
    Sasquatch,
    Shoony,
    KasthezviSignLanguage,
    Sphinx,
    Strix,
    Syrinx,
    Tengu,
    Thriae,
    Tripkee,
    Vanara,
    Vishkanya,
    Wayang,
    Wildsong,

    Aklo,
    Caligni,
    Canto,
    DrowSignLanguage,
    FlailSnail,
    Gug,
    Necril,
    Orvian,
    Sakvroth,
    Vegepygmy,

    Calda,
    Dtang,
    Erutaki,
    Hallit,
    Hongali,
    Hwan,
    Iobarian,
    Kelish,
    Kibwani,
    Lirgeni,
    Minatan,
    Minkaian,
    Mwangi,
    Mzunu,
    Ocotan,
    Osiriani,
    Razatlani,
    Senzar,
    Shadowtonuge,
    Shoanti,
    SignLanguage,
    NapsuSignLanguage,
    Skald,
    Tang,
    Thassilonian,
    Tien,
    Varisian,
    Varki,
    Vudrani,
    Xanmba,

    AncientOsiriani,
    AncientAzlanti,
    Jistka,
    Shory,
    Tekritanin,

    Tanuki,
    Yaksha,
    Daemonic,
    Protean,
    Requian,
    Utopian,
    Anadi,
    Garundi,
    Destrachan,
    Dziriak,
    Jyoti,
    Anugobu,
    Kitsune,
    Muan,
    Petran,
    Pyric,
    Sussuran,
    Talican,
    Thalassic,
    Chthonian,
    Empyrean,
    Diabolic,
    Androffan,
    Grioth,
    Kovintal,
    MiGo,
    Shae,
    Vishkanyan,
    Yithian,
    Rasu,
    Shisk,
    Arcadian,
    Wyrwood,
    Akitonian,
    Formian,
    Ikeshti,
    Shobhad,
    Okaiyan,
    Ratajin,
    ElderThing,
    Surki,
    Lashunta,
}

enum LanguageType {
    Secret,
    Regional,
    Dead,
    Ancestry,
    Monster,
}

pub struct Language {
    pub language: Languages,
    pub str: &'static str,
    pub language_type: LanguageType,
    pub rarity: Rarity,
}

impl Language {
    pub fn all() -> Vec<Self> {
        vec![
            Self::adlet(),
            Self::alghollthu(),
            Self::amurrun(),
            Self::arboreal(),
            Self::boggard(),
            Self::ceratioidi(),
            Self::common(),
            Self::cyclops(),
            Self::draconic(),
            Self::drooni(),
            Self::dwarven(),
            Self::elven(),
            Self::ekujae(),
            Self::fey(),
            Self::garuda(),
            Self::jotun(),
            Self::girtablilu(),
            Self::gnomish(),
            Self::goblin(),
            Self::goloma(),
            Self::halfling(),
            Self::iruxi(),
            Self::kasatha(),
            Self::kashrishi(),
            Self::kech(),
            Self::kholo(),
            Self::kuru(),
            Self::munavri(),
            Self::nagaji(),
            Self::orcish(),
            Self::plant_speech(),
            Self::ysoki(),
            Self::rougarou(),
            Self::sahuagin(),
            Self::samsaran(),
            Self::sasquatch(),
            Self::shoony(),
            Self::kasthezvi_sign_language(),
            Self::sphinx(),
            Self::strix(),
            Self::syrinx(),
            Self::tengu(),
            Self::thriae(),
            Self::tripkee(),
            Self::vanara(),
            Self::vishkanya(),
            Self::wayang(),
            Self::wildsong(),
            Self::aklo(),
            Self::caligni(),
            Self::canto(),
            Self::drow_sign_language(),
            Self::flail_snail(),
            Self::gug(),
            Self::necril(),
            Self::protean(),
            Self::requian(),
            Self::utopian(),
            Self::anadi(),
            Self::garundi(),
            Self::destrachan(),
            Self::dziriak(),
            Self::jyoti(),
            Self::anugobu(),
            Self::kitsune(),
            Self::muan(),
            Self::petran(),
            Self::pyric(),
            Self::sussuran(),
            Self::talican(),
            Self::thalassic(),
            Self::chthonian(),
            Self::empyrean(),
            Self::diabolic(),
            Self::androffan(),
            Self::grioth(),
            Self::kovintal(),
            Self::mi_go(),
            Self::shae(),
            Self::vishkanyan(),
            Self::yithian(),
            Self::rasu(),
            Self::shisk(),
            Self::arcadian(),
            Self::wyrwood(),
            Self::akitonian(),
            Self::formian(),
            Self::ikeshti(),
            Self::shobhad(),
            Self::okaiyan(),
            Self::ratajin(),
            Self::elder_thing(),
            Self::surki(),
            Self::lashunta(),
            
        ]
    }

    pub fn str_to_language(s: &str) -> Option<Language> {
        match s {
            _ => None
        }
    }

    fn adlet() -> Self {
        Self {
            language: Languages::Adlet,
            str: "Adlet",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn alghollthu() -> Self {
        Self {
            language: Languages::Alghollthu,
            str: "Alghollthu",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn amurrun() -> Self {
        Self {
            language: Languages::Amurrun,
            str: "Amurrun",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Uncommon,
        }
    }
    fn arboreal() -> Self {
        Self {
            language: Languages::Arboreal,
            str: "Arboreal",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn boggard() -> Self {
        Self {
            language: Languages::Boggard,
            str: "Boggard",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn ceratioidi() -> Self {
        Self {
            language: Languages::Ceratioidi,
            str: "Ceratioidi",
            language_type: LanguageType::Monster,
            rarity: Rarity::Unknown,
        }
    }
    fn common() -> Self {
        Self {
            language: Languages::Common,
            str: "Common",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Common,

        }
    }
    fn cyclops() -> Self {
        Self {
            language: Languages::Cyclops,
            str: "Cyclops",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }

    fn draconic() -> Self {
        Self {
            language: Languages::Draconic,
            str: "Draconic",
            language_type: LanguageType::Monster,
            rarity: Rarity::Common,
        }
    }
    fn drooni() -> Self {
        Self {
            language: Languages::Drooni,
            str: "Drooni",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn dwarven() -> Self {
        Self {
            language: Languages::Dwarven,
            str: "Dwarven",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Common,
        }
    }
    fn elven() -> Self {
        Self {
            language: Languages::Elven,
            str: "Elven",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Common,
        }
    }
    fn ekujae() -> Self {
        Self {
            language: Languages::Ekujae,
            str: "Ekujae",
            language_type: LanguageType::Dead,
            rarity: Rarity::Uncommon,
        }
    }
    fn fey() -> Self {
        Self {
            language: Languages::Fey,
            str: "Fey",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Common,
        }
    }
    fn garuda() -> Self {
        Self {
            language: Languages::Garuda,
            str: "Garuda",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn jotun() -> Self {
        Self {
            language: Languages::Jotun,
            str: "Jotun",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Common,
        }
    }
    fn girtablilu() -> Self {
        Self {
            language: Languages::Girtablilu,
            str: "Girtablilu",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn gnomish() -> Self {
        Self {
            language: Languages::Gnomish,
            str: "Gnomish",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Common,
        }
    }
    fn goblin() -> Self {
        Self {
            language: Languages::Goblin,
            str: "Goblin",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Common,
        }
    }
    fn goloma() -> Self {
        Self {
            language: Languages::Goloma,
            str: "Goloma",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn halfling() -> Self {
        Self {
            language: Languages::Halfling,
            str: "Halfling",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Common,
        }
    }
    fn iruxi() -> Self {
        Self {
            language: Languages::Iruxi,
            str: "Iruxi",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Uncommon,
        }
    }
    fn kasatha() -> Self {
        Self {
            language: Languages::Kasatha,
            str: "Kasatha",
            language_type: LanguageType::Monster,
            rarity: Rarity::Unknown,
        }
    }
    fn kashrishi() -> Self {
        Self {
            language: Languages::Kashrishi,
            str: "Kashrishi",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Rare,
        }
    }
    fn kech() -> Self {
        Self {
            language: Languages::Kech,
            str: "Kech",
            language_type: LanguageType::Monster,
            rarity: Rarity::Unknown,
        }
    }
    fn kholo() -> Self {
        Self {
            language: Languages::Kholo,
            str: "Kholo",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Uncommon,
        }
    }
    fn kuru() -> Self {
        Self {
            language: Languages::Kuru,
            str: "Kuru",
            language_type: LanguageType::Monster,
            rarity: Rarity::Unknown,
        }
    }
    fn munavri() -> Self {
        Self {
            language: Languages::Munavri,
            str: "Munavri",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn nagaji() -> Self {
        Self {
            language: Languages::Nagaji,
            str: "Nagaji",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Uncommon,
        }
    }
    fn orcish() -> Self {
        Self {
            language: Languages::Orcish,
            str: "Orcish",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Common,
        }
    }
    fn plant_speech() -> Self {
        Self {
            language: Languages::PlantSpeech,
            str: "Plant Speech",
            language_type: LanguageType::Secret,
            rarity: Rarity::Unknown,
        }
    }
    fn ysoki() -> Self {
        Self {
            language: Languages::Ysoki,
            str: "Ysoki",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Uncommon,
        }
    }
    fn rougarou() -> Self {
        Self {
            language: Languages::Rougarou,
            str: "Rougarou",
            language_type: LanguageType::Monster,
            rarity: Rarity::Unknown,
        }
    }
    fn sahuagin() -> Self {
        Self {
            language: Languages::Sahuagin,
            str: "Sahuagin",
            language_type: LanguageType::Monster,
            rarity: Rarity::Unknown,
        }
    }
    fn samsaran() -> Self {
        Self {
            language: Languages::Samsaran,
            str: "Samsaran",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Rare,
        }
    }
    fn sasquatch() -> Self {
        Self {
            language: Languages::Sasquatch,
            str: "Sasquatch",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn shoony() -> Self {
        Self {
            language: Languages::Shoony,
            str: "Shoony",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Uncommon,
        }
    }
    fn kasthezvi_sign_language() -> Self {
        Self {
            language: Languages::KasthezviSignLanguage,
            str: "Kasthezvi Sign Language",
            language_type: LanguageType::Monster,
            rarity: Rarity::Unknown,
        }
    }
    fn sphinx() -> Self {
        Self {
            language: Languages::Sphinx,
            str: "Sphinx",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn strix() -> Self {
        Self {
            language: Languages::Strix,
            str: "Strix",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Uncommon,
        }
    }
    fn syrinx() -> Self {
        Self {
            language: Languages::Syrinx,
            str: "Syrinx",
            language_type: LanguageType::Monster,
            rarity: Rarity::Unknown,
        }
    }
    fn tengu() -> Self {
        Self {
            language: Languages::Tengu,
            str: "Tengu",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Uncommon,
        }
    }
    fn thriae() -> Self {
        Self {
            language: Languages::Thriae,
            str: "Thriae",
            language_type: LanguageType::Monster,
            rarity: Rarity::Unknown,
        }
    }
    fn tripkee() -> Self {
        Self {
            language: Languages::Tripkee,
            str: "Tripkee",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Uncommon,
        }
    }
    fn vanara() -> Self {
        Self {
            language: Languages::Vanara,
            str: "Vanara",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Rare,
        }
    }
    fn vishkanya() -> Self {
        Self {
            language: Languages::Vishkanya,
            str: "Vishkanya",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn wayang() -> Self {
        Self {
            language: Languages::Wayang,
            str: "Wayang",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Common,
        }
    }
    fn wildsong() -> Self {
        Self {
            language: Languages::Wildsong,
            str: "Wildsong",
            language_type: LanguageType::Secret,
            rarity: Rarity::Unknown,
        }
    }
    fn aklo() -> Self {
        Self {
            language: Languages::Aklo,
            str: "Aklo",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Uncommon,
        }
    }
    fn caligni() -> Self {
        Self {
            language: Languages::Caligni,
            str: "Caligni",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn canto() -> Self {
        Self {
            language: Languages::Canto,
            str: "Canto",
            language_type: LanguageType::Monster,
            rarity: Rarity::Unknown,
        }
    }
    fn drow_sign_language() -> Self {
        Self {
            language: Languages::DrowSignLanguage,
            str: "Drow Sign Language",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn flail_snail() -> Self {
        Self {
            language: Languages::FlailSnail,
            str: "Flail Snail",
            language_type: LanguageType::Monster,
            rarity: Rarity::Unknown,
        }
    }
    fn gug() -> Self {
        Self {
            language: Languages::Gug,
            str: "Gug",
            language_type: LanguageType::Monster,
            rarity: Rarity::Unknown,
        }
    }
    fn necril() -> Self {
        Self {
            language: Languages::Necril,
            str: "Necril",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Uncommon,
        }
    }
    fn orvian() -> Self {
        Self {
            language: Languages::Orvian,
            str: "Orvian",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn calda() -> Self {
        Self {
            language: Languages::Calda,
            str: "Calda",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }
    fn dtang() -> Self {
        Self {
            language: Languages::Dtang,
            str: "Dtang",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn erutaki() -> Self {
        Self {
            language: Languages::Erutaki,
            str: "Erutaki",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn hallit() -> Self {
        Self {
            language: Languages::Hallit,
            str: "Hallit",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn hongali() -> Self {
        Self {
            language: Languages::Hongali,
            str: "Hongali",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn hwan() -> Self {
        Self {
            language: Languages::Hwan,
            str: "Hwan",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn iobarian() -> Self {
        Self {
            language: Languages::Iobarian,
            str: "Iobarian",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn kelish() -> Self {
        Self {
            language: Languages::Kelish,
            str: "Kelish",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn kibwani() -> Self {
        Self {
            language: Languages::Kibwani,
            str: "Kibwani",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn lirgeni() -> Self {
        Self {
            language: Languages::Lirgeni,
            str: "Lirgeni",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn minatan() -> Self {
        Self {
            language: Languages::Minatan,
            str: "Minatan",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn minkaian() -> Self {
        Self {
            language: Languages::Minkaian,
            str: "Minkaian",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn mwangi() -> Self {
        Self {
            language: Languages::Mwangi,
            str: "Mwangi",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn mzunu() -> Self {
        Self {
            language: Languages::Mzunu,
            str: "Mzunu",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn ocotan() -> Self {
        Self {
            language: Languages::Ocotan,
            str: "Ocotan",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn osiriani() -> Self {
        Self {
            language: Languages::Osiriani,
            str: "Osiriani",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn razatlani() -> Self {
        Self {
            language: Languages::Razatlani,
            str: "Razatlani",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn senzar() -> Self {
        Self {
            language: Languages::Senzar,
            str: "Senzar",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn shadowtounge() -> Self {
        Self {
            language: Languages::Shadowtonuge,
            str: "Shadowtonuge",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn shoanti() -> Self {
        Self {
            language: Languages::Shoanti,
            str: "Shoanti",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn sign_language() -> Self {
        Self {
            language: Languages::SignLanguage,
            str: "SignLanguage",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn napsu_sign_language() -> Self {
        Self {
            language: Languages::NapsuSignLanguage,
            str: "NapsuSignLanguage",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn skald() -> Self {
        Self {
            language: Languages::Skald,
            str: "Skald",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn tang() -> Self {
        Self {
            language: Languages::Tang,
            str: "Tang",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn thassilonian() -> Self {
        Self {
            language: Languages::Thassilonian,
            str: "Thassilonian",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn tien() -> Self {
        Self {
            language: Languages::Tien,
            str: "Tien",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn varisian() -> Self {
        Self {
            language: Languages::Varisian,
            str: "Varisian",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn varki() -> Self {
        Self {
            language: Languages::Varki,
            str: "Varki",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn vudrani() -> Self {
        Self {
            language: Languages::Vudrani,
            str: "Vudrani",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }

    fn xanmba() -> Self {
        Self {
            language: Languages::Xanmba,
            str: "Xanmba",
            language_type: LanguageType::Regional,
            rarity: Rarity::Unknown,
        }
    }
    fn ancient_osiriani() -> Self {
        Self {
            language: Languages::AncientOsiriani,
            str: "Ancient Osiriani",
            language_type: LanguageType::Dead,
            rarity: Rarity::Rare,
        }
    }
    fn ancient_azlanti() -> Self {
        Self {
            language: Languages::AncientAzlanti,
            str: "Ancient Azlanti",
            language_type: LanguageType::Dead,
            rarity: Rarity::Uncommon,
        }
    }
    fn jistka() -> Self {
        Self {
            language: Languages::Jistka,
            str: "Jistka",
            language_type: LanguageType::Dead,
            rarity: Rarity::Uncommon,
        }
    }
    fn shory() -> Self {
        Self {
            language: Languages::Shory,
            str: "Shory",
            language_type: LanguageType::Dead,
            rarity: Rarity::Unknown,
        }
    }
    fn tekritanin() -> Self {
        Self {
            language: Languages::Tekritanin,
            str: "Tekritanin",
            language_type: LanguageType::Dead,
            rarity: Rarity::Unknown,
        }
    }
    fn tanuki() -> Self {
        Self {
            language: Languages::Tanuki,
            str: "Tanuki",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Common,
        }
    }
    fn yaksha() -> Self {
        Self {
            language: Languages::Yaksha,
            str: "Yaksha",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Common,
        }
    }
    fn daemonic() -> Self {
        Self {
            language: Languages::Daemonic,
            str: "Daemonic",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn protean() -> Self {
        Self {
            language: Languages::Protean,
            str: "Protean",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn requian() -> Self {
        Self {
            language: Languages::Requian,
            str: "Requian",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn utopian() -> Self {
        Self {
            language: Languages::Utopian,
            str: "Utopian",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Uncommon,
        }
    }
    fn anadi() -> Self {
        Self {
            language: Languages::Anadi,
            str: "Anadi",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Uncommon,
        }
    }
    fn garundi() -> Self {
        Self {
            language: Languages::Garundi,
            str: "Garundi",
            language_type: LanguageType::Regional,
            rarity: Rarity::Uncommon,
        }
    }
    fn destrachan() -> Self {
        Self {
            language: Languages::Destrachan,
            str: "Destrachan",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn dziriak() -> Self {
        Self {
            language: Languages::Dziriak,
            str: "D'ziriak",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn jyoti() -> Self {
        Self {
            language: Languages::Jyoti,
            str: "Jyoti",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn anugobu() -> Self {
        Self {
            language: Languages::Anugobu,
            str: "Anugobu",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn kitsune() -> Self {
        Self {
            language: Languages::Kitsune,
            str: "Kitsune",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn muan() -> Self {
        Self {
            language: Languages::Muan,
            str: "Muan",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn petran() -> Self {
        Self {
            language: Languages::Petran,
            str: "Petran",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn pyric() -> Self {
        Self {
            language: Languages::Pyric,
            str: "Pyric",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn sussuran() -> Self {
        Self {
            language: Languages::Sussuran,
            str: "Sussuran",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn talican() -> Self {
        Self {
            language: Languages::Talican,
            str: "Talican",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn thalassic() -> Self {
        Self {
            language: Languages::Thalassic,
            str: "Thalassic",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Uncommon,
        }
    }
    fn chthonian() -> Self {
        Self {
            language: Languages::Chthonian,
            str: "Chthonian",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn empyrean() -> Self {
        Self {
            language: Languages::Empyrean,
            str: "Empyrean",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn diabolic() -> Self {
        Self {
            language: Languages::Diabolic,
            str: "Diabolic",
            language_type: LanguageType::Monster,
            rarity: Rarity::Uncommon,
        }
    }
    fn androffan() -> Self {
        Self {
            language: Languages::Androffan,
            str: "Androffan",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Rare,
        }
    }
    fn grioth() -> Self {
        Self {
            language: Languages::Grioth,
            str: "Grioth",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn kovintal() -> Self {
        Self {
            language: Languages::Kovintal,
            str: "Kovintal",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn mi_go() -> Self {
        Self {
            language: Languages::MiGo,
            str: "Mi-Go",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn shae() -> Self {
        Self {
            language: Languages::Shae,
            str: "Shae",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn vishkanyan() -> Self {
        Self {
            language: Languages::Vishkanyan,
            str: "Vishkanyan",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn yithian() -> Self {
        Self {
            language: Languages::Yithian,
            str: "Yithian",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn rasu() -> Self {
        Self {
            language: Languages::Rasu,
            str: "Rasu",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Rare,
        }
    }
    fn shisk() -> Self {
        Self {
            language: Languages::Shisk,
            str: "Shisk",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Rare,
        }
    }
    fn arcadian() -> Self {
        Self {
            language: Languages::Arcadian,
            str: "Arcadian",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Rare,
        }
    }
    fn wyrwood() -> Self {
        Self {
            language: Languages::Wyrwood,
            str: "Wyrwood",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Rare,
        }
    }
    fn akitonian() -> Self {
        Self {
            language: Languages::Akitonian,
            str: "Akitonian",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Rare,
        }
    }
    fn formian() -> Self {
        Self {
            language: Languages::Formian,
            str: "Formian",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn ikeshti() -> Self {
        Self {
            language: Languages::Ikeshti,
            str: "Ikeshti",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn shobhad() -> Self {
        Self {
            language: Languages::Shobhad,
            str: "Shobhad",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn okaiyan() -> Self {
        Self {
            language: Languages::Okaiyan,
            str: "Okaiyan",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Rare,
        }
    }
    fn ratajin() -> Self {
        Self {
            language: Languages::Ratajin,
            str: "Ratajin",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn elder_thing() -> Self {
        Self {
            language: Languages::ElderThing,
            str: "Elder Thing",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    fn surki() -> Self {
        Self {
            language: Languages::Surki,
            str: "Surki",
            language_type: LanguageType::Ancestry,
            rarity: Rarity::Rare,
        }
    }
    fn lashunta() -> Self {
        Self {
            language: Languages::Lashunta,
            str: "Lashunta",
            language_type: LanguageType::Monster,
            rarity: Rarity::Rare,
        }
    }
    
}