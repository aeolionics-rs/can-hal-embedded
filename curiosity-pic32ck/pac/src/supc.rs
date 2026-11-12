#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    status: Status,
    syncbusy: Syncbusy,
    bor: Bor,
    lvd: Lvd,
    vregctrl: Vregctrl,
    vrefctrl: Vrefctrl,
    _reserved9: [u8; 0x04],
    bkout: Bkout,
    bkin: Bkin,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x04 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x08 - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x0c - Flag status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - Synchronisation Busy"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x14 - BOR Control"]
    #[inline(always)]
    pub const fn bor(&self) -> &Bor {
        &self.bor
    }
    #[doc = "0x18 - LVD Control"]
    #[inline(always)]
    pub const fn lvd(&self) -> &Lvd {
        &self.lvd
    }
    #[doc = "0x1c - VREG Control"]
    #[inline(always)]
    pub const fn vregctrl(&self) -> &Vregctrl {
        &self.vregctrl
    }
    #[doc = "0x20 - VREF Control"]
    #[inline(always)]
    pub const fn vrefctrl(&self) -> &Vrefctrl {
        &self.vrefctrl
    }
    #[doc = "0x28 - Backup Output Control"]
    #[inline(always)]
    pub const fn bkout(&self) -> &Bkout {
        &self.bkout
    }
    #[doc = "0x2c - Backup Input Control"]
    #[inline(always)]
    pub const fn bkin(&self) -> &Bkin {
        &self.bkin
    }
}
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (r) register accessor: Flag status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Flag status"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: Synchronisation Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronisation Busy"]
pub mod syncbusy;
#[doc = "BOR (rw) register accessor: BOR Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bor`] module"]
#[doc(alias = "BOR")]
pub type Bor = crate::Reg<bor::BorSpec>;
#[doc = "BOR Control"]
pub mod bor;
#[doc = "LVD (rw) register accessor: LVD Control\n\nYou can [`read`](crate::Reg::read) this register and get [`lvd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvd`] module"]
#[doc(alias = "LVD")]
pub type Lvd = crate::Reg<lvd::LvdSpec>;
#[doc = "LVD Control"]
pub mod lvd;
#[doc = "VREGCTRL (rw) register accessor: VREG Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vregctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vregctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vregctrl`] module"]
#[doc(alias = "VREGCTRL")]
pub type Vregctrl = crate::Reg<vregctrl::VregctrlSpec>;
#[doc = "VREG Control"]
pub mod vregctrl;
#[doc = "VREFCTRL (rw) register accessor: VREF Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vrefctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrefctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrefctrl`] module"]
#[doc(alias = "VREFCTRL")]
pub type Vrefctrl = crate::Reg<vrefctrl::VrefctrlSpec>;
#[doc = "VREF Control"]
pub mod vrefctrl;
#[doc = "BKOUT (rw) register accessor: Backup Output Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bkout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkout`] module"]
#[doc(alias = "BKOUT")]
pub type Bkout = crate::Reg<bkout::BkoutSpec>;
#[doc = "Backup Output Control"]
pub mod bkout;
#[doc = "BKIN (r) register accessor: Backup Input Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bkin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkin`] module"]
#[doc(alias = "BKIN")]
pub type Bkin = crate::Reg<bkin::BkinSpec>;
#[doc = "Backup Input Control"]
pub mod bkin;
