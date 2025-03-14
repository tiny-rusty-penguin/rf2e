pub enum Geography {
    // Avistan
    // Absalom
    AbsalomHarbor,
    ArazlantMox,
    Cairnlands,
    Dunmire,
    HollowfrostPass,
    Immenwood,
    IsleOfErran,
    IsleOfKortos,
    KortosMounts,
    MountGanog,
    PariolIsland,
    PilotIsland,
    RivenHills,
    Scrape,
    StarstoneIsles,
    Swardlands,
    WeepingGrandfather,

    // Andoran
    AndoshenRiver,
    ArsenalIsland,
    ArthfellForest,
    ArthfellMountains,
    ArthfellRiver,
    ArthroshRiver,
    AspoBay,
    AspodellMountains,
    AspodellPass,
    BenedictsHope,
    CandlestoneCaverns,
    CapeOfHope,
    CarpendenPlains,
    ColdforgeRiver,
    DarkmoonPlain,
    DarkmoonRiver,
    DarkmoonVale,
    DarkmoonWood,
    DragonflyRiver,
    DroskarsCrag,
    Droskmere,
    ElberwickRise,
    FiveKingsRange,
    FoamRiver,
    FreshHarbor,
    GoldFalls,
    GoldmeltRiver,
    GreatSaltHarbor,
    KeldRiver,
    KerniteRiver,
    KingtowerPass,
    MissaShelf,
    MoultonRiver,
    NogorthaPeaks,
    Perch,
    SeliansWatch,
    SellenRiver,
    StarBay,
    TamuthRiver,
    VerduranForest,
    WolfrunHills,
    WolfrunRapids,

    // Belkzen
    RiverEsk,
    FloodRoad,
    Hopespring,
    KestrelRiver,
    PathRiver,
    RakeIsland,
    RedwoolRiver,
    CleftboneCanyon,
    KeeningHills,
    KodarMountains,
    MindspinMountains,
    Skittermounds,
    Smokespur,
    TuskMountains,
    WhisperfallPass,
    AlgidWastes,
    BloodPlains,
    DirtSea,
    GapOfValballus,
    GhostlightMarsh,
    UrglinGap,
    WitchgateForest,

    // Brevoy
    AcubenIsle,
    AwzeraRiver,
    ChoralRiver,
    CrookedFalls,
    EastSellenRiver,
    GolushkinMountains,
    GrayRiver,
    GronziForest,
    HillsOfNomen,
    IcerimePeaks,
    IcerimeRiver,
    Issia,
    JellicoeBay,
    LakeOfMistsAndVeils,
    LakeReykal,
    LittleIcerimeRiver,
    MountVeshka,
    RavenroostRiver,
    Rostland,
    RostlandPlains,
    ShrikeRiver,
    SilverRiver,
    StetvenRiver,
    UvallHills,
    ValleyOfFire,
    WinterbreakBay,
}

struct GeographyData {
    geography: Geography,
    geography_type: GeographyType,
}

