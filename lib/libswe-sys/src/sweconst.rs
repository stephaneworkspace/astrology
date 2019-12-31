/*
 * Traditional astrology for rust
 * ==============================
 *
 * Rust library by Stéphane Bressani (s.bressani@bluewin.ch)
 *
 * Using swissephem c library by Astrodienst AG
 * by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)
 *
 * The source code is released under an MIT License, which allows it to be used
 * also on commercial projects. This software uses the swiss ephemeris which is
 * licensed GPL.
 *
 * Therefore, if you want to use astro_compute_swisseph in your commercial
 * projects, you must adhere to the GPL license or buy a Swiss Ephemeris
 * commercial license.
 */
extern crate strum;
use crate::swerust::handler_swe17::{split_deg, SplitDegResult};

#[derive(Debug, Clone, Display, EnumIter)]
pub enum Signs {
    Aries = 1,
    Taurus = 2,
    Gemini = 3,
    Cancer = 4,
    Leo = 5,
    Virgo = 6,
    Libra = 7,
    Scorpio = 8,
    Sagittarius = 9,
    Capricorn = 10,
    Aquarius = 11,
    Pisces = 12,
}

#[derive(Debug, Clone, Display, EnumIter, AsStaticStr)]
pub enum Bodies {
    EclNut = -1,
    Sun = 0,
    Moon = 1,
    Mercury = 2,
    Venus = 3,
    Mars = 4,
    Jupiter = 5,
    Saturn = 6,
    Uranus = 7,
    Neptune = 8,
    Pluto = 9,
    MeanNode = 10,
    TrueNode = 11,
    MeanApog = 12,
    OscuApog = 13,
    Earth = 14,
    Chiron = 15,
    Pholus = 16,
    Ceres = 17,
    Pallas = 18,
    Juno = 19,
    Vesta = 20,
    IntpApog = 21,
    IntpPerg = 22,
    NPlanets = 23,
    // SE_FICT_OFFSET = 40,
    // SE_NFICT_ELEM = 15,
    // SE_AST_OFFSET = 10000,
    /* Hamburger or Uranian "planets" */
    Cupido = 40,
    Hades = 41,
    Zeus = 42,
    Kronos = 43,
    Apollon = 44,
    Admetos = 45,
    Vulkanus = 46,
    Poseidon = 47,
    /* other fictitious bodies */
    Isis = 48,
    Nibiru = 49,
    Harrington = 50,
    NeptuneLeverrier = 51,
    NeptuneAdams = 52,
    PlutoLowell = 53,
    PlutoPickering = 54,
    /* Asteroid */
    AsteroidAstera = 10000 + 5,
    AsteroidHebe = 10000 + 6,
    AsteroidIris = 10000 + 7,
    AsteroidFlora = 10000 + 8,
    AsteroidMetis = 10000 + 9,
    AsteroidHygiea = 10000 + 10,
    AsteroidUrania = 10000 + 30,
    AsteroidIsis = 10000 + 42,
    AsteroidHilda = 10000 + 153,
    AsteroidPhilosophia = 10000 + 227,
    AsteroidSophia = 10000 + 251,
    AsteroidAletheia = 10000 + 259,
    AsteroidSapientia = 10000 + 275,
    AsteroidThule = 10000 + 279,
    AsteroidUrsula = 10000 + 375,
    AsteroidEros = 10000 + 433,
    AsteroidCupido = 10000 + 763,
    AsteroidHidalgo = 10000 + 944,
    AsteroidLilith = 10000 + 1181,
    AsteroidAmor = 10000 + 1221,
    AsteroidKama = 10000 + 1387,
    AsteroidAphrodite = 10000 + 1388,
    AsteroidApollo = 10000 + 1862,
    AsteroidDamocles = 10000 + 3553,
    AsteroidCruithne = 10000 + 3753,
    AsteroidPoseidon = 10000 + 4341,
    AsteroidVulcano = 10000 + 4464,
    AsteroidZeus = 10000 + 5731,
    AsteroidNessus = 10000 + 7066,
}

