use serde::Deserialize;

// Придумать человеские имена
#[derive(Clone, Copy, Deserialize)]
pub enum Battlefield {
    BCH,
    DES,
    DRTR,
    DRMT,
    DRDD,
    GRMT,
    GRTR,
    LAVA,
    MAG,
    SNMT,
    SNTR,
    SUB,
    SWMP,
    FF,
    RK,
    MC,
    LP,
    HG,
    CF,
    EF,
    FW,
    CUR,
    RGH,
    BOAT,
    DECK,
}

impl Battlefield {
    pub const fn filename(self) -> &'static str {
        match self {
            Self::BCH => "CmBkBch.pcx",
            Self::DES => "CmBkDes.pcx",
            Self::DRTR => "CmBkDrTr.pcx",
            Self::DRMT => "CmBkDrMt.pcx",
            Self::DRDD => "CmBkDrDd.pcx",
            Self::GRMT => "CmBkGrMt.pcx",
            Self::GRTR => "CmBkGrTr.pcx",
            Self::LAVA => "CmBkLava.pcx",
            Self::MAG => "CmBkMag.pcx",
            Self::SNMT => "CmBkSnMt.pcx",
            Self::SNTR => "CmBkSntr.pcx",
            Self::SUB => "CmBkSub.pcx",
            Self::SWMP => "CmBkSwmp.pcx",
            Self::FF => "CmBkFF.pcx",
            Self::RK => "CmBkRK.pcx",
            Self::MC => "CmBkMC.pcx",
            Self::LP => "CmBkLP.pcx",
            Self::HG => "CmBkHG.pcx",
            Self::CF => "CmBkCF.pcx",
            Self::EF => "CmBkEF.pcx",
            Self::FW => "CmBkFW.pcx",
            Self::CUR => "CmBkCur.pcx",
            Self::RGH => "CmBkRgh.pcx",
            Self::BOAT => "CmBkBoat.pcx",
            Self::DECK => "CmBkDeck.pcx",
        }
    }
}
