#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    mutex: Mutex,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    status: Status,
    key: Key,
    addr: Addr,
    srcaddr: Srcaddr,
    data: [Data; 8],
    swap: Swap,
    pwp: [Pwp; 8],
    lbwp: Lbwp,
    ubwp: Ubwp,
    uowp: Uowp,
    cwp: Cwp,
    _reserved17: [u8; 0x04],
    hsmintenclr: Hsmintenclr,
    hsmintenset: Hsmintenset,
    hsmintflag: Hsmintflag,
    hsmcwp: Hsmcwp,
    hsmldat: [Hsmldat; 8],
    hsmudat: [Hsmudat; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - NVM Write Control Register"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - NVM Control B Register"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x08 - NVM MUTEX Register"]
    #[inline(always)]
    pub const fn mutex(&self) -> &Mutex {
        &self.mutex
    }
    #[doc = "0x0c - NVM Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x10 - NVM Interrupt Enable SET Register"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x14 - NVM Interrupt Flag Register"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x18 - NVM Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x1c - SFR Unlock Register"]
    #[inline(always)]
    pub const fn key(&self) -> &Key {
        &self.key
    }
    #[doc = "0x20 - Flash Address Register"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x24 - Source Data Address Register"]
    #[inline(always)]
    pub const fn srcaddr(&self) -> &Srcaddr {
        &self.srcaddr
    }
    #[doc = "0x28..0x48 - Flash Write Data Register"]
    #[inline(always)]
    pub const fn data(&self, n: usize) -> &Data {
        &self.data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x28..0x48 - Flash Write Data Register"]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = &Data> {
        self.data.iter()
    }
    #[doc = "0x48 - NVM Panel Swap Register"]
    #[inline(always)]
    pub const fn swap(&self) -> &Swap {
        &self.swap
    }
    #[doc = "0x4c..0x6c - PFM Write Protect Region 0"]
    #[inline(always)]
    pub const fn pwp(&self, n: usize) -> &Pwp {
        &self.pwp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4c..0x6c - PFM Write Protect Region 0"]
    #[inline(always)]
    pub fn pwp_iter(&self) -> impl Iterator<Item = &Pwp> {
        self.pwp.iter()
    }
    #[doc = "0x6c - Lower BFM Write Protect Register"]
    #[inline(always)]
    pub const fn lbwp(&self) -> &Lbwp {
        &self.lbwp
    }
    #[doc = "0x70 - Upper BFM Write Protect Register"]
    #[inline(always)]
    pub const fn ubwp(&self) -> &Ubwp {
        &self.ubwp
    }
    #[doc = "0x74 - User OTP Write Protect Register"]
    #[inline(always)]
    pub const fn uowp(&self) -> &Uowp {
        &self.uowp
    }
    #[doc = "0x78 - CFM Page Write Protect Register"]
    #[inline(always)]
    pub const fn cwp(&self) -> &Cwp {
        &self.cwp
    }
    #[doc = "0x80 - HSM NVM Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn hsmintenclr(&self) -> &Hsmintenclr {
        &self.hsmintenclr
    }
    #[doc = "0x84 - HSM NVM Interrupt Enable SET Register"]
    #[inline(always)]
    pub const fn hsmintenset(&self) -> &Hsmintenset {
        &self.hsmintenset
    }
    #[doc = "0x88 - HSM NVM Interrupt Flag Register"]
    #[inline(always)]
    pub const fn hsmintflag(&self) -> &Hsmintflag {
        &self.hsmintflag
    }
    #[doc = "0x8c - HSM CFG Write Protect Register"]
    #[inline(always)]
    pub const fn hsmcwp(&self) -> &Hsmcwp {
        &self.hsmcwp
    }
    #[doc = "0x90..0xb0 - HSM Tamper Lower Overwrite Data Register"]
    #[inline(always)]
    pub const fn hsmldat(&self, n: usize) -> &Hsmldat {
        &self.hsmldat[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x90..0xb0 - HSM Tamper Lower Overwrite Data Register"]
    #[inline(always)]
    pub fn hsmldat_iter(&self) -> impl Iterator<Item = &Hsmldat> {
        self.hsmldat.iter()
    }
    #[doc = "0xb0..0xd0 - HSM Tamper Upper Overwrite Data Register"]
    #[inline(always)]
    pub const fn hsmudat(&self, n: usize) -> &Hsmudat {
        &self.hsmudat[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xb0..0xd0 - HSM Tamper Upper Overwrite Data Register"]
    #[inline(always)]
    pub fn hsmudat_iter(&self) -> impl Iterator<Item = &Hsmudat> {
        self.hsmudat.iter()
    }
}
#[doc = "CTRLA (rw) register accessor: NVM Write Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "NVM Write Control Register"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: NVM Control B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "NVM Control B Register"]
pub mod ctrlb;
#[doc = "MUTEX (rw) register accessor: NVM MUTEX Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mutex`] module"]
#[doc(alias = "MUTEX")]
pub type Mutex = crate::Reg<mutex::MutexSpec>;
#[doc = "NVM MUTEX Register"]
pub mod mutex;
#[doc = "INTENCLR (rw) register accessor: NVM Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "NVM Interrupt Enable Clear Register"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: NVM Interrupt Enable SET Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "NVM Interrupt Enable SET Register"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: NVM Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "NVM Interrupt Flag Register"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: NVM Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "NVM Status Register"]
pub mod status;
#[doc = "KEY (rw) register accessor: SFR Unlock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`] module"]
#[doc(alias = "KEY")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "SFR Unlock Register"]
pub mod key;
#[doc = "ADDR (rw) register accessor: Flash Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Flash Address Register"]
pub mod addr;
#[doc = "SRCADDR (rw) register accessor: Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srcaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr`] module"]
#[doc(alias = "SRCADDR")]
pub type Srcaddr = crate::Reg<srcaddr::SrcaddrSpec>;
#[doc = "Source Data Address Register"]
pub mod srcaddr;
#[doc = "DATA (rw) register accessor: Flash Write Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Flash Write Data Register"]
pub mod data;
#[doc = "SWAP (rw) register accessor: NVM Panel Swap Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swap`] module"]
#[doc(alias = "SWAP")]
pub type Swap = crate::Reg<swap::SwapSpec>;
#[doc = "NVM Panel Swap Register"]
pub mod swap;
#[doc = "PWP (rw) register accessor: PFM Write Protect Region 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pwp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwp`] module"]
#[doc(alias = "PWP")]
pub type Pwp = crate::Reg<pwp::PwpSpec>;
#[doc = "PFM Write Protect Region 0"]
pub mod pwp;
#[doc = "LBWP (rw) register accessor: Lower BFM Write Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lbwp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lbwp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lbwp`] module"]
#[doc(alias = "LBWP")]
pub type Lbwp = crate::Reg<lbwp::LbwpSpec>;
#[doc = "Lower BFM Write Protect Register"]
pub mod lbwp;
#[doc = "UBWP (rw) register accessor: Upper BFM Write Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ubwp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ubwp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ubwp`] module"]
#[doc(alias = "UBWP")]
pub type Ubwp = crate::Reg<ubwp::UbwpSpec>;
#[doc = "Upper BFM Write Protect Register"]
pub mod ubwp;
#[doc = "UOWP (rw) register accessor: User OTP Write Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uowp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uowp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uowp`] module"]
#[doc(alias = "UOWP")]
pub type Uowp = crate::Reg<uowp::UowpSpec>;
#[doc = "User OTP Write Protect Register"]
pub mod uowp;
#[doc = "CWP (rw) register accessor: CFM Page Write Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cwp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwp`] module"]
#[doc(alias = "CWP")]
pub type Cwp = crate::Reg<cwp::CwpSpec>;
#[doc = "CFM Page Write Protect Register"]
pub mod cwp;
#[doc = "HSMINTENCLR (rw) register accessor: HSM NVM Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsmintenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsmintenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsmintenclr`] module"]
#[doc(alias = "HSMINTENCLR")]
pub type Hsmintenclr = crate::Reg<hsmintenclr::HsmintenclrSpec>;
#[doc = "HSM NVM Interrupt Enable Clear Register"]
pub mod hsmintenclr;
#[doc = "HSMINTENSET (rw) register accessor: HSM NVM Interrupt Enable SET Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsmintenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsmintenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsmintenset`] module"]
#[doc(alias = "HSMINTENSET")]
pub type Hsmintenset = crate::Reg<hsmintenset::HsmintensetSpec>;
#[doc = "HSM NVM Interrupt Enable SET Register"]
pub mod hsmintenset;
#[doc = "HSMINTFLAG (rw) register accessor: HSM NVM Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsmintflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsmintflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsmintflag`] module"]
#[doc(alias = "HSMINTFLAG")]
pub type Hsmintflag = crate::Reg<hsmintflag::HsmintflagSpec>;
#[doc = "HSM NVM Interrupt Flag Register"]
pub mod hsmintflag;
#[doc = "HSMCWP (rw) register accessor: HSM CFG Write Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsmcwp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsmcwp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsmcwp`] module"]
#[doc(alias = "HSMCWP")]
pub type Hsmcwp = crate::Reg<hsmcwp::HsmcwpSpec>;
#[doc = "HSM CFG Write Protect Register"]
pub mod hsmcwp;
#[doc = "HSMLDAT (rw) register accessor: HSM Tamper Lower Overwrite Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsmldat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsmldat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsmldat`] module"]
#[doc(alias = "HSMLDAT")]
pub type Hsmldat = crate::Reg<hsmldat::HsmldatSpec>;
#[doc = "HSM Tamper Lower Overwrite Data Register"]
pub mod hsmldat;
#[doc = "HSMUDAT (rw) register accessor: HSM Tamper Upper Overwrite Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsmudat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsmudat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsmudat`] module"]
#[doc(alias = "HSMUDAT")]
pub type Hsmudat = crate::Reg<hsmudat::HsmudatSpec>;
#[doc = "HSM Tamper Upper Overwrite Data Register"]
pub mod hsmudat;