// Don't forgot to remove in exemple this structure
#[derive(Debug, PartialEq, Display, EnumIter)]
pub enum ObjectType {
    Unknown,
    PlanetOrStar,
    Earth,
    Fiction,
    Asteroid,
}

impl Bodies {
    pub fn object_type(self) -> ObjectType {
        match self {
            Bodies::EclNut => ObjectType::Unknown,
            Bodies::Sun => ObjectType::PlanetOrStar,
            Bodies::Moon => ObjectType::PlanetOrStar,
            Bodies::Mercury => ObjectType::PlanetOrStar,
            Bodies::Venus => ObjectType::PlanetOrStar,
            Bodies::Mars => ObjectType::PlanetOrStar,
            Bodies::Jupiter => ObjectType::PlanetOrStar,
            Bodies::Saturn => ObjectType::PlanetOrStar,
            Bodies::Uranus => ObjectType::PlanetOrStar,
            Bodies::Neptune => ObjectType::PlanetOrStar,
            Bodies::Pluto => ObjectType::PlanetOrStar,
            Bodies::MeanNode => ObjectType::PlanetOrStar,
            Bodies::TrueNode => ObjectType::PlanetOrStar,
            Bodies::MeanApog => ObjectType::PlanetOrStar,
            Bodies::OscuApog => ObjectType::PlanetOrStar,
            Bodies::Earth => ObjectType::Earth,
            Bodies::Chiron => ObjectType::Fiction,
            Bodies::Pholus => ObjectType::Fiction,
            Bodies::Ceres => ObjectType::Fiction,
            Bodies::Pallas => ObjectType::Fiction,
            Bodies::Juno => ObjectType::Fiction,
            Bodies::Vesta => ObjectType::Fiction,
            Bodies::IntpApog => ObjectType::Fiction,
            Bodies::IntpPerg => ObjectType::Fiction,
            Bodies::NPlanets => ObjectType::Fiction,
            Bodies::Cupido => ObjectType::Fiction,
            Bodies::Hades => ObjectType::Fiction,
            Bodies::Zeus => ObjectType::Fiction,
            Bodies::Kronos => ObjectType::Fiction,
            Bodies::Apollon => ObjectType::Fiction,
            Bodies::Admetos => ObjectType::Fiction,
            Bodies::Vulkanus => ObjectType::Fiction,
            Bodies::Poseidon => ObjectType::Fiction,
            Bodies::Isis => ObjectType::Fiction,
            Bodies::Nibiru => ObjectType::Fiction,
            Bodies::Harrington => ObjectType::Fiction,
            Bodies::NeptuneLeverrier => ObjectType::Fiction,
            Bodies::NeptuneAdams => ObjectType::Fiction,
            Bodies::PlutoLowell => ObjectType::Fiction,
            Bodies::PlutoPickering => ObjectType::Fiction,
            Bodies::AsteroidAstera => ObjectType::Asteroid,
            Bodies::AsteroidHebe => ObjectType::Asteroid,
            Bodies::AsteroidIris => ObjectType::Asteroid,
            Bodies::AsteroidFlora => ObjectType::Asteroid,
            Bodies::AsteroidMetis => ObjectType::Asteroid,
            Bodies::AsteroidHygiea => ObjectType::Asteroid,
            Bodies::AsteroidUrania => ObjectType::Asteroid,
            Bodies::AsteroidIsis => ObjectType::Asteroid,
            Bodies::AsteroidHilda => ObjectType::Asteroid,
            Bodies::AsteroidPhilosophia => ObjectType::Asteroid,
            Bodies::AsteroidSophia => ObjectType::Asteroid,
            Bodies::AsteroidAletheia => ObjectType::Asteroid,
            Bodies::AsteroidSapientia => ObjectType::Asteroid,
            Bodies::AsteroidThule => ObjectType::Asteroid,
            Bodies::AsteroidUrsula => ObjectType::Asteroid,
            Bodies::AsteroidEros => ObjectType::Asteroid,
            Bodies::AsteroidCupido => ObjectType::Asteroid,
            Bodies::AsteroidHidalgo => ObjectType::Asteroid,
            Bodies::AsteroidLilith => ObjectType::Asteroid,
            Bodies::AsteroidAmor => ObjectType::Asteroid,
            Bodies::AsteroidKama => ObjectType::Asteroid,
            Bodies::AsteroidAphrodite => ObjectType::Asteroid,
            Bodies::AsteroidApollo => ObjectType::Asteroid,
            Bodies::AsteroidDamocles => ObjectType::Asteroid,
            Bodies::AsteroidCruithne => ObjectType::Asteroid,
            Bodies::AsteroidPoseidon => ObjectType::Asteroid,
            Bodies::AsteroidVulcano => ObjectType::Asteroid,
            Bodies::AsteroidZeus => ObjectType::Asteroid,
            Bodies::AsteroidNessus => ObjectType::Asteroid,
        }
    }
}

