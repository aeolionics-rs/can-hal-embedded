#[repr(C)]
#[doc = "CHANNEL\\[%s\\]"]
#[doc(alias = "CHANNEL")]
pub struct Channel {
    chctrla: Chctrla,
    chctrlb: Chctrlb,
    chevctrl: Chevctrl,
    chintenclr: Chintenclr,
    chintenset: Chintenset,
    chintf: Chintf,
    chssa: Chssa,
    chdsa: Chdsa,
    chsstrd: Chsstrd,
    chdstrd: Chdstrd,
    chxsiz: Chxsiz,
    chpdat: Chpdat,
    chctrlcrc: Chctrlcrc,
    chcrcdat: Chcrcdat,
    chnxt: Chnxt,
    chllcfgstat: Chllcfgstat,
    chstatbc: Chstatbc,
    chstatcc: Chstatcc,
    chstat: Chstat,
}
impl Channel {
    #[doc = "0x00 - CHANNEL CONTROL REGISTER A"]
    #[inline(always)]
    pub const fn chctrla(&self) -> &Chctrla {
        &self.chctrla
    }
    #[doc = "0x04 - CHANNEL CONTROL REGISTER B"]
    #[inline(always)]
    pub const fn chctrlb(&self) -> &Chctrlb {
        &self.chctrlb
    }
    #[doc = "0x08 - CHANNEL EVENT CONTROL REGISTER"]
    #[inline(always)]
    pub const fn chevctrl(&self) -> &Chevctrl {
        &self.chevctrl
    }
    #[doc = "0x0c - CHANNEL INTERRUPT ENABLE CLEAR REGISTER"]
    #[inline(always)]
    pub const fn chintenclr(&self) -> &Chintenclr {
        &self.chintenclr
    }
    #[doc = "0x10 - CHANNEL INTERRUPT ENABLE SET REGISTER"]
    #[inline(always)]
    pub const fn chintenset(&self) -> &Chintenset {
        &self.chintenset
    }
    #[doc = "0x14 - CHANNEL INTERRUPT FLAG REGISTER"]
    #[inline(always)]
    pub const fn chintf(&self) -> &Chintf {
        &self.chintf
    }
    #[doc = "0x18 - CHANNEL SOURCE START ADDRESS"]
    #[inline(always)]
    pub const fn chssa(&self) -> &Chssa {
        &self.chssa
    }
    #[doc = "0x1c - CHANNEL DESTINATION START ADDRESS"]
    #[inline(always)]
    pub const fn chdsa(&self) -> &Chdsa {
        &self.chdsa
    }
    #[doc = "0x20 - CHANNEL SOURCE CELL STRIDE SIZE REGISTER"]
    #[inline(always)]
    pub const fn chsstrd(&self) -> &Chsstrd {
        &self.chsstrd
    }
    #[doc = "0x24 - CHANNEL DESTINATION CELL STRIDE SIZE REGISTER"]
    #[inline(always)]
    pub const fn chdstrd(&self) -> &Chdstrd {
        &self.chdstrd
    }
    #[doc = "0x28 - CHANNEL TRANSFER SIZE REGISTER"]
    #[inline(always)]
    pub const fn chxsiz(&self) -> &Chxsiz {
        &self.chxsiz
    }
    #[doc = "0x2c - CHANNEL PATTERN MATCH DATA"]
    #[inline(always)]
    pub const fn chpdat(&self) -> &Chpdat {
        &self.chpdat
    }
    #[doc = "0x30 - CHANNEL CONTROL CRC"]
    #[inline(always)]
    pub const fn chctrlcrc(&self) -> &Chctrlcrc {
        &self.chctrlcrc
    }
    #[doc = "0x34 - CHANNEL CRC/CHECKSUM DATA REGISTER"]
    #[inline(always)]
    pub const fn chcrcdat(&self) -> &Chcrcdat {
        &self.chcrcdat
    }
    #[doc = "0x38 - CHANNEL NEXT DESCRIPTOR ADDRESS POINTER"]
    #[inline(always)]
    pub const fn chnxt(&self) -> &Chnxt {
        &self.chnxt
    }
    #[doc = "0x3c - CHANNEL LINKED LIST CONFIGURATION STATUS REGISTER"]
    #[inline(always)]
    pub const fn chllcfgstat(&self) -> &Chllcfgstat {
        &self.chllcfgstat
    }
    #[doc = "0x40 - CHANNEL STATUS BLOCK COUNT REGISTER"]
    #[inline(always)]
    pub const fn chstatbc(&self) -> &Chstatbc {
        &self.chstatbc
    }
    #[doc = "0x44 - CHANNEL STATUS CELL COUNT REGISTER"]
    #[inline(always)]
    pub const fn chstatcc(&self) -> &Chstatcc {
        &self.chstatcc
    }
    #[doc = "0x48 - CHANNEL STATUS REGISTER"]
    #[inline(always)]
    pub const fn chstat(&self) -> &Chstat {
        &self.chstat
    }
}
#[doc = "CHCTRLA (rw) register accessor: CHANNEL CONTROL REGISTER A\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctrla`] module"]
#[doc(alias = "CHCTRLA")]
pub type Chctrla = crate::Reg<chctrla::ChctrlaSpec>;
#[doc = "CHANNEL CONTROL REGISTER A"]
pub mod chctrla;
#[doc = "CHCTRLB (rw) register accessor: CHANNEL CONTROL REGISTER B\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctrlb`] module"]
#[doc(alias = "CHCTRLB")]
pub type Chctrlb = crate::Reg<chctrlb::ChctrlbSpec>;
#[doc = "CHANNEL CONTROL REGISTER B"]
pub mod chctrlb;
#[doc = "CHEVCTRL (rw) register accessor: CHANNEL EVENT CONTROL REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chevctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chevctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chevctrl`] module"]
#[doc(alias = "CHEVCTRL")]
pub type Chevctrl = crate::Reg<chevctrl::ChevctrlSpec>;
#[doc = "CHANNEL EVENT CONTROL REGISTER"]
pub mod chevctrl;
#[doc = "CHINTENCLR (rw) register accessor: CHANNEL INTERRUPT ENABLE CLEAR REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chintenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chintenclr`] module"]
#[doc(alias = "CHINTENCLR")]
pub type Chintenclr = crate::Reg<chintenclr::ChintenclrSpec>;
#[doc = "CHANNEL INTERRUPT ENABLE CLEAR REGISTER"]
pub mod chintenclr;
#[doc = "CHINTENSET (rw) register accessor: CHANNEL INTERRUPT ENABLE SET REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chintenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chintenset`] module"]
#[doc(alias = "CHINTENSET")]
pub type Chintenset = crate::Reg<chintenset::ChintensetSpec>;
#[doc = "CHANNEL INTERRUPT ENABLE SET REGISTER"]
pub mod chintenset;
#[doc = "CHINTF (rw) register accessor: CHANNEL INTERRUPT FLAG REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chintf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chintf`] module"]
#[doc(alias = "CHINTF")]
pub type Chintf = crate::Reg<chintf::ChintfSpec>;
#[doc = "CHANNEL INTERRUPT FLAG REGISTER"]
pub mod chintf;
#[doc = "CHSSA (rw) register accessor: CHANNEL SOURCE START ADDRESS\n\nYou can [`read`](crate::Reg::read) this register and get [`chssa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chssa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chssa`] module"]
#[doc(alias = "CHSSA")]
pub type Chssa = crate::Reg<chssa::ChssaSpec>;
#[doc = "CHANNEL SOURCE START ADDRESS"]
pub mod chssa;
#[doc = "CHDSA (rw) register accessor: CHANNEL DESTINATION START ADDRESS\n\nYou can [`read`](crate::Reg::read) this register and get [`chdsa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdsa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdsa`] module"]
#[doc(alias = "CHDSA")]
pub type Chdsa = crate::Reg<chdsa::ChdsaSpec>;
#[doc = "CHANNEL DESTINATION START ADDRESS"]
pub mod chdsa;
#[doc = "CHSSTRD (rw) register accessor: CHANNEL SOURCE CELL STRIDE SIZE REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chsstrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsstrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsstrd`] module"]
#[doc(alias = "CHSSTRD")]
pub type Chsstrd = crate::Reg<chsstrd::ChsstrdSpec>;
#[doc = "CHANNEL SOURCE CELL STRIDE SIZE REGISTER"]
pub mod chsstrd;
#[doc = "CHDSTRD (rw) register accessor: CHANNEL DESTINATION CELL STRIDE SIZE REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chdstrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdstrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdstrd`] module"]
#[doc(alias = "CHDSTRD")]
pub type Chdstrd = crate::Reg<chdstrd::ChdstrdSpec>;
#[doc = "CHANNEL DESTINATION CELL STRIDE SIZE REGISTER"]
pub mod chdstrd;
#[doc = "CHXSIZ (rw) register accessor: CHANNEL TRANSFER SIZE REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chxsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chxsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chxsiz`] module"]
#[doc(alias = "CHXSIZ")]
pub type Chxsiz = crate::Reg<chxsiz::ChxsizSpec>;
#[doc = "CHANNEL TRANSFER SIZE REGISTER"]
pub mod chxsiz;
#[doc = "CHPDAT (rw) register accessor: CHANNEL PATTERN MATCH DATA\n\nYou can [`read`](crate::Reg::read) this register and get [`chpdat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chpdat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chpdat`] module"]
#[doc(alias = "CHPDAT")]
pub type Chpdat = crate::Reg<chpdat::ChpdatSpec>;
#[doc = "CHANNEL PATTERN MATCH DATA"]
pub mod chpdat;
#[doc = "CHCTRLCRC (rw) register accessor: CHANNEL CONTROL CRC\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrlcrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrlcrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctrlcrc`] module"]
#[doc(alias = "CHCTRLCRC")]
pub type Chctrlcrc = crate::Reg<chctrlcrc::ChctrlcrcSpec>;
#[doc = "CHANNEL CONTROL CRC"]
pub mod chctrlcrc;
#[doc = "CHCRCDAT (rw) register accessor: CHANNEL CRC/CHECKSUM DATA REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chcrcdat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chcrcdat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chcrcdat`] module"]
#[doc(alias = "CHCRCDAT")]
pub type Chcrcdat = crate::Reg<chcrcdat::ChcrcdatSpec>;
#[doc = "CHANNEL CRC/CHECKSUM DATA REGISTER"]
pub mod chcrcdat;
#[doc = "CHNXT (rw) register accessor: CHANNEL NEXT DESCRIPTOR ADDRESS POINTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chnxt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnxt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnxt`] module"]
#[doc(alias = "CHNXT")]
pub type Chnxt = crate::Reg<chnxt::ChnxtSpec>;
#[doc = "CHANNEL NEXT DESCRIPTOR ADDRESS POINTER"]
pub mod chnxt;
#[doc = "CHLLCFGSTAT (r) register accessor: CHANNEL LINKED LIST CONFIGURATION STATUS REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chllcfgstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chllcfgstat`] module"]
#[doc(alias = "CHLLCFGSTAT")]
pub type Chllcfgstat = crate::Reg<chllcfgstat::ChllcfgstatSpec>;
#[doc = "CHANNEL LINKED LIST CONFIGURATION STATUS REGISTER"]
pub mod chllcfgstat;
#[doc = "CHSTATBC (r) register accessor: CHANNEL STATUS BLOCK COUNT REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatbc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chstatbc`] module"]
#[doc(alias = "CHSTATBC")]
pub type Chstatbc = crate::Reg<chstatbc::ChstatbcSpec>;
#[doc = "CHANNEL STATUS BLOCK COUNT REGISTER"]
pub mod chstatbc;
#[doc = "CHSTATCC (r) register accessor: CHANNEL STATUS CELL COUNT REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatcc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chstatcc`] module"]
#[doc(alias = "CHSTATCC")]
pub type Chstatcc = crate::Reg<chstatcc::ChstatccSpec>;
#[doc = "CHANNEL STATUS CELL COUNT REGISTER"]
pub mod chstatcc;
#[doc = "CHSTAT (r) register accessor: CHANNEL STATUS REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chstat`] module"]
#[doc(alias = "CHSTAT")]
pub type Chstat = crate::Reg<chstat::ChstatSpec>;
#[doc = "CHANNEL STATUS REGISTER"]
pub mod chstat;
