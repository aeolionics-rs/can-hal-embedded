#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    status: Status,
    _reserved6: [u8; 0xe8],
    mbfifo: [Mbfifo; 16],
    mbtxstatus: Mbtxstatus,
    mbrxstatus: Mbrxstatus,
    mbtxprot: Mbtxprot,
    mbrxprot: Mbrxprot,
    mbtxhead: Mbtxhead,
    mbrxhead: Mbrxhead,
    mbconfig: Mbconfig,
}
impl RegisterBlock {
    #[doc = "0x00 - CONTROL REGISTER A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - CONTROL REGISTER B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x08 - INTERRUPT ENABLE CLEAR REGISTER"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x0c - INTERRUPT ENABLE SET"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x10 - INTERRUPT FLAG"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x14 - STATUS"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x100..0x140 - MAILBOX FIFO"]
    #[inline(always)]
    pub const fn mbfifo(&self, n: usize) -> &Mbfifo {
        &self.mbfifo[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x140 - MAILBOX FIFO"]
    #[inline(always)]
    pub fn mbfifo_iter(&self) -> impl Iterator<Item = &Mbfifo> {
        self.mbfifo.iter()
    }
    #[doc = "0x140 - MAILBOX TRANSMIT STATUS"]
    #[inline(always)]
    pub const fn mbtxstatus(&self) -> &Mbtxstatus {
        &self.mbtxstatus
    }
    #[doc = "0x144 - MAILBOX RECEIVE STATUS"]
    #[inline(always)]
    pub const fn mbrxstatus(&self) -> &Mbrxstatus {
        &self.mbrxstatus
    }
    #[doc = "0x148 - MAILBOX TX PROTECTION"]
    #[inline(always)]
    pub const fn mbtxprot(&self) -> &Mbtxprot {
        &self.mbtxprot
    }
    #[doc = "0x14c - MAILBOX RX PROTECTION"]
    #[inline(always)]
    pub const fn mbrxprot(&self) -> &Mbrxprot {
        &self.mbrxprot
    }
    #[doc = "0x150 - MAILBOX TRANSMIT HEADER"]
    #[inline(always)]
    pub const fn mbtxhead(&self) -> &Mbtxhead {
        &self.mbtxhead
    }
    #[doc = "0x154 - MAILBOX RECEIVE HEADER"]
    #[inline(always)]
    pub const fn mbrxhead(&self) -> &Mbrxhead {
        &self.mbrxhead
    }
    #[doc = "0x158 - MAILBOX CONFIGURATION"]
    #[inline(always)]
    pub const fn mbconfig(&self) -> &Mbconfig {
        &self.mbconfig
    }
}
#[doc = "CTRLA (rw) register accessor: CONTROL REGISTER A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "CONTROL REGISTER A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: CONTROL REGISTER B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "CONTROL REGISTER B"]
pub mod ctrlb;
#[doc = "INTENCLR (rw) register accessor: INTERRUPT ENABLE CLEAR REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "INTERRUPT ENABLE CLEAR REGISTER"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: INTERRUPT ENABLE SET\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "INTERRUPT ENABLE SET"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: INTERRUPT FLAG\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "INTERRUPT FLAG"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "STATUS"]
pub mod status;
#[doc = "MBFIFO (rw) register accessor: MAILBOX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`mbfifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbfifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>\n\nFor information about available fields see [`mod@mbfifo`] module"]
#[doc(alias = "MBFIFO")]
pub type Mbfifo = crate::Reg<mbfifo::MbfifoSpec>;
#[doc = "MAILBOX FIFO"]
pub mod mbfifo;
#[doc = "MBTXSTATUS (rw) register accessor: MAILBOX TRANSMIT STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mbtxstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbtxstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbtxstatus`] module"]
#[doc(alias = "MBTXSTATUS")]
pub type Mbtxstatus = crate::Reg<mbtxstatus::MbtxstatusSpec>;
#[doc = "MAILBOX TRANSMIT STATUS"]
pub mod mbtxstatus;
#[doc = "MBRXSTATUS (rw) register accessor: MAILBOX RECEIVE STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mbrxstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbrxstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbrxstatus`] module"]
#[doc(alias = "MBRXSTATUS")]
pub type Mbrxstatus = crate::Reg<mbrxstatus::MbrxstatusSpec>;
#[doc = "MAILBOX RECEIVE STATUS"]
pub mod mbrxstatus;
#[doc = "MBTXPROT (rw) register accessor: MAILBOX TX PROTECTION\n\nYou can [`read`](crate::Reg::read) this register and get [`mbtxprot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbtxprot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbtxprot`] module"]
#[doc(alias = "MBTXPROT")]
pub type Mbtxprot = crate::Reg<mbtxprot::MbtxprotSpec>;
#[doc = "MAILBOX TX PROTECTION"]
pub mod mbtxprot;
#[doc = "MBRXPROT (rw) register accessor: MAILBOX RX PROTECTION\n\nYou can [`read`](crate::Reg::read) this register and get [`mbrxprot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbrxprot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbrxprot`] module"]
#[doc(alias = "MBRXPROT")]
pub type Mbrxprot = crate::Reg<mbrxprot::MbrxprotSpec>;
#[doc = "MAILBOX RX PROTECTION"]
pub mod mbrxprot;
#[doc = "MBTXHEAD (rw) register accessor: MAILBOX TRANSMIT HEADER\n\nYou can [`read`](crate::Reg::read) this register and get [`mbtxhead::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbtxhead::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>\n\nFor information about available fields see [`mod@mbtxhead`] module"]
#[doc(alias = "MBTXHEAD")]
pub type Mbtxhead = crate::Reg<mbtxhead::MbtxheadSpec>;
#[doc = "MAILBOX TRANSMIT HEADER"]
pub mod mbtxhead;
#[doc = "MBRXHEAD (rw) register accessor: MAILBOX RECEIVE HEADER\n\nYou can [`read`](crate::Reg::read) this register and get [`mbrxhead::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbrxhead::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>\n\nFor information about available fields see [`mod@mbrxhead`] module"]
#[doc(alias = "MBRXHEAD")]
pub type Mbrxhead = crate::Reg<mbrxhead::MbrxheadSpec>;
#[doc = "MAILBOX RECEIVE HEADER"]
pub mod mbrxhead;
#[doc = "MBCONFIG (rw) register accessor: MAILBOX CONFIGURATION\n\nYou can [`read`](crate::Reg::read) this register and get [`mbconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbconfig`] module"]
#[doc(alias = "MBCONFIG")]
pub type Mbconfig = crate::Reg<mbconfig::MbconfigSpec>;
#[doc = "MAILBOX CONFIGURATION"]
pub mod mbconfig;
