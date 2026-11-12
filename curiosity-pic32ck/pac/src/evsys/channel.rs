#[repr(C)]
#[doc = "CHANNEL\\[%s\\]"]
#[doc(alias = "CHANNEL")]
pub struct Channel {
    chctrl: Chctrl,
    chintenclr: Chintenclr,
    chintenset: Chintenset,
    chintflag: Chintflag,
    chstatus: Chstatus,
}
impl Channel {
    #[doc = "0x00 - Channel n Control"]
    #[inline(always)]
    pub const fn chctrl(&self) -> &Chctrl {
        &self.chctrl
    }
    #[doc = "0x04 - Channel n Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn chintenclr(&self) -> &Chintenclr {
        &self.chintenclr
    }
    #[doc = "0x05 - Channel n Interrupt Enable Set"]
    #[inline(always)]
    pub const fn chintenset(&self) -> &Chintenset {
        &self.chintenset
    }
    #[doc = "0x06 - Channel n Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn chintflag(&self) -> &Chintflag {
        &self.chintflag
    }
    #[doc = "0x07 - Channel n Status"]
    #[inline(always)]
    pub const fn chstatus(&self) -> &Chstatus {
        &self.chstatus
    }
}
#[doc = "CHCTRL (rw) register accessor: Channel n Control\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctrl`] module"]
#[doc(alias = "CHCTRL")]
pub type Chctrl = crate::Reg<chctrl::ChctrlSpec>;
#[doc = "Channel n Control"]
pub mod chctrl;
#[doc = "CHINTENCLR (rw) register accessor: Channel n Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`chintenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chintenclr`] module"]
#[doc(alias = "CHINTENCLR")]
pub type Chintenclr = crate::Reg<chintenclr::ChintenclrSpec>;
#[doc = "Channel n Interrupt Enable Clear"]
pub mod chintenclr;
#[doc = "CHINTENSET (rw) register accessor: Channel n Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`chintenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chintenset`] module"]
#[doc(alias = "CHINTENSET")]
pub type Chintenset = crate::Reg<chintenset::ChintensetSpec>;
#[doc = "Channel n Interrupt Enable Set"]
pub mod chintenset;
#[doc = "CHINTFLAG (rw) register accessor: Channel n Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`chintflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chintflag`] module"]
#[doc(alias = "CHINTFLAG")]
pub type Chintflag = crate::Reg<chintflag::ChintflagSpec>;
#[doc = "Channel n Interrupt Flag Status and Clear"]
pub mod chintflag;
#[doc = "CHSTATUS (r) register accessor: Channel n Status\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chstatus`] module"]
#[doc(alias = "CHSTATUS")]
pub type Chstatus = crate::Reg<chstatus::ChstatusSpec>;
#[doc = "Channel n Status"]
pub mod chstatus;
