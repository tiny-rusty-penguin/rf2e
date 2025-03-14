use crate::Individuals;
use crate::location::nations::Nations;

pub enum Settlements {
    // Absalom
    Abberton,
    Absalom,
    Arbo,
    Bosco,
    Brume,
    Castinlee,
    Cawshax,
    Copperwood,
    Dawnfoot,
    Diobel,
    Elyon,
    Escadar,
    Ferny,
    Flesk,
    Galizhur,
    Hazrak,
    Kerrick,
    MattenCleave,
    Meravon,
    Otari,
    PiersEnd,
    Shoreline,
    Stump,
    TurpinRowe,
    Westerhold,
    Willowside,

    // Andoran
    Almas,
    Alvis,
    Augustana,
    Bellis,
    Caldamin,
    Carpenden,
    ChimeraCove,
    Claes,
    Cyremium,
    Elesomare,
    FalconsHollow,
    Falconridge,
    Fusil,
    Lavieton,
    Occarin,
    Olfden,
    Oregent,
    PirensBluff,
    Rippleden,
    Riverford,
    Sauerton,
    Souston,
    Steyr,
    Triela,
    Whiterush,
    Wittleshine,

    // Artume
    Artume,

    // Belkzen
    Blisterwell,
    TheFoundry,
    Kragnaroth,
    Truau,
    Deepgate,
    Wyvernsting,
    Urgir,
    DawnsReach,
    FreedomTown,
    Olvoro,
    Ossogard,

    // Brevoy
    Brunderton,
    EaglesWatch,
    FortSerenko,
    Golushkin,
    Grayhaven,
    Highdelve,
    NewStetven,
    NivaktasCrossing,
    PortIce,
    Restov,
    Silverhall,
    Skywatch,
    Stoneclimb,
    Winterbreak,
    Zmeyka,

    // mixed
    Nagisa,
    AlkenstarCity,
    Peijita,
    Bloodcove,
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
    XinBakrakhan,
    XinCyrusian,
    XinGastash,
    XinHaruka,
}

struct Settlement {
    nation: Nations,
    population: Option<u32>,
    ruler: Option<Individuals>,
    location: (f32, f32),
}

impl Settlement {
    // Absalom
    fn abberton() -> Self {
        Self {
            nation: Nations::Absalom,
            ruler: Some(Individuals::JaeAbber),
            population: None,
            location: (32.2019448, -0.5569667),

        }
    }
    fn absalom() -> Self {
        Self {
            nation: Nations::Absalom,
            population: Some(306_900),
            ruler: Some(Individuals::Gyr),
            location: (30.8886260, -0.2343082),
        }
    }
    fn arbo() -> Self {
        Self {
            nation: Nations::Absalom,
            population: Some(117),
            ruler: None,
            location: (31.107471,-1.279781),
        }
    }
    fn bosco() -> Self {
        Self {
            nation: Nations::Absalom,
            population: None,
            ruler: None,
            location: (31.0209980, 0.3843493),
        }
    }
    fn brume() -> Self {
        Self {
            nation: Nations::Absalom,
            population: None,
            ruler: Some(Individuals::PaldreshaAtterbay),
            location: (31.072528,-1.215423),
        }
    }
    fn castinlee() -> Self {
        Self {
            nation: Nations::Absalom,
            population: Some(2_080),
            ruler: Some(Individuals::JulpinCrags),
            location: (30.985473,-1.501801),
        }
    }
    fn cawshax() -> Self {
        Self {
            nation: Nations::Absalom,
            population: Some(316),
            ruler: Some(Individuals::HanleyCasterog),
            location: (31.013166,-1.348706),
        }
    }
    fn copperwood() -> Self {
        Self {
            nation: Nations::Absalom,
            population: None,
            ruler: None,
            location: (30.9167269, -0.1923561),
        }
    }
    fn dawnfoot() -> Self {
        Self {
            nation: Nations::Absalom,
            population: None,
            ruler: None,
            location: (30.8139625, -0.1865311),
        }
    }
    fn diobel() -> Self {
        Self {
            nation: Nations::Absalom,
            population: Some(4_850),
            ruler: Some(Individuals::AvidArnsen),
            location: (30.6563241, -1.6269120),
        }
    }
    fn elyon() -> Self {
        Self {
            nation: Nations::Absalom,
            population: None,
            ruler: None,
            location: (31.2280318, -1.9510144),
        }
    }
    fn escadar() -> Self {
        Self {
            nation: Nations::Absalom,
            population: Some(11_700),
            ruler: None,
            location: (31.9528186, -0.9840943),
        }
    }
    fn ferny() -> Self {
        Self {
            nation: Nations::Absalom,
            population: Some(100),
            ruler: None,
            location: (31.226350, -1.469111),
        }
    }
    fn flesk() -> Self {
        Self {
            nation: Nations::Absalom,
            population: None,
            ruler: None,
            location: (31.4294676, 0.6018789),
        }
    }
    fn galizhur() -> Self {
        Self {
            nation: Nations::Absalom,
            population: None,
            ruler: None,
            location: (30.5093000, -1.2984935),
        }
    }
    fn hazrak() -> Self {
        Self {
            nation: Nations::Absalom,
            population: None,
            ruler: None,
            location: (31.3873385, -0.0566420),
        }
    }
    fn kerrick() -> Self {
        Self {
            nation: Nations::Absalom,
            population: Some(3_533),
            ruler: Some(Individuals::PerivarAltrusi),
            location: (31.1478822, -1.6600111),
        }
    }
    fn matten_cleave() -> Self {
        Self {
            nation: Nations::Absalom,
            population: Some(2_134),
            ruler: None,
            location: (31.218330, -1.539802),
        }
    }
    fn meravon() -> Self {
        Self {
            nation: Nations::Absalom,
            population: None,
            ruler: None,
            location: (31.0032731,-1.0003321),
        }
    }
    fn otari() -> Self {
        Self {
            nation: Nations::Absalom,
            population: Some(1_240),
            ruler: Some(Individuals::LardusLongsaddle),
            location: (30.7873323,-0.9350036),
        }
    }
    fn piers_end() -> Self {
        Self {
            nation: Nations::Absalom,
            population: Some(78),
            ruler: Some(Individuals::RegisCoombs),
            location: (31.6928526, -0.8317509),
        }
    }
    fn shoreline() -> Self {
        Self {
            nation: Nations::Absalom,
            population: None,
            ruler: None,
            location: (30.8734142, -0.2954081),
        }
    }
    fn stump() -> Self {
        Self {
            nation: Nations::Absalom,
            population: Some(50),
            ruler: Some(Individuals::ElithuVargan),
            location: (31.061932, -1.298404),
        }
    }
    fn turpin_rowe() -> Self {
        Self {
            nation: Nations::Absalom,
            population: Some(862),
            ruler: None,
            location: (30.961751, -1.301262),
        }
    }
    fn westerhold() -> Self {
        Self {
            nation: Nations::Absalom,
            population: None,
            ruler: None,
            location: (30.9082246, -0.2913800),
        }
    }
    fn willowside() -> Self {
        Self {
            nation: Nations::Absalom,
            population: Some(1_831),
            ruler: Some(Individuals::EstessaVandy),
            location: (31.5221041, -1.1983603),
        }
    }
}