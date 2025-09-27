use crate::to_string;

to_string! {
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Character {
        Bloodhound,
        Gibraltar,
        Lifeline,
        Pathfinder,
        Wraith,
        Bangalore,
        Caustic,
        Mirage,
        Octane,
        Wattson,
        Crypto,
        Revenant,
        Loba,
        Rampart,
        Horizon,
        Fuse,
        Valkyrie,
        Seer,
        Ash,
        MadMaggie,
        Newcastle,
        Vantage,
        Catalyst,
        Ballistic,
        Conduit,
        Alter,

        /// Also global.
        Unknown,
    }
}
