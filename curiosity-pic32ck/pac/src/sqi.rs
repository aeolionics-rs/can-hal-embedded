#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    _reserved1: [u8; 0x0c],
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    _reserved4: [u8; 0x04],
    syncbusy: Syncbusy,
    _reserved5: [u8; 0xdc],
    xcon1: Xcon1,
    xcon2: Xcon2,
    cfg: Cfg,
    con: Con,
    clkcon: Clkcon,
    cmdthr: Cmdthr,
    intthr: Intthr,
    inten: Inten,
    intstat: Intstat,
    txdata: Txdata,
    rxdata: Rxdata,
    stat1: Stat1,
    stat2: Stat2,
    bdcon: Bdcon,
    bdcuradd: Bdcuradd,
    _reserved20: [u8; 0x04],
    bdbaseadd: Bdbaseadd,
    bdstat: Bdstat,
    bdpollcon: Bdpollcon,
    bdtxdstat: Bdtxdstat,
    bdrxdstat: Bdrxdstat,
    thr: Thr,
    intsigen: Intsigen,
    tapcon: Tapcon,
    memstat: Memstat,
    xcon3: Xcon3,
    xcon4: Xcon4,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A register"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x10 - Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x14 - Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x18 - Interrupt Status and Clear Register"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x20 - Syncbusy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x100 - SPI XIP Control1 register"]
    #[inline(always)]
    pub const fn xcon1(&self) -> &Xcon1 {
        &self.xcon1
    }
    #[doc = "0x104 - SPI XIP Control2 register"]
    #[inline(always)]
    pub const fn xcon2(&self) -> &Xcon2 {
        &self.xcon2
    }
    #[doc = "0x108 - SPI Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x10c - SPI Control register"]
    #[inline(always)]
    pub const fn con(&self) -> &Con {
        &self.con
    }
    #[doc = "0x110 - SPI Clock Control register"]
    #[inline(always)]
    pub const fn clkcon(&self) -> &Clkcon {
        &self.clkcon
    }
    #[doc = "0x114 - SPI command threshold register"]
    #[inline(always)]
    pub const fn cmdthr(&self) -> &Cmdthr {
        &self.cmdthr
    }
    #[doc = "0x118 - SPI Interrupt Threshold register"]
    #[inline(always)]
    pub const fn intthr(&self) -> &Intthr {
        &self.intthr
    }
    #[doc = "0x11c - SPI Interrupt Enable Register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x120 - SPI Interrupt Status Register"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x124 - SPI Transmit Data Register"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x128 - SPI Receive Data Register"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x12c - SPI Status1 Register"]
    #[inline(always)]
    pub const fn stat1(&self) -> &Stat1 {
        &self.stat1
    }
    #[doc = "0x130 - SPI status 2 register"]
    #[inline(always)]
    pub const fn stat2(&self) -> &Stat2 {
        &self.stat2
    }
    #[doc = "0x134 - BD_CTRL Register"]
    #[inline(always)]
    pub const fn bdcon(&self) -> &Bdcon {
        &self.bdcon
    }
    #[doc = "0x138 - BD_CURR_ADDR Register"]
    #[inline(always)]
    pub const fn bdcuradd(&self) -> &Bdcuradd {
        &self.bdcuradd
    }
    #[doc = "0x140 - BD_BASE_ADDR register"]
    #[inline(always)]
    pub const fn bdbaseadd(&self) -> &Bdbaseadd {
        &self.bdbaseadd
    }
    #[doc = "0x144 - BD_STATUS Register"]
    #[inline(always)]
    pub const fn bdstat(&self) -> &Bdstat {
        &self.bdstat
    }
    #[doc = "0x148 - BD_POLL_CTRL Register"]
    #[inline(always)]
    pub const fn bdpollcon(&self) -> &Bdpollcon {
        &self.bdpollcon
    }
    #[doc = "0x14c - BD_TX_DMA_STATUS Register"]
    #[inline(always)]
    pub const fn bdtxdstat(&self) -> &Bdtxdstat {
        &self.bdtxdstat
    }
    #[doc = "0x150 - BD_RX_DMA_STATUS Register"]
    #[inline(always)]
    pub const fn bdrxdstat(&self) -> &Bdrxdstat {
        &self.bdrxdstat
    }
    #[doc = "0x154 - SPI Control threshold register"]
    #[inline(always)]
    pub const fn thr(&self) -> &Thr {
        &self.thr
    }
    #[doc = "0x158 - SPI Interrupt Signal Enable Register"]
    #[inline(always)]
    pub const fn intsigen(&self) -> &Intsigen {
        &self.intsigen
    }
    #[doc = "0x15c - Tap Control Register"]
    #[inline(always)]
    pub const fn tapcon(&self) -> &Tapcon {
        &self.tapcon
    }
    #[doc = "0x160 - SPI Status Control Register"]
    #[inline(always)]
    pub const fn memstat(&self) -> &Memstat {
        &self.memstat
    }
    #[doc = "0x164 - SPI XIP Control 3 Register"]
    #[inline(always)]
    pub const fn xcon3(&self) -> &Xcon3 {
        &self.xcon3
    }
    #[doc = "0x168 - SPI XIP Control4 Register"]
    #[inline(always)]
    pub const fn xcon4(&self) -> &Xcon4 {
        &self.xcon4
    }
}
#[doc = "CTRLA (rw) register accessor: Control A register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A register"]
pub mod ctrla;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear Register"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set Register"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Status and Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Status and Clear Register"]
pub mod intflag;
#[doc = "SYNCBUSY (r) register accessor: Syncbusy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Syncbusy Register"]
pub mod syncbusy;
#[doc = "XCON1 (rw) register accessor: SPI XIP Control1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`xcon1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcon1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcon1`] module"]
#[doc(alias = "XCON1")]
pub type Xcon1 = crate::Reg<xcon1::Xcon1Spec>;
#[doc = "SPI XIP Control1 register"]
pub mod xcon1;
#[doc = "XCON2 (rw) register accessor: SPI XIP Control2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`xcon2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcon2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcon2`] module"]
#[doc(alias = "XCON2")]
pub type Xcon2 = crate::Reg<xcon2::Xcon2Spec>;
#[doc = "SPI XIP Control2 register"]
pub mod xcon2;
#[doc = "CFG (rw) register accessor: SPI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "SPI Configuration Register"]
pub mod cfg;
#[doc = "CON (rw) register accessor: SPI Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`con::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`con::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@con`] module"]
#[doc(alias = "CON")]
pub type Con = crate::Reg<con::ConSpec>;
#[doc = "SPI Control register"]
#[path = "sqi/con_.rs"]
pub mod con;
#[doc = "CLKCON (rw) register accessor: SPI Clock Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcon`] module"]
#[doc(alias = "CLKCON")]
pub type Clkcon = crate::Reg<clkcon::ClkconSpec>;
#[doc = "SPI Clock Control register"]
pub mod clkcon;
#[doc = "CMDTHR (rw) register accessor: SPI command threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdthr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdthr`] module"]
#[doc(alias = "CMDTHR")]
pub type Cmdthr = crate::Reg<cmdthr::CmdthrSpec>;
#[doc = "SPI command threshold register"]
pub mod cmdthr;
#[doc = "INTTHR (rw) register accessor: SPI Interrupt Threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`intthr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intthr`] module"]
#[doc(alias = "INTTHR")]
pub type Intthr = crate::Reg<intthr::IntthrSpec>;
#[doc = "SPI Interrupt Threshold register"]
pub mod intthr;
#[doc = "INTEN (rw) register accessor: SPI Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "SPI Interrupt Enable Register"]
pub mod inten;
#[doc = "INTSTAT (rw) register accessor: SPI Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`] module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "SPI Interrupt Status Register"]
pub mod intstat;
#[doc = "TXDATA (rw) register accessor: SPI Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`] module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "SPI Transmit Data Register"]
pub mod txdata;
#[doc = "RXDATA (r) register accessor: SPI Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`] module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "SPI Receive Data Register"]
pub mod rxdata;
#[doc = "STAT1 (r) register accessor: SPI Status1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat1`] module"]
#[doc(alias = "STAT1")]
pub type Stat1 = crate::Reg<stat1::Stat1Spec>;
#[doc = "SPI Status1 Register"]
pub mod stat1;
#[doc = "STAT2 (r) register accessor: SPI status 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat2`] module"]
#[doc(alias = "STAT2")]
pub type Stat2 = crate::Reg<stat2::Stat2Spec>;
#[doc = "SPI status 2 register"]
pub mod stat2;
#[doc = "BDCON (rw) register accessor: BD_CTRL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcon`] module"]
#[doc(alias = "BDCON")]
pub type Bdcon = crate::Reg<bdcon::BdconSpec>;
#[doc = "BD_CTRL Register"]
pub mod bdcon;
#[doc = "BDCURADD (r) register accessor: BD_CURR_ADDR Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcuradd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcuradd`] module"]
#[doc(alias = "BDCURADD")]
pub type Bdcuradd = crate::Reg<bdcuradd::BdcuraddSpec>;
#[doc = "BD_CURR_ADDR Register"]
pub mod bdcuradd;
#[doc = "BDBASEADD (rw) register accessor: BD_BASE_ADDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdbaseadd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdbaseadd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdbaseadd`] module"]
#[doc(alias = "BDBASEADD")]
pub type Bdbaseadd = crate::Reg<bdbaseadd::BdbaseaddSpec>;
#[doc = "BD_BASE_ADDR register"]
pub mod bdbaseadd;
#[doc = "BDSTAT (r) register accessor: BD_STATUS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdstat`] module"]
#[doc(alias = "BDSTAT")]
pub type Bdstat = crate::Reg<bdstat::BdstatSpec>;
#[doc = "BD_STATUS Register"]
pub mod bdstat;
#[doc = "BDPOLLCON (rw) register accessor: BD_POLL_CTRL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdpollcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdpollcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdpollcon`] module"]
#[doc(alias = "BDPOLLCON")]
pub type Bdpollcon = crate::Reg<bdpollcon::BdpollconSpec>;
#[doc = "BD_POLL_CTRL Register"]
pub mod bdpollcon;
#[doc = "BDTXDSTAT (rw) register accessor: BD_TX_DMA_STATUS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdtxdstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtxdstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtxdstat`] module"]
#[doc(alias = "BDTXDSTAT")]
pub type Bdtxdstat = crate::Reg<bdtxdstat::BdtxdstatSpec>;
#[doc = "BD_TX_DMA_STATUS Register"]
pub mod bdtxdstat;
#[doc = "BDRXDSTAT (r) register accessor: BD_RX_DMA_STATUS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdrxdstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdrxdstat`] module"]
#[doc(alias = "BDRXDSTAT")]
pub type Bdrxdstat = crate::Reg<bdrxdstat::BdrxdstatSpec>;
#[doc = "BD_RX_DMA_STATUS Register"]
pub mod bdrxdstat;
#[doc = "THR (rw) register accessor: SPI Control threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`] module"]
#[doc(alias = "THR")]
pub type Thr = crate::Reg<thr::ThrSpec>;
#[doc = "SPI Control threshold register"]
pub mod thr;
#[doc = "INTSIGEN (rw) register accessor: SPI Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intsigen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsigen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsigen`] module"]
#[doc(alias = "INTSIGEN")]
pub type Intsigen = crate::Reg<intsigen::IntsigenSpec>;
#[doc = "SPI Interrupt Signal Enable Register"]
pub mod intsigen;
#[doc = "TAPCON (rw) register accessor: Tap Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tapcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tapcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tapcon`] module"]
#[doc(alias = "TAPCON")]
pub type Tapcon = crate::Reg<tapcon::TapconSpec>;
#[doc = "Tap Control Register"]
pub mod tapcon;
#[doc = "MEMSTAT (rw) register accessor: SPI Status Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`memstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memstat`] module"]
#[doc(alias = "MEMSTAT")]
pub type Memstat = crate::Reg<memstat::MemstatSpec>;
#[doc = "SPI Status Control Register"]
pub mod memstat;
#[doc = "XCON3 (rw) register accessor: SPI XIP Control 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xcon3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcon3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcon3`] module"]
#[doc(alias = "XCON3")]
pub type Xcon3 = crate::Reg<xcon3::Xcon3Spec>;
#[doc = "SPI XIP Control 3 Register"]
pub mod xcon3;
#[doc = "XCON4 (rw) register accessor: SPI XIP Control4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xcon4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcon4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcon4`] module"]
#[doc(alias = "XCON4")]
pub type Xcon4 = crate::Reg<xcon4::Xcon4Spec>;
#[doc = "SPI XIP Control4 Register"]
pub mod xcon4;