#[derive(Debug)]
pub struct Object {
    object_name: String,
    object_type: ObjectType,
    longitude: f64,
    latitude: f64,
    split: SplitDegResult,
}

impl Object {
    pub fn new(
        object_name: &str,
        object_type: ObjectType,
        longitude: f64,
        latitude: f64,
    ) -> Object {
        Object {
            object_name: object_name.to_string(),
            object_type: object_type,
            longitude: longitude,
            latitude: latitude,
            split: split_deg(longitude, 0),
        }
    }
}

#[derive(Debug)]
pub struct House {
    object_id: i32,
    longitude: f64,
    split: SplitDegResult,
}

impl House {
    pub fn new(object_id: i32, longitude: f64) -> House {
        House {
            object_id: object_id,
            longitude: longitude,
            split: split_deg(longitude, 0),
        }
    }
}

pub enum Calandar {
    Julian = 0,
    Gregorian = 1,
}

pub enum OptionalFlag {
    JplEph = 1,
    SwissEph = 2,
    Moshier = 4,
    Heliocentric = 8,
    TruePosition = 16,
    J2000Equinox = 32,
    NoNutation = 64,
    Speed3 = 128,
    Speed = 256,
    NoGravitanionalDeflection = 512,
    NoAnnualAberration = 1024,
    AstronomicPosition = 1024 | 512,
    // AstronomicPosition = OptionalFlag::NoAnnualAberration
    //     | OptionalFlag::NoGravitanionalDeflection,
    EquatorialPosition = 2 * 1024,
    XYZCartesianNotPolarCoordinate = 4 * 1024,
    Radians = 8 * 1024,
    BarycentricPosition = 16 * 1024,
    TopocentricPosition = 32 * 1024,
    SideralPosition = 64 * 1024,
    ICRS = 128 * 1024,
    Dpsideps1980 = 256 * 1024,
    JplHorApprox = 512 * 1024,
}

/// I have put in enum only the most important houses methods
pub enum HouseSystem {
    Campanus,
    Equal,
    Koch,
    Placidus,
    Porphyrius,
    Regiomontanus,
    WholeSign,
}
/*
impl HouseSystem {
    /// Return unit for call in c
    fn hsys(self) -> char {
        match self {
            HouseSystem::Campanus => 'C',
            HouseSystem::Equal => 'E',
            HouseSystem::Koch => 'K',
            HouseSystem::Placidus => 'P',
            HouseSystem::Porphyrius => 'O',
            HouseSystem::Regiomontanus => 'R',
            HouseSystem::WholeSign => 'W',
        }
    }

    /// Return name of house in english
    fn name(self) -> &'static str {
        match self {
            HouseSystem::Campanus => "Campanus",
            HouseSystem::Equal => "Equal",
            HouseSystem::Koch => "Koch",
            HouseSystem::Placidus => "Placidus",
            HouseSystem::Porphyrius => "Porphyrius",
            HouseSystem::Regiomontanus => "Regiomontanus",
            HouseSystem::WholeSign => "Whole sign",
        }
    }
}*/
