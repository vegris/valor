// Придумать человеские имена
#[derive(Clone, Copy)]
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
	pub fn filename(self) -> &'static str {
		FILENAMES[self as usize]
	}
}

const FILENAMES: [&str; 25] = [
	"CmBkBch.pcx",
	"CmBkDes.pcx",
	"CmBkDrTr.pcx",
	"CmBkDrMt.pcx",
	"CmBkDrDd.pcx",
	"CmBkGrMt.pcx",
	"CmBkGrTr.pcx",
	"CmBkLava.pcx",
	"CmBkMag.pcx",
	"CmBkSnMt.pcx",
	"CmBkSntr.pcx",
	"CmBkSub.pcx",
	"CmBkSwmp.pcx",
	"CmBkFF.pcx",
	"CmBkRK.pcx",
	"CmBkMC.pcx",
	"CmBkLP.pcx",
	"CmBkHG.pcx",
	"CmBkCF.pcx",
	"CmBkEF.pcx",
	"CmBkFW.pcx",
	"CmBkCur.pcx",
	"CmBkRgh.pcx",
	"CmBkBoat.pcx",
	"CmBkDeck.pcx"
];
