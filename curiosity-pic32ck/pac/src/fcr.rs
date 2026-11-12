#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    _reserved1: [u8; 0x04],
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    _reserved4: [u8; 0x04],
    dbgctrl: Dbgctrl,
    eccctrl: Eccctrl,
    crcctrl: Crcctrl,
    crcpause: Crcpause,
    crcmadr: Crcmadr,
    crcmlen: Crcmlen,
    crciv: Crciv,
    crcacc: Crcacc,
    crcpoly: Crcpoly,
    crcfxor: Crcfxor,
    crcsum: Crcsum,
    ffltctrl: Ffltctrl,
    ffltptr: Ffltptr,
    ffltadr: Ffltadr,
    ffltcap: Ffltcap,
    ffltpar: Ffltpar,
    ffltsyn: Ffltsyn,
    crp: Crp,
    hsmctrl: Hsmctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A Register"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x08 - Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x0c - Interrupt Enable SET Register"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x10 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x18 - Debug Control Register"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x1c - ECC Control Register"]
    #[inline(always)]
    pub const fn eccctrl(&self) -> &Eccctrl {
        &self.eccctrl
    }
    #[doc = "0x20 - CRC Control Register"]
    #[inline(always)]
    pub const fn crcctrl(&self) -> &Crcctrl {
        &self.crcctrl
    }
    #[doc = "0x24 - CRC PAUSE Register"]
    #[inline(always)]
    pub const fn crcpause(&self) -> &Crcpause {
        &self.crcpause
    }
    #[doc = "0x28 - CRC Message Address Register"]
    #[inline(always)]
    pub const fn crcmadr(&self) -> &Crcmadr {
        &self.crcmadr
    }
    #[doc = "0x2c - CRC Message Length Register"]
    #[inline(always)]
    pub const fn crcmlen(&self) -> &Crcmlen {
        &self.crcmlen
    }
    #[doc = "0x30 - CRC Initial Value Register"]
    #[inline(always)]
    pub const fn crciv(&self) -> &Crciv {
        &self.crciv
    }
    #[doc = "0x34 - CRC Accumulator Register"]
    #[inline(always)]
    pub const fn crcacc(&self) -> &Crcacc {
        &self.crcacc
    }
    #[doc = "0x38 - CRC Polynomial Register"]
    #[inline(always)]
    pub const fn crcpoly(&self) -> &Crcpoly {
        &self.crcpoly
    }
    #[doc = "0x3c - CRC Final XOR Register"]
    #[inline(always)]
    pub const fn crcfxor(&self) -> &Crcfxor {
        &self.crcfxor
    }
    #[doc = "0x40 - CRC Checksum Register"]
    #[inline(always)]
    pub const fn crcsum(&self) -> &Crcsum {
        &self.crcsum
    }
    #[doc = "0x44 - Flash ECC Fault Control Register"]
    #[inline(always)]
    pub const fn ffltctrl(&self) -> &Ffltctrl {
        &self.ffltctrl
    }
    #[doc = "0x48 - Flash ECC Fault Pointer Register"]
    #[inline(always)]
    pub const fn ffltptr(&self) -> &Ffltptr {
        &self.ffltptr
    }
    #[doc = "0x4c - Flash Fault Address Register"]
    #[inline(always)]
    pub const fn ffltadr(&self) -> &Ffltadr {
        &self.ffltadr
    }
    #[doc = "0x50 - Flash Fault Capture Address Register"]
    #[inline(always)]
    pub const fn ffltcap(&self) -> &Ffltcap {
        &self.ffltcap
    }
    #[doc = "0x54 - Flash ECC Fault Parity Register"]
    #[inline(always)]
    pub const fn ffltpar(&self) -> &Ffltpar {
        &self.ffltpar
    }
    #[doc = "0x58 - Flash ECC Fault Syndrome Register"]
    #[inline(always)]
    pub const fn ffltsyn(&self) -> &Ffltsyn {
        &self.ffltsyn
    }
    #[doc = "0x5c - CFM Page Read Protect Register"]
    #[inline(always)]
    pub const fn crp(&self) -> &Crp {
        &self.crp
    }
    #[doc = "0x60 - HSM CFM PAGES Control Register"]
    #[inline(always)]
    pub const fn hsmctrl(&self) -> &Hsmctrl {
        &self.hsmctrl
    }
}
#[doc = "CTRLA (rw) register accessor: Control A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A Register"]
pub mod ctrla;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear Register"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable SET Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable SET Register"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag Register"]
pub mod intflag;
#[doc = "DBGCTRL (rw) register accessor: Debug Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`] module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug Control Register"]
pub mod dbgctrl;
#[doc = "ECCCTRL (rw) register accessor: ECC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccctrl`] module"]
#[doc(alias = "ECCCTRL")]
pub type Eccctrl = crate::Reg<eccctrl::EccctrlSpec>;
#[doc = "ECC Control Register"]
pub mod eccctrl;
#[doc = "CRCCTRL (rw) register accessor: CRC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcctrl`] module"]
#[doc(alias = "CRCCTRL")]
pub type Crcctrl = crate::Reg<crcctrl::CrcctrlSpec>;
#[doc = "CRC Control Register"]
pub mod crcctrl;
#[doc = "CRCPAUSE (rw) register accessor: CRC PAUSE Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpause`] module"]
#[doc(alias = "CRCPAUSE")]
pub type Crcpause = crate::Reg<crcpause::CrcpauseSpec>;
#[doc = "CRC PAUSE Register"]
pub mod crcpause;
#[doc = "CRCMADR (rw) register accessor: CRC Message Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcmadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcmadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcmadr`] module"]
#[doc(alias = "CRCMADR")]
pub type Crcmadr = crate::Reg<crcmadr::CrcmadrSpec>;
#[doc = "CRC Message Address Register"]
pub mod crcmadr;
#[doc = "CRCMLEN (rw) register accessor: CRC Message Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcmlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcmlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcmlen`] module"]
#[doc(alias = "CRCMLEN")]
pub type Crcmlen = crate::Reg<crcmlen::CrcmlenSpec>;
#[doc = "CRC Message Length Register"]
pub mod crcmlen;
#[doc = "CRCIV (rw) register accessor: CRC Initial Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crciv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crciv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crciv`] module"]
#[doc(alias = "CRCIV")]
pub type Crciv = crate::Reg<crciv::CrcivSpec>;
#[doc = "CRC Initial Value Register"]
pub mod crciv;
#[doc = "CRCACC (r) register accessor: CRC Accumulator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcacc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcacc`] module"]
#[doc(alias = "CRCACC")]
pub type Crcacc = crate::Reg<crcacc::CrcaccSpec>;
#[doc = "CRC Accumulator Register"]
pub mod crcacc;
#[doc = "CRCPOLY (rw) register accessor: CRC Polynomial Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpoly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpoly`] module"]
#[doc(alias = "CRCPOLY")]
pub type Crcpoly = crate::Reg<crcpoly::CrcpolySpec>;
#[doc = "CRC Polynomial Register"]
pub mod crcpoly;
#[doc = "CRCFXOR (rw) register accessor: CRC Final XOR Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcfxor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcfxor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcfxor`] module"]
#[doc(alias = "CRCFXOR")]
pub type Crcfxor = crate::Reg<crcfxor::CrcfxorSpec>;
#[doc = "CRC Final XOR Register"]
pub mod crcfxor;
#[doc = "CRCSUM (rw) register accessor: CRC Checksum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcsum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcsum`] module"]
#[doc(alias = "CRCSUM")]
pub type Crcsum = crate::Reg<crcsum::CrcsumSpec>;
#[doc = "CRC Checksum Register"]
pub mod crcsum;
#[doc = "FFLTCTRL (rw) register accessor: Flash ECC Fault Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffltctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffltctrl`] module"]
#[doc(alias = "FFLTCTRL")]
pub type Ffltctrl = crate::Reg<ffltctrl::FfltctrlSpec>;
#[doc = "Flash ECC Fault Control Register"]
pub mod ffltctrl;
#[doc = "FFLTPTR (rw) register accessor: Flash ECC Fault Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffltptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffltptr`] module"]
#[doc(alias = "FFLTPTR")]
pub type Ffltptr = crate::Reg<ffltptr::FfltptrSpec>;
#[doc = "Flash ECC Fault Pointer Register"]
pub mod ffltptr;
#[doc = "FFLTADR (rw) register accessor: Flash Fault Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffltadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffltadr`] module"]
#[doc(alias = "FFLTADR")]
pub type Ffltadr = crate::Reg<ffltadr::FfltadrSpec>;
#[doc = "Flash Fault Address Register"]
pub mod ffltadr;
#[doc = "FFLTCAP (r) register accessor: Flash Fault Capture Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltcap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffltcap`] module"]
#[doc(alias = "FFLTCAP")]
pub type Ffltcap = crate::Reg<ffltcap::FfltcapSpec>;
#[doc = "Flash Fault Capture Address Register"]
pub mod ffltcap;
#[doc = "FFLTPAR (r) register accessor: Flash ECC Fault Parity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltpar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffltpar`] module"]
#[doc(alias = "FFLTPAR")]
pub type Ffltpar = crate::Reg<ffltpar::FfltparSpec>;
#[doc = "Flash ECC Fault Parity Register"]
pub mod ffltpar;
#[doc = "FFLTSYN (r) register accessor: Flash ECC Fault Syndrome Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltsyn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffltsyn`] module"]
#[doc(alias = "FFLTSYN")]
pub type Ffltsyn = crate::Reg<ffltsyn::FfltsynSpec>;
#[doc = "Flash ECC Fault Syndrome Register"]
pub mod ffltsyn;
#[doc = "CRP (rw) register accessor: CFM Page Read Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crp`] module"]
#[doc(alias = "CRP")]
pub type Crp = crate::Reg<crp::CrpSpec>;
#[doc = "CFM Page Read Protect Register"]
pub mod crp;
#[doc = "HSMCTRL (rw) register accessor: HSM CFM PAGES Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsmctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsmctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsmctrl`] module"]
#[doc(alias = "HSMCTRL")]
pub type Hsmctrl = crate::Reg<hsmctrl::HsmctrlSpec>;
#[doc = "HSM CFM PAGES Control Register"]
pub mod hsmctrl;
