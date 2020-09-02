// Придумать человеские имена
#[derive(Clone, Copy)]
#[allow(unused)]
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
	DECK
}

impl Battlefield {
	pub const fn filename(self) -> &'static str {
		match self {
			Battlefield::BCH => "CmBkBch.pcx",
			Battlefield::DES => "CmBkDes.pcx",
			Battlefield::DRTR => "CmBkDrTr.pcx",
			Battlefield::DRMT => "CmBkDrMt.pcx",
			Battlefield::DRDD => "CmBkDrDd.pcx",
			Battlefield::GRMT => "CmBkGrMt.pcx",
			Battlefield::GRTR => "CmBkGrTr.pcx",
			Battlefield::LAVA => "CmBkLava.pcx",
			Battlefield::MAG => "CmBkMag.pcx",
			Battlefield::SNMT => "CmBkSnMt.pcx",
			Battlefield::SNTR => "CmBkSntr.pcx",
			Battlefield::SUB => "CmBkSub.pcx",
			Battlefield::SWMP => "CmBkSwmp.pcx",
			Battlefield::FF => "CmBkFF.pcx",
			Battlefield::RK => "CmBkRK.pcx",
			Battlefield::MC => "CmBkMC.pcx",
			Battlefield::LP => "CmBkLP.pcx",
			Battlefield::HG => "CmBkHG.pcx",
			Battlefield::CF => "CmBkCF.pcx",
			Battlefield::EF => "CmBkEF.pcx",
			Battlefield::FW => "CmBkFW.pcx",
			Battlefield::CUR => "CmBkCur.pcx",
			Battlefield::RGH => "CmBkRgh.pcx",
			Battlefield::BOAT => "CmBkBoat.pcx",
			Battlefield::DECK => "CmBkDeck.pcx"
		}
	}
}
