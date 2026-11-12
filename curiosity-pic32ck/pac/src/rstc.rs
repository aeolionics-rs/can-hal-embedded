#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rcause: Rcause,
    bkupexit: Bkupexit,
    _reserved2: [u8; 0x01],
    dbgctrl: Dbgctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Reset Cause"]
    #[inline(always)]
    pub const fn rcause(&self) -> &Rcause {
        &self.rcause
    }
    #[doc = "0x02 - Backup Exit Source. Implemented only if RSTC_BACKUP_IMPLEMENTED=1"]
    #[inline(always)]
    pub const fn bkupexit(&self) -> &Bkupexit {
        &self.bkupexit
    }
    #[doc = "0x04 - Debug Control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
}
#[doc = "RCAUSE (r) register accessor: Reset Cause\n\nYou can [`read`](crate::Reg::read) this register and get [`rcause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcause`] module"]
#[doc(alias = "RCAUSE")]
pub type Rcause = crate::Reg<rcause::RcauseSpec>;
#[doc = "Reset Cause"]
pub mod rcause;
#[doc = "BKUPEXIT (r) register accessor: Backup Exit Source. Implemented only if RSTC_BACKUP_IMPLEMENTED=1\n\nYou can [`read`](crate::Reg::read) this register and get [`bkupexit::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkupexit`] module"]
#[doc(alias = "BKUPEXIT")]
pub type Bkupexit = crate::Reg<bkupexit::BkupexitSpec>;
#[doc = "Backup Exit Source. Implemented only if RSTC_BACKUP_IMPLEMENTED=1"]
pub mod bkupexit;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`] module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug Control"]
pub mod dbgctrl;