impl GeographyData {
    // Absalom
    fn absalom_harbor() -> Self {
        GeographyData {
            geography: Geography::AbsalomHarbor,
            geography_type: GeographyType::Harbor,
        }
    }
    fn arazlant_mox() -> Self {
        GeographyData {
            geography: Geography::ArazlantMox,
            geography_type: GeographyType::Mountain,
        }
    }
    fn cairnlands() -> Self {
        GeographyData {
            geography: Geography::Cairnlands,
            geography_type: GeographyType::Region,
        }
    }
    fn dunmire() -> Self {
        GeographyData {
            geography: Geography::Dunmire,
            geography_type: GeographyType::Bog,
        }
    }
    fn hollowfrost_pass() -> Self {
        GeographyData {
            geography: Geography::HollowfrostPass,
            geography_type: GeographyType::Pass,
        }
    }
    fn immenwood() -> Self {
        GeographyData {
            geography: Geography::Immenwood,
            geography_type: GeographyType::Forest,
        }
    }
    fn isle_of_erran() -> Self {
        GeographyData {
            geography: Geography::IsleOfErran,
            geography_type: GeographyType::Island,
        }
    }
    fn isle_of_kortos() -> Self {
        GeographyData {
            geography: Geography::IsleOfKortos,
            geography_type: GeographyType::Island,
        }
    }
    fn kortos_mounts() -> Self {
        GeographyData {
            geography: Geography::KortosMounts,
            geography_type: GeographyType::MountainRange,
        }
    }
    fn mount_ganog() -> Self {
        GeographyData {
            geography: Geography::MountGanog,
            geography_type: GeographyType::Mountain,
        }
    }
    fn pariol_island() -> Self {
        GeographyData {
            geography: Geography::PariolIsland,
            geography_type: GeographyType::Island,
        }
    }
    fn pilot_island() -> Self {
        GeographyData {
            geography: Geography::PilotIsland,
            geography_type: GeographyType::Island,
        }
    }
    fn riven_hills() -> Self {
        GeographyData {
            geography: Geography::RivenHills,
            geography_type: GeographyType::Hills,
        }
    }
    fn scrape() -> Self {
        GeographyData {
            geography: Geography::Scrape,
            geography_type: GeographyType::Desert,
        }
    }
    fn starstone_isles() -> Self {
        GeographyData {
            geography: Geography::StarstoneIsles,
            geography_type: GeographyType::Isles,
        }
    }
    fn swardlands() -> Self {
        GeographyData {
            geography: Geography::Swardlands,
            geography_type: GeographyType::Plains,
        }
    }
    fn weeping_grandfather() -> Self {
        GeographyData {
            geography: Geography::WeepingGrandfather,
            geography_type: GeographyType::Mountain,
        }
    }

