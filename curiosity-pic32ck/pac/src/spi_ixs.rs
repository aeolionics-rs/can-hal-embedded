#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    selctrl: Selctrl,
    spictrl: Spictrl,
    framectrl: Framectrl,
    audctrl: Audctrl,
    tpdctrl: Tpdctrl,
    intenset: Intenset,
    intenclr: Intenclr,
    intflag: Intflag,
    status: Status,
    buf: Buf,
    brg: Brg,
    dbgctrl: Dbgctrl,
    syncbusy: Syncbusy,
    evctrl: Evctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI Control Enable Register"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - SPI Control Options Select Register"]
    #[inline(always)]
    pub const fn selctrl(&self) -> &Selctrl {
        &self.selctrl
    }
    #[doc = "0x08 - SPI Control Register"]
    #[inline(always)]
    pub const fn spictrl(&self) -> &Spictrl {
        &self.spictrl
    }
    #[doc = "0x0c - SPI Control Frame Register"]
    #[inline(always)]
    pub const fn framectrl(&self) -> &Framectrl {
        &self.framectrl
    }
    #[doc = "0x10 - SPI Control Audio Register"]
    #[inline(always)]
    pub const fn audctrl(&self) -> &Audctrl {
        &self.audctrl
    }
    #[doc = "0x14 - SPI Control Tpd Register"]
    #[inline(always)]
    pub const fn tpdctrl(&self) -> &Tpdctrl {
        &self.tpdctrl
    }
    #[doc = "0x18 - SPI Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x1c - SPI Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x20 - SPI Interrupt Flag Register"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x24 - SPI Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x28 - SPI Buffer Register"]
    #[inline(always)]
    pub const fn buf(&self) -> &Buf {
        &self.buf
    }
    #[doc = "0x2c - SPI Baud Rate Register"]
    #[inline(always)]
    pub const fn brg(&self) -> &Brg {
        &self.brg
    }
    #[doc = "0x30 - SPI Debug Control Register"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x34 - SPI Sync Busy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x38 - SPI Event Control Register"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
}
#[doc = "CTRLA (rw) register accessor: SPI Control Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "SPI Control Enable Register"]
pub mod ctrla;
#[doc = "SELCTRL (rw) register accessor: SPI Control Options Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`selctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`selctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@selctrl`] module"]
#[doc(alias = "SELCTRL")]
pub type Selctrl = crate::Reg<selctrl::SelctrlSpec>;
#[doc = "SPI Control Options Select Register"]
pub mod selctrl;
#[doc = "SPICTRL (rw) register accessor: SPI Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spictrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spictrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spictrl`] module"]
#[doc(alias = "SPICTRL")]
pub type Spictrl = crate::Reg<spictrl::SpictrlSpec>;
#[doc = "SPI Control Register"]
pub mod spictrl;
#[doc = "FRAMECTRL (rw) register accessor: SPI Control Frame Register\n\nYou can [`read`](crate::Reg::read) this register and get [`framectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framectrl`] module"]
#[doc(alias = "FRAMECTRL")]
pub type Framectrl = crate::Reg<framectrl::FramectrlSpec>;
#[doc = "SPI Control Frame Register"]
pub mod framectrl;
#[doc = "AUDCTRL (rw) register accessor: SPI Control Audio Register\n\nYou can [`read`](crate::Reg::read) this register and get [`audctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audctrl`] module"]
#[doc(alias = "AUDCTRL")]
pub type Audctrl = crate::Reg<audctrl::AudctrlSpec>;
#[doc = "SPI Control Audio Register"]
pub mod audctrl;
#[doc = "TPDCTRL (rw) register accessor: SPI Control Tpd Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpdctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpdctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpdctrl`] module"]
#[doc(alias = "TPDCTRL")]
pub type Tpdctrl = crate::Reg<tpdctrl::TpdctrlSpec>;
#[doc = "SPI Control Tpd Register"]
pub mod tpdctrl;
#[doc = "INTENSET (rw) register accessor: SPI Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "SPI Interrupt Enable Set Register"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: SPI Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "SPI Interrupt Enable Clear Register"]
pub mod intenclr;
#[doc = "INTFLAG (rw) register accessor: SPI Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "SPI Interrupt Flag Register"]
pub mod intflag;
#[doc = "STATUS (r) register accessor: SPI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "SPI Status Register"]
pub mod status;
#[doc = "BUF (rw) register accessor: SPI Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf`] module"]
#[doc(alias = "BUF")]
pub type Buf = crate::Reg<buf::BufSpec>;
#[doc = "SPI Buffer Register"]
pub mod buf;
#[doc = "BRG (rw) register accessor: SPI Baud Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`brg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brg`] module"]
#[doc(alias = "BRG")]
pub type Brg = crate::Reg<brg::BrgSpec>;
#[doc = "SPI Baud Rate Register"]
pub mod brg;
#[doc = "DBGCTRL (rw) register accessor: SPI Debug Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`] module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "SPI Debug Control Register"]
pub mod dbgctrl;
#[doc = "SYNCBUSY (r) register accessor: SPI Sync Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "SPI Sync Busy Register"]
pub mod syncbusy;
#[doc = "EVCTRL (rw) register accessor: SPI Event Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`] module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "SPI Event Control Register"]
pub mod evctrl;
