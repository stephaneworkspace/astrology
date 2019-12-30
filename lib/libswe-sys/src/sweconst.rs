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

pub enum Calandar {
    Julian = 0,
    Gregorian = 1,
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
