#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cs_x: [CsX; 4],
    _reserved1: [u8; 0xa4],
    smc_wpmr: SmcWpmr,
    smc_wpsr: SmcWpsr,
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - CS_X\\[%s\\]"]
    #[inline(always)]
    pub const fn cs_x(&self, n: usize) -> &CsX {
        &self.cs_x[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - CS_X\\[%s\\]"]
    #[inline(always)]
    pub fn cs_x_iter(&self) -> impl Iterator<Item = &CsX> {
        self.cs_x.iter()
    }
    #[doc = "0xe4 - SMC Write Protection Mode Register"]
    #[inline(always)]
    pub const fn smc_wpmr(&self) -> &SmcWpmr {
        &self.smc_wpmr
    }
    #[doc = "0xe8 - SMC Write Protection Status Register"]
    #[inline(always)]
    pub const fn smc_wpsr(&self) -> &SmcWpsr {
        &self.smc_wpsr
    }
}
#[doc = "CS_X\\[%s\\]"]
pub use self::cs_x::CsX;
#[doc = r"Cluster"]
#[doc = "CS_X\\[%s\\]"]
pub mod cs_x;
#[doc = "SMC_WPMR (rw) register accessor: SMC Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smc_wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smc_wpmr`] module"]
#[doc(alias = "SMC_WPMR")]
pub type SmcWpmr = crate::Reg<smc_wpmr::SmcWpmrSpec>;
#[doc = "SMC Write Protection Mode Register"]
pub mod smc_wpmr;
#[doc = "SMC_WPSR (r) register accessor: SMC Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smc_wpsr`] module"]
#[doc(alias = "SMC_WPSR")]
pub type SmcWpsr = crate::Reg<smc_wpsr::SmcWpsrSpec>;
#[doc = "SMC Write Protection Status Register"]
pub mod smc_wpsr;