    // Andoran
    fn andoshen_river() -> Self {
        GeographyData {
            geography: Geography::AndoshenRiver,
            geography_type: GeographyType::River,
        }
    }
    fn arsenal_island() -> Self {
        GeographyData {
            geography: Geography::ArsenalIsland,
            geography_type: GeographyType::Island,
        }
    }
    fn arthfell_forest() -> Self {
        GeographyData {
            geography: Geography::ArthfellForest,
            geography_type: GeographyType::Forest,
        }
    }
    fn arthfell_mountains() -> Self {
        GeographyData {
            geography: Geography::ArthfellMountains,
            geography_type: GeographyType::MountainRange,
        }
    }
    fn arthfell_river() -> Self {
        GeographyData {
            geography: Geography::ArthfellRiver,
            geography_type: GeographyType::River,
        }
    }
    fn arthrosh_river() -> Self {
        GeographyData {
            geography: Geography::ArthroshRiver,
            geography_type: GeographyType::River,
        }
    }
    fn aspo_bay() -> Self {
        GeographyData {
            geography: Geography::AspoBay,
            geography_type: GeographyType::Bay,
        }
    }
    fn aspodell_mountains() -> Self {
        GeographyData {
            geography: Geography::AspodellMountains,
            geography_type: GeographyType::MountainRange,
        }
    }
    fn aspodell_pass() -> Self {
        GeographyData {
            geography: Geography::AspodellPass,
            geography_type: GeographyType::Pass,
        }
    }
    fn benedicts_hope() -> Self {
        GeographyData {
            geography: Geography::BenedictsHope,
            geography_type: GeographyType::Island,
        }
    }
    fn candlestone_caverns() -> Self {
        GeographyData {
            geography: Geography::CandlestoneCaverns,
            geography_type: GeographyType::Caves,
        }
    }
    fn cape_of_hope() -> Self {
        GeographyData {
            geography: Geography::CapeOfHope,
            geography_type: GeographyType::Cape,
        }
    }
    fn carpenden_plains() -> Self {
        GeographyData {
            geography: Geography::CarpendenPlains,
            geography_type: GeographyType::Plains,
        }
    }
    fn coldforge_river() -> Self {
        GeographyData {
            geography: Geography::ColdforgeRiver,
            geography_type: GeographyType::River,
        }
    }
    fn darkmoon_plain() -> Self {
        GeographyData {
            geography: Geography::DarkmoonPlain,
            geography_type: GeographyType::Plains,
        }
    }
    fn darkmoon_river() -> Self {
        GeographyData {
            geography: Geography::DarkmoonRiver,
            geography_type: GeographyType::River,
        }
    }
    fn darkmoon_vale() -> Self {
        GeographyData {
            geography: Geography::DarkmoonVale,
            geography_type: GeographyType::Region,
        }
    }
    fn darkmoon_wood() -> Self {
        GeographyData {
            geography: Geography::DarkmoonWood,
            geography_type: GeographyType::Forest,
        }
    }
    fn dragonfly_river() -> Self {
        GeographyData {
            geography: Geography::DragonflyRiver,
            geography_type: GeographyType::River,
        }
    }
    fn droskars_crag() -> Self {
        GeographyData {
            geography: Geography::DroskarsCrag,
            geography_type: GeographyType::Volcano,
        }
    }
    fn droskmere() -> Self {
        GeographyData {
            geography: Geography::Droskmere,
            geography_type: GeographyType::Lake,
        }
    }
    fn elberwick_rise() -> Self {
        GeographyData {
            geography: Geography::ElberwickRise,
            geography_type: GeographyType::Shelf,
        }
    }
    fn five_kings_range() -> Self {
        GeographyData {
            geography: Geography::FiveKingsRange,
            geography_type: GeographyType::MountainRange,
        }
    }
    fn foam_river() -> Self {
        GeographyData {
            geography: Geography::FoamRiver,
            geography_type: GeographyType::River,
        }
    }
    fn fresh_harbor() -> Self {
        GeographyData {
            geography: Geography::FreshHarbor,
            geography_type: GeographyType::Harbor,
        }
    }
    fn gold_falls() -> Self {
        GeographyData {
            geography: Geography::GoldFalls,
            geography_type: GeographyType::Waterfall,
        }
    }
    fn goldmelt_river() -> Self {
        GeographyData {
            geography: Geography::GoldmeltRiver,
            geography_type: GeographyType::River,
        }
    }
    fn great_salt_harbor() -> Self {
        GeographyData {
            geography: Geography::GreatSaltHarbor,
            geography_type: GeographyType::Harbor,
        }
    }
    fn keld_river() -> Self {
        GeographyData {
            geography: Geography::KeldRiver,
            geography_type: GeographyType::River,
        }
    }
    fn kernite_river() -> Self {
        GeographyData {
            geography: Geography::KerniteRiver,
            geography_type: GeographyType::River,
        }
    }
    fn kingtower_pass() -> Self {
        GeographyData {
            geography: Geography::KingtowerPass,
            geography_type: GeographyType::Pass,
        }
    }
    fn missa_shelf() -> Self {
        GeographyData {
            geography: Geography::MissaShelf,
            geography_type: GeographyType::Plains,
        }
    }
    fn moulton_river() -> Self {
        GeographyData {
            geography: Geography::MoultonRiver,
            geography_type: GeographyType::River,
        }
    }
    fn nogortha_peaks() -> Self {
        GeographyData {
            geography: Geography::NogorthaPeaks,
            geography_type: GeographyType::MountainRange,
        }
    }
    fn perch() -> Self {
        GeographyData {
            geography: Geography::Perch,
            geography_type: GeographyType::Hill,
        }
    }
    fn selians_watch() -> Self {
        GeographyData {
            geography: Geography::SeliansWatch,
            geography_type: GeographyType::Island,
        }
    }
    fn sellen_river() -> Self {
        GeographyData {
            geography: Geography::SellenRiver,
            geography_type: GeographyType::River,
        }
    }
    fn star_bay() -> Self {
        GeographyData {
            geography: Geography::StarBay,
            geography_type: GeographyType::Bay,
        }
    }
    fn tamuth_river() -> Self {
        GeographyData {
            geography: Geography::TamuthRiver,
            geography_type: GeographyType::River,
        }
    }
    fn verduran_forest() -> Self {
        GeographyData {
            geography: Geography::VerduranForest,
            geography_type: GeographyType::Forest,
        }
    }
    fn wolfrun_hills() -> Self {
        GeographyData {
            geography: Geography::WolfrunHills,
            geography_type: GeographyType::Hills,
        }
    }
    fn wolfrun_rapids() -> Self {
        GeographyData {
            geography: Geography::WolfrunRapids,
            geography_type: GeographyType::Rapids,
        }
    }

