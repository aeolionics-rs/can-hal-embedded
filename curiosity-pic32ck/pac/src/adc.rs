#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    ctrlc: Ctrlc,
    _reserved3: [u8; 0x04],
    ctrld: Ctrld,
    _reserved4: [u8; 0x0c],
    config: [Config; 1],
    _reserved5: [u8; 0x70],
    cmpctrl: Cmpctrl,
    _reserved6: [u8; 0x0c],
    fltctrl: Fltctrl,
    _reserved7: [u8; 0x0c],
    corchdataid: Corchdataid,
    chrdydat: Chrdydat,
    pffdata: Pffdata,
    dmabase: Dmabase,
    dmactrl: Dmactrl,
    pffctrl: Pffctrl,
    syncbusy: Syncbusy,
    _reserved14: [u8; 0x04],
    dmaintenclr: Dmaintenclr,
    dmaintset: Dmaintset,
    dmaintflag: Dmaintflag,
    ctlintenset: Ctlintenset,
    ctlintenclr: Ctlintenclr,
    ctlintflag: Ctlintflag,
    _reserved20: [u8; 0x18],
    int: [Int; 1],
    _reserved21: [u8; 0x3c],
    dbgctrl: Dbgctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - CONTROL A REGISTER"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - CONTROL B REGISTER"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x08 - Control C Register"]
    #[inline(always)]
    pub const fn ctrlc(&self) -> &Ctrlc {
        &self.ctrlc
    }
    #[doc = "0x10 - Control D Register"]
    #[inline(always)]
    pub const fn ctrld(&self) -> &Ctrld {
        &self.ctrld
    }
    #[doc = "0x20..0x40 - CONFIG\\[%s\\]"]
    #[inline(always)]
    pub const fn config(&self, n: usize) -> &Config {
        &self.config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x40 - CONFIG\\[%s\\]"]
    #[inline(always)]
    pub fn config_iter(&self) -> impl Iterator<Item = &Config> {
        self.config.iter()
    }
    #[doc = "0xb0 - Comparator Control"]
    #[inline(always)]
    pub const fn cmpctrl(&self) -> &Cmpctrl {
        &self.cmpctrl
    }
    #[doc = "0xc0 - Filter Control"]
    #[inline(always)]
    pub const fn fltctrl(&self) -> &Fltctrl {
        &self.fltctrl
    }
    #[doc = "0xd0 - Channel Ready DATA ID"]
    #[inline(always)]
    pub const fn corchdataid(&self) -> &Corchdataid {
        &self.corchdataid
    }
    #[doc = "0xd4 - Channel Ready Data Register"]
    #[inline(always)]
    pub const fn chrdydat(&self) -> &Chrdydat {
        &self.chrdydat
    }
    #[doc = "0xd8 - APB FIFO Output Data"]
    #[inline(always)]
    pub const fn pffdata(&self) -> &Pffdata {
        &self.pffdata
    }
    #[doc = "0xdc - DMA Sample Base Address"]
    #[inline(always)]
    pub const fn dmabase(&self) -> &Dmabase {
        &self.dmabase
    }
    #[doc = "0xe0 - DMA Control Register"]
    #[inline(always)]
    pub const fn dmactrl(&self) -> &Dmactrl {
        &self.dmactrl
    }
    #[doc = "0xe4 - APB FIFO Control Register"]
    #[inline(always)]
    pub const fn pffctrl(&self) -> &Pffctrl {
        &self.pffctrl
    }
    #[doc = "0xe8 - CORE SYNC Busy Status Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0xf0 - DMA Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn dmaintenclr(&self) -> &Dmaintenclr {
        &self.dmaintenclr
    }
    #[doc = "0xf4 - DMA Interrupt Enable Set"]
    #[inline(always)]
    pub const fn dmaintset(&self) -> &Dmaintset {
        &self.dmaintset
    }
    #[doc = "0xf8 - DMA Interrupt Flag and Status"]
    #[inline(always)]
    pub const fn dmaintflag(&self) -> &Dmaintflag {
        &self.dmaintflag
    }
    #[doc = "0xfc - CORE Controller Interrupt Enable Set"]
    #[inline(always)]
    pub const fn ctlintenset(&self) -> &Ctlintenset {
        &self.ctlintenset
    }
    #[doc = "0x100 - CORE Controller Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn ctlintenclr(&self) -> &Ctlintenclr {
        &self.ctlintenclr
    }
    #[doc = "0x104 - CORE Controller Interrupt Flags"]
    #[inline(always)]
    pub const fn ctlintflag(&self) -> &Ctlintflag {
        &self.ctlintflag
    }
    #[doc = "0x120..0x12c - INT\\[%s\\]"]
    #[inline(always)]
    pub const fn int(&self, n: usize) -> &Int {
        &self.int[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x12c - INT\\[%s\\]"]
    #[inline(always)]
    pub fn int_iter(&self) -> impl Iterator<Item = &Int> {
        self.int.iter()
    }
    #[doc = "0x168 - Debug Control Register"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
}
#[doc = "CTRLA (rw) register accessor: CONTROL A REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "CONTROL A REGISTER"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: CONTROL B REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "CONTROL B REGISTER"]
pub mod ctrlb;
#[doc = "CTRLC (rw) register accessor: Control C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlc`] module"]
#[doc(alias = "CTRLC")]
pub type Ctrlc = crate::Reg<ctrlc::CtrlcSpec>;
#[doc = "Control C Register"]
pub mod ctrlc;
#[doc = "CTRLD (rw) register accessor: Control D Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrld`] module"]
#[doc(alias = "CTRLD")]
pub type Ctrld = crate::Reg<ctrld::CtrldSpec>;
#[doc = "Control D Register"]
pub mod ctrld;
#[doc = "CONFIG\\[%s\\]"]
pub use self::config::Config;
#[doc = r"Cluster"]
#[doc = "CONFIG\\[%s\\]"]
pub mod config;
#[doc = "CMPCTRL (rw) register accessor: Comparator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpctrl`] module"]
#[doc(alias = "CMPCTRL")]
pub type Cmpctrl = crate::Reg<cmpctrl::CmpctrlSpec>;
#[doc = "Comparator Control"]
pub mod cmpctrl;
#[doc = "FLTCTRL (rw) register accessor: Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fltctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltctrl`] module"]
#[doc(alias = "FLTCTRL")]
pub type Fltctrl = crate::Reg<fltctrl::FltctrlSpec>;
#[doc = "Filter Control"]
pub mod fltctrl;
#[doc = "CORCHDATAID (rw) register accessor: Channel Ready DATA ID\n\nYou can [`read`](crate::Reg::read) this register and get [`corchdataid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`corchdataid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@corchdataid`] module"]
#[doc(alias = "CORCHDATAID")]
pub type Corchdataid = crate::Reg<corchdataid::CorchdataidSpec>;
#[doc = "Channel Ready DATA ID"]
pub mod corchdataid;
#[doc = "CHRDYDAT (rw) register accessor: Channel Ready Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chrdydat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chrdydat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chrdydat`] module"]
#[doc(alias = "CHRDYDAT")]
pub type Chrdydat = crate::Reg<chrdydat::ChrdydatSpec>;
#[doc = "Channel Ready Data Register"]
pub mod chrdydat;
#[doc = "PFFDATA (r) register accessor: APB FIFO Output Data\n\nYou can [`read`](crate::Reg::read) this register and get [`pffdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pffdata`] module"]
#[doc(alias = "PFFDATA")]
pub type Pffdata = crate::Reg<pffdata::PffdataSpec>;
#[doc = "APB FIFO Output Data"]
pub mod pffdata;
#[doc = "DMABASE (rw) register accessor: DMA Sample Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmabase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmabase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmabase`] module"]
#[doc(alias = "DMABASE")]
pub type Dmabase = crate::Reg<dmabase::DmabaseSpec>;
#[doc = "DMA Sample Base Address"]
pub mod dmabase;
#[doc = "DMACTRL (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactrl`] module"]
#[doc(alias = "DMACTRL")]
pub type Dmactrl = crate::Reg<dmactrl::DmactrlSpec>;
#[doc = "DMA Control Register"]
pub mod dmactrl;
#[doc = "PFFCTRL (rw) register accessor: APB FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pffctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pffctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pffctrl`] module"]
#[doc(alias = "PFFCTRL")]
pub type Pffctrl = crate::Reg<pffctrl::PffctrlSpec>;
#[doc = "APB FIFO Control Register"]
pub mod pffctrl;
#[doc = "SYNCBUSY (r) register accessor: CORE SYNC Busy Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "CORE SYNC Busy Status Register"]
pub mod syncbusy;
#[doc = "DMAINTENCLR (rw) register accessor: DMA Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaintenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaintenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaintenclr`] module"]
#[doc(alias = "DMAINTENCLR")]
pub type Dmaintenclr = crate::Reg<dmaintenclr::DmaintenclrSpec>;
#[doc = "DMA Interrupt Enable Clear"]
pub mod dmaintenclr;
#[doc = "DMAINTSET (rw) register accessor: DMA Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaintset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaintset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaintset`] module"]
#[doc(alias = "DMAINTSET")]
pub type Dmaintset = crate::Reg<dmaintset::DmaintsetSpec>;
#[doc = "DMA Interrupt Enable Set"]
pub mod dmaintset;
#[doc = "DMAINTFLAG (rw) register accessor: DMA Interrupt Flag and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaintflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaintflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaintflag`] module"]
#[doc(alias = "DMAINTFLAG")]
pub type Dmaintflag = crate::Reg<dmaintflag::DmaintflagSpec>;
#[doc = "DMA Interrupt Flag and Status"]
pub mod dmaintflag;
#[doc = "CTLINTENSET (rw) register accessor: CORE Controller Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`ctlintenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctlintenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlintenset`] module"]
#[doc(alias = "CTLINTENSET")]
pub type Ctlintenset = crate::Reg<ctlintenset::CtlintensetSpec>;
#[doc = "CORE Controller Interrupt Enable Set"]
pub mod ctlintenset;
#[doc = "CTLINTENCLR (rw) register accessor: CORE Controller Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`ctlintenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctlintenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlintenclr`] module"]
#[doc(alias = "CTLINTENCLR")]
pub type Ctlintenclr = crate::Reg<ctlintenclr::CtlintenclrSpec>;
#[doc = "CORE Controller Interrupt Enable Clear"]
pub mod ctlintenclr;
#[doc = "CTLINTFLAG (rw) register accessor: CORE Controller Interrupt Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`ctlintflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctlintflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlintflag`] module"]
#[doc(alias = "CTLINTFLAG")]
pub type Ctlintflag = crate::Reg<ctlintflag::CtlintflagSpec>;
#[doc = "CORE Controller Interrupt Flags"]
pub mod ctlintflag;
#[doc = "INT\\[%s\\]"]
pub use self::int::Int;
#[doc = r"Cluster"]
#[doc = "INT\\[%s\\]"]
pub mod int;
#[doc = "DBGCTRL (rw) register accessor: Debug Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`] module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug Control Register"]
pub mod dbgctrl;
