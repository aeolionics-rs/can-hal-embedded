#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    statusa: Statusa,
    statusb: Statusb,
    _reserved3: [u8; 0x0ff4],
    regions: [Regions; 24],
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Status A"]
    #[inline(always)]
    pub const fn statusa(&self) -> &Statusa {
        &self.statusa
    }
    #[doc = "0x08 - Status B"]
    #[inline(always)]
    pub const fn statusb(&self) -> &Statusb {
        &self.statusb
    }
    #[doc = "0x1000..0x1180 - REGIONS\\[%s\\]"]
    #[inline(always)]
    pub const fn regions(&self, n: usize) -> &Regions {
        &self.regions[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1180 - REGIONS\\[%s\\]"]
    #[inline(always)]
    pub fn regions_iter(&self) -> impl Iterator<Item = &Regions> {
        self.regions.iter()
    }
}
#[doc = "CTRL (w) register accessor: Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "STATUSA (r) register accessor: Status A\n\nYou can [`read`](crate::Reg::read) this register and get [`statusa::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusa`] module"]
#[doc(alias = "STATUSA")]
pub type Statusa = crate::Reg<statusa::StatusaSpec>;
#[doc = "Status A"]
pub mod statusa;
#[doc = "STATUSB (rw) register accessor: Status B\n\nYou can [`read`](crate::Reg::read) this register and get [`statusb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusb`] module"]
#[doc(alias = "STATUSB")]
pub type Statusb = crate::Reg<statusb::StatusbSpec>;
#[doc = "Status B"]
pub mod statusb;
#[doc = "REGIONS\\[%s\\]"]
pub use self::regions::Regions;
#[doc = r"Cluster"]
#[doc = "REGIONS\\[%s\\]"]
pub mod regions;
