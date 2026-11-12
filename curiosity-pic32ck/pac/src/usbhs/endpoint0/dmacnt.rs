#[repr(C)]
#[doc = "DMACNT\\[%s\\]"]
#[doc(alias = "DMACNT")]
pub struct Dmacnt {
    dmacntl: Dmacntl,
    dmaaddr: Dmaaddr,
    dmacount: Dmacount,
}
impl Dmacnt {
    #[doc = "0x00 - DMA Transfer Control For Each Channel"]
    #[inline(always)]
    pub const fn dmacntl(&self) -> &Dmacntl {
        &self.dmacntl
    }
    #[doc = "0x04 - Memory Address Of The Corresponding DMA Channel"]
    #[inline(always)]
    pub const fn dmaaddr(&self) -> &Dmaaddr {
        &self.dmaaddr
    }
    #[doc = "0x08 - Current DMA Count Of The Transfer"]
    #[inline(always)]
    pub const fn dmacount(&self) -> &Dmacount {
        &self.dmacount
    }
}
#[doc = "DMACNTL (rw) register accessor: DMA Transfer Control For Each Channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacntl`] module"]
#[doc(alias = "DMACNTL")]
pub type Dmacntl = crate::Reg<dmacntl::DmacntlSpec>;
#[doc = "DMA Transfer Control For Each Channel"]
pub mod dmacntl;
#[doc = "DMAADDR (rw) register accessor: Memory Address Of The Corresponding DMA Channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaaddr`] module"]
#[doc(alias = "DMAADDR")]
pub type Dmaaddr = crate::Reg<dmaaddr::DmaaddrSpec>;
#[doc = "Memory Address Of The Corresponding DMA Channel"]
pub mod dmaaddr;
#[doc = "DMACOUNT (rw) register accessor: Current DMA Count Of The Transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacount`] module"]
#[doc(alias = "DMACOUNT")]
pub type Dmacount = crate::Reg<dmacount::DmacountSpec>;
#[doc = "Current DMA Count Of The Transfer"]
pub mod dmacount;