    // Belkzen
    fn river_esk() -> Self {
        GeographyData {
            geography: Geography::RiverEsk,
            geography_type: GeographyType::River,
        }
    }
    fn flood_road() -> Self {
        GeographyData {
            geography: Geography::FloodRoad,
            geography_type: GeographyType::Road,
        }
    }
    fn hopespring() -> Self {
        GeographyData {
            geography: Geography::Hopespring,
            geography_type: GeographyType::Spring,
        }
    }
    fn kestrel_river() -> Self {
        GeographyData {
            geography: Geography::KestrelRiver,
            geography_type: GeographyType::River,
        }
    }
    fn path_river() -> Self {
        GeographyData {
            geography: Geography::PathRiver,
            geography_type: GeographyType::River,
        }
    }
    fn rake_island() -> Self {
        GeographyData {
            geography: Geography::RakeIsland,
            geography_type: GeographyType::Island,
        }
    }
    fn redwool_river() -> Self {
        GeographyData {
            geography: Geography::RedwoolRiver,
            geography_type: GeographyType::River,
        }
    }
    fn cleftbone_canyon() -> Self {
        GeographyData {
            geography: Geography::CleftboneCanyon,
            geography_type: GeographyType::Canyon,
        }
    }
    fn keening_hills() -> Self {
        GeographyData {
            geography: Geography::KeeningHills,
            geography_type: GeographyType::Hills,
        }
    }
    fn kodar_mountains() -> Self {
        GeographyData {
            geography: Geography::KodarMountains,
            geography_type: GeographyType::MountainRange,
        }
    }
    fn mindspin_mountains() -> Self {
        GeographyData {
            geography: Geography::MindspinMountains,
            geography_type: GeographyType::MountainRange,
        }
    }
    fn skittermounds() -> Self {
        GeographyData {
            geography: Geography::Skittermounds,
            geography_type: GeographyType::Hills,
        }
    }
    fn smokespur() -> Self {
        GeographyData {
            geography: Geography::Smokespur,
            geography_type: GeographyType::Region,
        }
    }
    fn tusk_mountains() -> Self {
        GeographyData {
            geography: Geography::TuskMountains,
            geography_type: GeographyType::MountainRange,
        }
    }
    fn whisperfall_pass() -> Self {
        GeographyData {
            geography: Geography::WhisperfallPass,
            geography_type: GeographyType::Pass,
        }
    }
    fn algid_wastes() -> Self {
        GeographyData {
            geography: Geography::AlgidWastes,
            geography_type: GeographyType::Wastes,
        }
    }
    fn blood_plains() -> Self {
        GeographyData {
            geography: Geography::BloodPlains,
            geography_type: GeographyType::Region,
        }
    }
    fn dirt_sea() -> Self {
        GeographyData {
            geography: Geography::DirtSea,
            geography_type: GeographyType::Marsh,
        }
    }
    fn gap_of_valballus() -> Self {
        GeographyData {
            geography: Geography::GapOfValballus,
            geography_type: GeographyType::Pass,
        }
    }
    fn ghostlight_marsh() -> Self {
        GeographyData {
            geography: Geography::GhostlightMarsh,
            geography_type: GeographyType::Marsh,
        }
    }
    fn urglin_gap() -> Self {
        GeographyData {
            geography: Geography::UrglinGap,
            geography_type: GeographyType::Plains,
        }
    }
    fn witchgate_forest() -> Self {
        GeographyData {
            geography: Geography::WitchgateForest,
            geography_type: GeographyType::Forest,
        }
    }

