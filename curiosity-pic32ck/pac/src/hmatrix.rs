#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcfg: [Mcfg; 16],
    scfg: [Scfg; 13],
    _reserved2: [u8; 0x0c],
    prs: [Prs; 13],
    _reserved3: [u8; 0x18],
    mrcr: Mrcr,
    _reserved4: [u8; 0x0c],
    sfr: [Sfr; 13],
    _reserved5: [u8; 0x0c],
    meier: Meier,
    meidr: Meidr,
    meimr: Meimr,
    mesr: Mesr,
    mear: [Mear; 16],
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - Master Configuration"]
    #[inline(always)]
    pub const fn mcfg(&self, n: usize) -> &Mcfg {
        &self.mcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - Master Configuration"]
    #[inline(always)]
    pub fn mcfg_iter(&self) -> impl Iterator<Item = &Mcfg> {
        self.mcfg.iter()
    }
    #[doc = "0x40..0x74 - Slave Configuration"]
    #[inline(always)]
    pub const fn scfg(&self, n: usize) -> &Scfg {
        &self.scfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x74 - Slave Configuration"]
    #[inline(always)]
    pub fn scfg_iter(&self) -> impl Iterator<Item = &Scfg> {
        self.scfg.iter()
    }
    #[doc = "0x80..0xe8 - PRS\\[%s\\]"]
    #[inline(always)]
    pub const fn prs(&self, n: usize) -> &Prs {
        &self.prs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xe8 - PRS\\[%s\\]"]
    #[inline(always)]
    pub fn prs_iter(&self) -> impl Iterator<Item = &Prs> {
        self.prs.iter()
    }
    #[doc = "0x100 - Master Remap Control"]
    #[inline(always)]
    pub const fn mrcr(&self) -> &Mrcr {
        &self.mrcr
    }
    #[doc = "0x110..0x144 - Special Function"]
    #[inline(always)]
    pub const fn sfr(&self, n: usize) -> &Sfr {
        &self.sfr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x144 - Special Function"]
    #[inline(always)]
    pub fn sfr_iter(&self) -> impl Iterator<Item = &Sfr> {
        self.sfr.iter()
    }
    #[doc = "0x150 - Master Error Interrupt Enable"]
    #[inline(always)]
    pub const fn meier(&self) -> &Meier {
        &self.meier
    }
    #[doc = "0x154 - Master Error Interrupt Disable"]
    #[inline(always)]
    pub const fn meidr(&self) -> &Meidr {
        &self.meidr
    }
    #[doc = "0x158 - Master Error Interrupt Mask"]
    #[inline(always)]
    pub const fn meimr(&self) -> &Meimr {
        &self.meimr
    }
    #[doc = "0x15c - Master Error Status"]
    #[inline(always)]
    pub const fn mesr(&self) -> &Mesr {
        &self.mesr
    }
    #[doc = "0x160..0x1a0 - Master Error Address"]
    #[inline(always)]
    pub const fn mear(&self, n: usize) -> &Mear {
        &self.mear[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x1a0 - Master Error Address"]
    #[inline(always)]
    pub fn mear_iter(&self) -> impl Iterator<Item = &Mear> {
        self.mear.iter()
    }
}
#[doc = "MCFG (rw) register accessor: Master Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg`] module"]
#[doc(alias = "MCFG")]
pub type Mcfg = crate::Reg<mcfg::McfgSpec>;
#[doc = "Master Configuration"]
pub mod mcfg;
#[doc = "SCFG (rw) register accessor: Slave Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg`] module"]
#[doc(alias = "SCFG")]
pub type Scfg = crate::Reg<scfg::ScfgSpec>;
#[doc = "Slave Configuration"]
pub mod scfg;
#[doc = "PRS\\[%s\\]"]
pub use self::prs::Prs;
#[doc = r"Cluster"]
#[doc = "PRS\\[%s\\]"]
pub mod prs;
#[doc = "MRCR (rw) register accessor: Master Remap Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrcr`] module"]
#[doc(alias = "MRCR")]
pub type Mrcr = crate::Reg<mrcr::MrcrSpec>;
#[doc = "Master Remap Control"]
pub mod mrcr;
#[doc = "SFR (rw) register accessor: Special Function\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr`] module"]
#[doc(alias = "SFR")]
pub type Sfr = crate::Reg<sfr::SfrSpec>;
#[doc = "Special Function"]
pub mod sfr;
#[doc = "MEIER (w) register accessor: Master Error Interrupt Enable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meier`] module"]
#[doc(alias = "MEIER")]
pub type Meier = crate::Reg<meier::MeierSpec>;
#[doc = "Master Error Interrupt Enable"]
pub mod meier;
#[doc = "MEIDR (w) register accessor: Master Error Interrupt Disable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meidr`] module"]
#[doc(alias = "MEIDR")]
pub type Meidr = crate::Reg<meidr::MeidrSpec>;
#[doc = "Master Error Interrupt Disable"]
pub mod meidr;
#[doc = "MEIMR (r) register accessor: Master Error Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`meimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meimr`] module"]
#[doc(alias = "MEIMR")]
pub type Meimr = crate::Reg<meimr::MeimrSpec>;
#[doc = "Master Error Interrupt Mask"]
pub mod meimr;
#[doc = "MESR (r) register accessor: Master Error Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mesr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mesr`] module"]
#[doc(alias = "MESR")]
pub type Mesr = crate::Reg<mesr::MesrSpec>;
#[doc = "Master Error Status"]
pub mod mesr;
#[doc = "MEAR (r) register accessor: Master Error Address\n\nYou can [`read`](crate::Reg::read) this register and get [`mear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mear`] module"]
#[doc(alias = "MEAR")]
pub type Mear = crate::Reg<mear::MearSpec>;
#[doc = "Master Error Address"]
pub mod mear;
