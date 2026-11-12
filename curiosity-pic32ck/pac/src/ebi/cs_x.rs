#[repr(C)]
#[doc = "CS_X\\[%s\\]"]
#[doc(alias = "CS_X")]
pub struct CsX {
    smc_setup: SmcSetup,
    smc_pulse: SmcPulse,
    smc_cycle: SmcCycle,
    smc_mode: SmcMode,
}
impl CsX {
    #[doc = "0x00 - SMC Setup Register"]
    #[inline(always)]
    pub const fn smc_setup(&self) -> &SmcSetup {
        &self.smc_setup
    }
    #[doc = "0x04 - SMC Pulse Register"]
    #[inline(always)]
    pub const fn smc_pulse(&self) -> &SmcPulse {
        &self.smc_pulse
    }
    #[doc = "0x08 - SMC Cycle Register"]
    #[inline(always)]
    pub const fn smc_cycle(&self) -> &SmcCycle {
        &self.smc_cycle
    }
    #[doc = "0x0c - SMC Mode Register"]
    #[inline(always)]
    pub const fn smc_mode(&self) -> &SmcMode {
        &self.smc_mode
    }
}
#[doc = "SMC_SETUP (rw) register accessor: SMC Setup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smc_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smc_setup`] module"]
#[doc(alias = "SMC_SETUP")]
pub type SmcSetup = crate::Reg<smc_setup::SmcSetupSpec>;
#[doc = "SMC Setup Register"]
pub mod smc_setup;
#[doc = "SMC_PULSE (rw) register accessor: SMC Pulse Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_pulse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smc_pulse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smc_pulse`] module"]
#[doc(alias = "SMC_PULSE")]
pub type SmcPulse = crate::Reg<smc_pulse::SmcPulseSpec>;
#[doc = "SMC Pulse Register"]
pub mod smc_pulse;
#[doc = "SMC_CYCLE (rw) register accessor: SMC Cycle Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_cycle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smc_cycle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smc_cycle`] module"]
#[doc(alias = "SMC_CYCLE")]
pub type SmcCycle = crate::Reg<smc_cycle::SmcCycleSpec>;
#[doc = "SMC Cycle Register"]
pub mod smc_cycle;
#[doc = "SMC_MODE (rw) register accessor: SMC Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smc_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smc_mode`] module"]
#[doc(alias = "SMC_MODE")]
pub type SmcMode = crate::Reg<smc_mode::SmcModeSpec>;
#[doc = "SMC Mode Register"]
pub mod smc_mode;