    // Brevoy
    fn acuben_isle() -> Self {
        GeographyData {
            geography: Geography::AcubenIsle,
            geography_type: GeographyType::Island,
        }
    }
    fn awzera_river() -> Self {
        GeographyData {
            geography: Geography::AwzeraRiver,
            geography_type: GeographyType::River,
        }
    }
    fn choral_river() -> Self {
        GeographyData {
            geography: Geography::ChoralRiver,
            geography_type: GeographyType::River,
        }
    }
    fn crooked_falls() -> Self {
        GeographyData {
            geography: Geography::CrookedFalls,
            geography_type: GeographyType::Waterfall,
        }
    }
    fn east_sellen_river() -> Self {
        GeographyData {
            geography: Geography::EastSellenRiver,
            geography_type: GeographyType::River,
        }
    }
    fn golushkin_mountains() -> Self {
        GeographyData {
            geography: Geography::GolushkinMountains,
            geography_type: GeographyType::MountainRange,
        }
    }
    fn gray_river() -> Self {
        GeographyData {
            geography: Geography::GrayRiver,
            geography_type: GeographyType::River,
        }
    }
    fn gronzi_forest() -> Self {
        GeographyData {
            geography: Geography::GronziForest,
            geography_type: GeographyType::Forest,
        }
    }
    fn hills_of_nomen() -> Self {
        GeographyData {
            geography: Geography::HillsOfNomen,
            geography_type: GeographyType::Hills,
        }
    }
    fn icerime_peaks() -> Self {
        GeographyData {
            geography: Geography::IcerimePeaks,
            geography_type: GeographyType::MountainRange,
        }
    }
    fn icerime_river() -> Self {
        GeographyData {
            geography: Geography::IcerimeRiver,
            geography_type: GeographyType::River,
        }
    }
    fn issia() -> Self {
        GeographyData {
            geography: Geography::Issia,
            geography_type: GeographyType::Region,
        }
    }
    fn jellicoe_bay() -> Self {
        GeographyData {
            geography: Geography::JellicoeBay,
            geography_type: GeographyType::Bay,
        }
    }
    fn lake_of_mists_and_veils() -> Self {
        GeographyData {
            geography: Geography::LakeOfMistsAndVeils,
            geography_type: GeographyType::Lake,
        }
    }
    fn lake_reykal() -> Self {
        GeographyData {
            geography: Geography::LakeReykal,
            geography_type: GeographyType::Lake,
        }
    }
    fn little_icerime_river() -> Self {
        GeographyData {
            geography: Geography::LittleIcerimeRiver,
            geography_type: GeographyType::River,
        }
    }
    fn mount_veshka() -> Self {
        GeographyData {
            geography: Geography::MountVeshka,
            geography_type: GeographyType::Mountain,
        }
    }
    fn ravenroost_river() -> Self {
        GeographyData {
            geography: Geography::RavenroostRiver,
            geography_type: GeographyType::River,
        }
    }
    fn rostland() -> Self {
        GeographyData {
            geography: Geography::Rostland,
            geography_type: GeographyType::Region,
        }
    }
    fn rostland_plains() -> Self {
        GeographyData {
            geography: Geography::RostlandPlains,
            geography_type: GeographyType::Plains,
        }
    }
    fn shrike_river() -> Self {
        GeographyData {
            geography: Geography::ShrikeRiver,
            geography_type: GeographyType::River,
        }
    }
    fn silver_river() -> Self {
        GeographyData {
            geography: Geography::SilverRiver,
            geography_type: GeographyType::River,
        }
    }
    fn stetven_river() -> Self {
        GeographyData {
            geography: Geography::StetvenRiver,
            geography_type: GeographyType::River,
        }
    }
    fn uvall_hills() -> Self {
        GeographyData {
            geography: Geography::UvallHills,
            geography_type: GeographyType::Hills,
        }
    }
    fn valley_of_fire() -> Self {
        GeographyData {
            geography: Geography::ValleyOfFire,
            geography_type: GeographyType::Valley,
        }
    }
    fn winterbreak_bay() -> Self {
        GeographyData {
            geography: Geography::WinterbreakBay,
            geography_type: GeographyType::Bay,
        }
    }
}

enum GeographyType {
    Harbor,
    Mountain,
    Region,
    Bog,
    Pass,
    Forest,
    Island,
    MountainRange,
    Hills,
    Desert,
    Isles,
    Plains,
    River,
    Bay,
    Caves,
    Cape,
    Volcano,
    Lake,
    Shelf,
    Waterfall,
    Hill,
    Rapids,
    Road,
    Spring,
    Canyon,
    Wastes,
    Marsh,
    Valley,
}

