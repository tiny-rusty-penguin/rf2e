
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
    CrystalFalls,
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

    // Cheliax
    AdivianRiver,
    AnferitaWood,
    ArgoIsle,
    Barrowood,
    BayOfDeng,
    BayOfSolva,
    BogRock,
    BrastleRiver,
    CapeDis,
    CapeErebus,
    CapeKraken,
    CliffsOfFury,
    Deepmar,
    DevilsPerch,
    Dhaenfens,
    DismalNitch,
    DoraciumRiver,
    Dunrock,
    EgobariusPlain,
    Eismonts,
    FatHarbor,
    FieldsOfChelam,
    GemcrownBay,
    HalikarnassosHills,
    HellmouthGulf,
    HesperethStrait,
    IseldRiver,
    JeniRiver,
    JewelcoastCape,
    KatharevousaRiver,
    KeelwrackHarbor,
    KeelwrackIsland,
    Kharijite,
    LostmastCape,
    MaiestasRiver,
    MalvesaRiver,
    MenadorMountains,
    MistshroudCliff,
    MountEmihym,
    MountNyisaid,
    MountRinia,
    MountSpire,
    NalKashel,
    NisrochBay,
    NorthPlains,
    OrneianCoast,
    PillarsOfAnferita,
    Pinch,
    RavounelForest,
    Rifardona,
    RikkanFalls,
    SallowCoast,
    SednaRiver,
    Shardstone,
    SirmiumPlains,
    LakeSorrow,
    SouthernUskRiver,
    Southrun,
    StavianArches,
    SteamingSea,
    Thuryan,
    TolsenPeak,
    TomarsulkRiver,
    TravekRiver,
    TuranianHills,
    VyreIsland,
    VyreRiver,
    WarlockIsland,
    Westchannel,
    WhalersPoint,
    WhisperRiver,
    Whisperwood,
    WinterGrove,
    WolfsTooth,
    YolubilisRiver,

    // Daggermark
    DaggerRiver,
    Shroudwood,

    // Druma
    LakeEncarthan,
    GreatGolodpanRiver,
    GulfOfAccord,
    MountSinatuk,
    PalakarForest,
    ProfitsFlow,

    // Five Kings Mountains
    BloodOfTheMountain,
    EmperorsPeak,
    GreatGoldpanRiver,
    LightningsCall,
    MountArugak,
    MountCarissa,
    MountGustus,
    MountKla,
    MountLangley,
    MountMist,
    MountOnik,
    MountSoryu,
    QueenPeak,
    ShapingBluffs,
    ToragsBreath,
    ValeOfShadows,

    // Galt
    Boarwood,
    FogPeaks,
    HorunPlain,
    SouthernHymbrianForest,
    KanteleRiver,
    MountCania,
    MountRein,
    NordeinGlacier,
    QiOamatokPass,
    StormfloodRiver,

    // Gravelands,
    Fangwood,
    PitOfZedoran,
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
    fn crystal_falls() -> Self {
        GeographyData {
            geography: Geography::CrystalFalls,
            geography_type: GeographyType::Waterfall,
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
            geography_type: GeographyType::Wetlands,
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

    // Cheliax
    fn adivian_river() -> Self {
        GeographyData {
            geography: Geography::AdivianRiver,
            geography_type: GeographyType::River,
        }
    }
    fn anferita_wood() -> Self {
        GeographyData {
            geography: Geography::AnferitaWood,
            geography_type: GeographyType::Forest,
        }
    }
    fn argo_isle() -> Self {
        GeographyData {
            geography: Geography::ArgoIsle,
            geography_type: GeographyType::Island,
        }
    }
    fn barrowood() -> Self {
        GeographyData {
            geography: Geography::Barrowood,
            geography_type: GeographyType::Forest,
        }
    }
    fn bay_of_deng() -> Self {
        GeographyData {
            geography: Geography::BayOfDeng,
            geography_type: GeographyType::Bay,
        }
    }
    fn bay_of_solva() -> Self {
        GeographyData {
            geography: Geography::BayOfSolva,
            geography_type: GeographyType::Bay,
        }
    }
    fn bog_rock() -> Self {
        GeographyData {
            geography: Geography::BogRock,
            geography_type: GeographyType::Island,
        }
    }
    fn brastle_river() -> Self {
        GeographyData {
            geography: Geography::BrastleRiver,
            geography_type: GeographyType::River,
        }
    }
    fn cape_dis() -> Self {
        GeographyData {
            geography: Geography::CapeDis,
            geography_type: GeographyType::Cape,
        }
    }
    fn cape_erebus() -> Self {
        GeographyData {
            geography: Geography::CapeErebus,
            geography_type: GeographyType::Cape,
        }
    }
    fn cape_kraken() -> Self {
        GeographyData {
            geography: Geography::CapeKraken,
            geography_type: GeographyType::Cape,
        }
    }
    fn cliffs_of_fury() -> Self {
        GeographyData {
            geography: Geography::CliffsOfFury,
            geography_type: GeographyType::Cliffs,
        }
    }
    fn deepmar() -> Self {
        GeographyData {
            geography: Geography::Deepmar,
            geography_type: GeographyType::Island,
        }
    }
    fn devils_perch() -> Self {
        GeographyData {
            geography: Geography::DevilsPerch,
            geography_type: GeographyType::MountainRange,
        }
    }
    fn dhaenfens() -> Self {
        GeographyData {
            geography: Geography::Dhaenfens,
            geography_type: GeographyType::Wetlands,
        }
    }
    fn dismal_nitch() -> Self {
        GeographyData {
            geography: Geography::DismalNitch,
            geography_type: GeographyType::Coast,
        }
    }
    fn doracium_river() -> Self {
        GeographyData {
            geography: Geography::DoraciumRiver,
            geography_type: GeographyType::River,
        }
    }
    fn dunrock() -> Self {
        GeographyData {
            geography: Geography::Dunrock,
            geography_type: GeographyType::Island,
        }
    }
    fn egobarius_plain() -> Self {
        GeographyData {
            geography: Geography::EgobariusPlain,
            geography_type: GeographyType::Plains,
        }
    }
    fn eismonts() -> Self {
        GeographyData {
            geography: Geography::Eismonts,
            geography_type: GeographyType::MountainRange,
        }
    }
    fn fat_harbor() -> Self {
        GeographyData {
            geography: Geography::FatHarbor,
            geography_type: GeographyType::Harbor,
        }
    }
    fn fields_of_chelam() -> Self {
        GeographyData {
            geography: Geography::FieldsOfChelam,
            geography_type: GeographyType::Plains,
        }
    }
    fn gemcrown_bay() -> Self {
        GeographyData {
            geography: Geography::GemcrownBay,
            geography_type: GeographyType::Bay,
        }
    }
    fn halikarnassos_hills() -> Self {
        GeographyData {
            geography: Geography::HalikarnassosHills,
            geography_type: GeographyType::Hills,
        }
    }
    fn hellmouth_gulf() -> Self {
        GeographyData {
            geography: Geography::HellmouthGulf,
            geography_type: GeographyType::Bay,
        }
    }
    fn hespereth_strait() -> Self {
        GeographyData {
            geography: Geography::HesperethStrait,
            geography_type: GeographyType::Strait,
        }
    }
    fn iseld_river() -> Self {
        GeographyData {
            geography: Geography::IseldRiver,
            geography_type: GeographyType::River,
        }
    }
    fn jeni_river() -> Self {
        GeographyData {
            geography: Geography::JeniRiver,
            geography_type: GeographyType::River,
        }
    }
    fn jewelcoast_cape() -> Self {
        GeographyData {
            geography: Geography::JewelcoastCape,
            geography_type: GeographyType::Cape,
        }
    }
    fn katharevousa_river() -> Self {
        GeographyData {
            geography: Geography::KatharevousaRiver,
            geography_type: GeographyType::River,
        }
    }
    fn keelwrack_harbor() -> Self {
        GeographyData {
            geography: Geography::KeelwrackHarbor,
            geography_type: GeographyType::Harbor,
        }
    }
    fn keelwrack_island() -> Self {
        GeographyData {
            geography: Geography::KeelwrackIsland,
            geography_type: GeographyType::Island,
        }
    }
    fn kharijite() -> Self {
        GeographyData {
            geography: Geography::Kharijite,
            geography_type: GeographyType::Region,
        }
    }
    fn lostmast_cape() -> Self {
        GeographyData {
            geography: Geography::LostmastCape,
            geography_type: GeographyType::Cape,
        }
    }
    fn maiestas_river() -> Self {
        GeographyData {
            geography: Geography::MaiestasRiver,
            geography_type: GeographyType::River,
        }
    }
    fn malvesa_river() -> Self {
        GeographyData {
            geography: Geography::MalvesaRiver,
            geography_type: GeographyType::River,
        }
    }
    fn menador_mountains() -> Self {
        GeographyData {
            geography: Geography::MenadorMountains,
            geography_type: GeographyType::MountainRange,
        }
    }
    fn mistshroud_cliff() -> Self {
        GeographyData {
            geography: Geography::MistshroudCliff,
            geography_type: GeographyType::Cliffs,
        }
    }
    fn mount_emihym() -> Self {
        GeographyData {
            geography: Geography::MountEmihym,
            geography_type: GeographyType::Mountain,
        }
    }
    fn mount_nyisaid() -> Self {
        GeographyData {
            geography: Geography::MountNyisaid,
            geography_type: GeographyType::Mountain,
        }
    }
    fn mount_rinia() -> Self {
        GeographyData {
            geography: Geography::MountRinia,
            geography_type: GeographyType::Mountain,
        }
    }
    fn mount_spire() -> Self {
        GeographyData {
            geography: Geography::MountSpire,
            geography_type: GeographyType::Mountain,
        }
    }
    fn nal_kashel() -> Self {
        GeographyData {
            geography: Geography::NalKashel,
            geography_type: GeographyType::Island,
        }
    }
    fn nisroch_bay() -> Self {
        GeographyData {
            geography: Geography::NisrochBay,
            geography_type: GeographyType::Bay,
        }
    }
    fn north_plains() -> Self {
        GeographyData {
            geography: Geography::NorthPlains,
            geography_type: GeographyType::Plains,
        }
    }
    fn orneian_coast() -> Self {
        GeographyData {
            geography: Geography::OrneianCoast,
            geography_type: GeographyType::Coast,
        }
    }
    fn pillars_of_anferita() -> Self {
        GeographyData {
            geography: Geography::PillarsOfAnferita,
            geography_type: GeographyType::RockFormation,
        }
    }
    fn pinch() -> Self {
        GeographyData {
            geography: Geography::Pinch,
            geography_type: GeographyType::Strait,
        }
    }
    fn ravounel_forest() -> Self {
        GeographyData {
            geography: Geography::RavounelForest,
            geography_type: GeographyType::Forest,
        }
    }
    fn rifardona() -> Self {
        GeographyData {
            geography: Geography::Rifardona,
            geography_type: GeographyType::Sandbar,
        }
    }
    fn sallow_coast() -> Self {
        GeographyData {
            geography: Geography::SallowCoast,
            geography_type: GeographyType::Coast,
        }
    }
    fn sedna_river() -> Self {
        GeographyData {
            geography: Geography::SednaRiver,
            geography_type: GeographyType::River,
        }
    }
    fn shardstone() -> Self {
        GeographyData {
            geography: Geography::Shardstone,
            geography_type: GeographyType::Island,
        }
    }
    fn sirmium_plains() -> Self {
        GeographyData {
            geography: Geography::SirmiumPlains,
            geography_type: GeographyType::Plains,
        }
    }
    fn lake_sorrow() -> Self {
        GeographyData {
            geography: Geography::LakeSorrow,
            geography_type: GeographyType::Lake,
        }
    }
    fn southern_usk_river() -> Self {
        GeographyData {
            geography: Geography::SouthernUskRiver,
            geography_type: GeographyType::River,
        }
    }
    fn southrun() -> Self {
        GeographyData {
            geography: Geography::Southrun,
            geography_type: GeographyType::River,
        }
    }
    fn stavian_arches() -> Self {
        GeographyData {
            geography: Geography::StavianArches,
            geography_type: GeographyType::Bridge,
        }
    }
    fn steaming_sea() -> Self {
        GeographyData {
            geography: Geography::SteamingSea,
            geography_type: GeographyType::Sea,
        }
    }
    fn thuryan() -> Self {
        GeographyData {
            geography: Geography::Thuryan,
            geography_type: GeographyType::Island,
        }
    }
    fn tolsen_peak() -> Self {
        GeographyData {
            geography: Geography::TolsenPeak,
            geography_type: GeographyType::Mountain,
        }
    }
    fn tomarsulk_river() -> Self {
        GeographyData {
            geography: Geography::TomarsulkRiver,
            geography_type: GeographyType::River,
        }
    }
    fn travek_river() -> Self {
        GeographyData {
            geography: Geography::TravekRiver,
            geography_type: GeographyType::River,
        }
    }
    fn turanian_hills() -> Self {
        GeographyData {
            geography: Geography::TuranianHills,
            geography_type: GeographyType::Hills,
        }
    }
    fn vyre_island() -> Self {
        GeographyData {
            geography: Geography::VyreIsland,
            geography_type: GeographyType::Island,
        }
    }
    fn vyre_river() -> Self {
        GeographyData {
            geography: Geography::VyreRiver,
            geography_type: GeographyType::River,
        }
    }
    fn warlock_island() -> Self {
        GeographyData {
            geography: Geography::WarlockIsland,
            geography_type: GeographyType::Island,
        }
    }
    fn westchannel() -> Self {
        GeographyData {
            geography: Geography::Westchannel,
            geography_type: GeographyType::Waterway,
        }
    }
    fn whalers_point() -> Self {
        GeographyData {
            geography: Geography::WhalersPoint,
            geography_type: GeographyType::Spit,
        }
    }
    fn whisper_river() -> Self {
        GeographyData {
            geography: Geography::WhisperRiver,
            geography_type: GeographyType::River,
        }
    }
    fn whisperwood() -> Self {
        GeographyData {
            geography: Geography::Whisperwood,
            geography_type: GeographyType::Forest,
        }
    }
    fn winter_grove() -> Self {
        GeographyData {
            geography: Geography::WinterGrove,
            geography_type: GeographyType::Grove,
        }
    }
    fn wolfs_tooth() -> Self {
        GeographyData {
            geography: Geography::WolfsTooth,
            geography_type: GeographyType::Island,
        }
    }

    // Daggermark
    fn dagger_river() -> Self {
        GeographyData {
            geography: Geography::DaggerRiver,
            geography_type: GeographyType::River,
        }
    }
    fn shroudwood() -> Self {
        GeographyData {
            geography: Geography::Shroudwood,
            geography_type: GeographyType::Forest,
        }
    }

    // Druma
    fn lake_encarthan() -> Self {
        GeographyData {
            geography: Geography::LakeEncarthan,
            geography_type: GeographyType::Lake,
        }
    }
    fn great_golodpan_river() -> Self {
        GeographyData {
            geography: Geography::GreatGolodpanRiver,
            geography_type: GeographyType::River,
        }
    }
    fn gulf_of_accord() -> Self {
        GeographyData {
            geography: Geography::GulfOfAccord,
            geography_type: GeographyType::Bay,
        }
    }
    fn mount_sinatuk() -> Self {
        GeographyData {
            geography: Geography::MountSinatuk,
            geography_type: GeographyType::Mountain,
        }
    }
    fn palakar_forest() -> Self {
        GeographyData {
            geography: Geography::PalakarForest,
            geography_type: GeographyType::Forest,
        }
    }
    fn profits_flow() -> Self {
        GeographyData {
            geography: Geography::ProfitsFlow,
            geography_type: GeographyType::River,
        }
    }

    // Five Kings Mountains
    fn blood_of_the_mountain() -> Self {
        GeographyData {
            geography: Geography::BloodOfTheMountain,
            geography_type: GeographyType::LavaLake,
        }
    }
    fn emperors_peak() -> Self {
        GeographyData {
            geography: Geography::EmperorsPeak,
            geography_type: GeographyType::Mountain,
        }
    }
    fn great_goldpan_river() -> Self {
        GeographyData {
            geography: Geography::GreatGoldpanRiver,
            geography_type: GeographyType::River,
        }
    }
    fn lightnings_call() -> Self {
        GeographyData {
            geography: Geography::LightningsCall,
            geography_type: GeographyType::Mountain,
        }
    }
    fn mount_arugak() -> Self {
        GeographyData {
            geography: Geography::MountArugak,
            geography_type: GeographyType::Mountain,
        }
    }
    fn mount_carissa() -> Self {
        GeographyData {
            geography: Geography::MountCarissa,
            geography_type: GeographyType::Mountain,
        }
    }
    fn mount_gustus() -> Self {
        GeographyData {
            geography: Geography::MountGustus,
            geography_type: GeographyType::Mountain,
        }
    }
    fn mount_kla() -> Self {
        GeographyData {
            geography: Geography::MountKla,
            geography_type: GeographyType::Mountain,
        }
    }
    fn mount_langley() -> Self {
        GeographyData {
            geography: Geography::MountLangley,
            geography_type: GeographyType::Mountain,
        }
    }
    fn mount_mist() -> Self {
        GeographyData {
            geography: Geography::MountMist,
            geography_type: GeographyType::Mountain,
        }
    }
    fn mount_onik() -> Self {
        GeographyData {
            geography: Geography::MountOnik,
            geography_type: GeographyType::Mountain,
        }
    }
    fn mount_soryu() -> Self {
        GeographyData {
            geography: Geography::MountSoryu,
            geography_type: GeographyType::Mountain,
        }
    }
    fn queen_peak() -> Self {
        GeographyData {
            geography: Geography::QueenPeak,
            geography_type: GeographyType::Mountain,
        }
    }
    fn shaping_bluffs() -> Self {
        GeographyData {
            geography: Geography::ShapingBluffs,
            geography_type: GeographyType::RockFormation,
        }
    }
    fn torags_breath() -> Self {
        GeographyData {
            geography: Geography::ToragsBreath,
            geography_type: GeographyType::AirRiver,
        }
    }
    fn vale_of_shadows() -> Self {
        GeographyData {
            geography: Geography::ValeOfShadows,
            geography_type: GeographyType::Valley,
        }
    }

    // Galt
    fn boarwood() -> Self {
        GeographyData {
            geography: Geography::Boarwood,
            geography_type: GeographyType::Forest,
        }
    }
    fn fog_peaks() -> Self {
        GeographyData {
            geography: Geography::FogPeaks,
            geography_type: GeographyType::MountainRange,
        }
    }
    fn horun_plain() -> Self {
        GeographyData {
            geography: Geography::HorunPlain,
            geography_type: GeographyType::Plains,
        }
    }
    fn southern_hymbrian_forest() -> Self {
        GeographyData {
            geography: Geography::SouthernHymbrianForest,
            geography_type: GeographyType::Forest,
        }
    }
    fn kantele_river() -> Self {
        GeographyData {
            geography: Geography::KanteleRiver,
            geography_type: GeographyType::River,
        }
    }
    fn mount_cania() -> Self {
        GeographyData {
            geography: Geography::MountCania,
            geography_type: GeographyType::Mountain,
        }
    }
    fn nordein_glacier() -> Self {
        GeographyData {
            geography: Geography::NordeinGlacier,
            geography_type: GeographyType::Glacier,
        }
    }
    fn qi_oamatok_pass() -> Self {
        GeographyData {
            geography: Geography::QiOamatokPass,
            geography_type: GeographyType::Pass,
        }
    }
    fn stormflood_river() -> Self {
        GeographyData {
            geography: Geography::StormfloodRiver,
            geography_type: GeographyType::River,
        }
    }

    // Gravelands
    fn fangwood() -> Self {
        GeographyData {
            geography: Geography::Fangwood,
            geography_type: GeographyType::Forest,
        }
    }
    fn pit_of_zedoran() -> Self {
        GeographyData {
            geography: Geography::PitOfZedoran,
            geography_type: GeographyType::Sinkhole,
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
    Cliffs,
    Wetlands,
    Coast,
    Strait,
    RockFormation,
    Sandbar,
    Bridge,
    Sea,
    Waterway,
    Spit,
    Grove,
    LavaLake,
    AirRiver,
    Glacier,
    Sinkhole,
    Pit,
    Fog,
    IceLake,
    Roads,
    Swamp,
}

