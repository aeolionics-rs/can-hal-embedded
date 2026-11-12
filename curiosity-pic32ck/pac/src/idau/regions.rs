#[repr(C)]
#[doc = "REGIONS\\[%s\\]"]
#[doc(alias = "REGIONS")]
pub struct Regions {
    _reserved_0_rctrl: [u8; 0x04],
    rstatusa: Rstatusa,
    _reserved_2_rstatusb: [u8; 0x04],
    _reserved_3_rstatusc: [u8; 0x04],
}
impl Regions {
    #[doc = "0x00 - Region Control"]
    #[inline(always)]
    pub const fn rctrl_watermark_mode(&self) -> &RctrlWatermarkMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Region Control"]
    #[inline(always)]
    pub const fn rctrl_block_mode(&self) -> &RctrlBlockMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - Region Status A"]
    #[inline(always)]
    pub const fn rstatusa(&self) -> &Rstatusa {
        &self.rstatusa
    }
    #[doc = "0x08 - Region Status B"]
    #[inline(always)]
    pub const fn rstatusb_watermark_mode(&self) -> &RstatusbWatermarkMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Region Status B"]
    #[inline(always)]
    pub const fn rstatusb_block_mode(&self) -> &RstatusbBlockMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Region Status C"]
    #[inline(always)]
    pub const fn rstatusc_link_mode(&self) -> &RstatuscLinkMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Region Status C"]
    #[inline(always)]
    pub const fn rstatusc_block_mode(&self) -> &RstatuscBlockMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
}
#[doc = "RCTRL_BLOCK_MODE (w) register accessor: Region Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rctrl_block_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rctrl_block_mode`] module"]
#[doc(alias = "RCTRL_BLOCK_MODE")]
pub type RctrlBlockMode = crate::Reg<rctrl_block_mode::RctrlBlockModeSpec>;
#[doc = "Region Control"]
pub mod rctrl_block_mode;
#[doc = "RCTRL_WATERMARK_MODE (w) register accessor: Region Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rctrl_watermark_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rctrl_watermark_mode`] module"]
#[doc(alias = "RCTRL_WATERMARK_MODE")]
pub type RctrlWatermarkMode = crate::Reg<rctrl_watermark_mode::RctrlWatermarkModeSpec>;
#[doc = "Region Control"]
pub mod rctrl_watermark_mode;
#[doc = "RSTATUSA (r) register accessor: Region Status A\n\nYou can [`read`](crate::Reg::read) this register and get [`rstatusa::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstatusa`] module"]
#[doc(alias = "RSTATUSA")]
pub type Rstatusa = crate::Reg<rstatusa::RstatusaSpec>;
#[doc = "Region Status A"]
pub mod rstatusa;
#[doc = "RSTATUSB_BLOCK_MODE (r) register accessor: Region Status B\n\nYou can [`read`](crate::Reg::read) this register and get [`rstatusb_block_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstatusb_block_mode`] module"]
#[doc(alias = "RSTATUSB_BLOCK_MODE")]
pub type RstatusbBlockMode = crate::Reg<rstatusb_block_mode::RstatusbBlockModeSpec>;
#[doc = "Region Status B"]
pub mod rstatusb_block_mode;
#[doc = "RSTATUSB_WATERMARK_MODE (r) register accessor: Region Status B\n\nYou can [`read`](crate::Reg::read) this register and get [`rstatusb_watermark_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstatusb_watermark_mode`] module"]
#[doc(alias = "RSTATUSB_WATERMARK_MODE")]
pub type RstatusbWatermarkMode = crate::Reg<rstatusb_watermark_mode::RstatusbWatermarkModeSpec>;
#[doc = "Region Status B"]
pub mod rstatusb_watermark_mode;
#[doc = "RSTATUSC_BLOCK_MODE (r) register accessor: Region Status C\n\nYou can [`read`](crate::Reg::read) this register and get [`rstatusc_block_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstatusc_block_mode`] module"]
#[doc(alias = "RSTATUSC_BLOCK_MODE")]
pub type RstatuscBlockMode = crate::Reg<rstatusc_block_mode::RstatuscBlockModeSpec>;
#[doc = "Region Status C"]
pub mod rstatusc_block_mode;
#[doc = "RSTATUSC_LINK_MODE (r) register accessor: Region Status C\n\nYou can [`read`](crate::Reg::read) this register and get [`rstatusc_link_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstatusc_link_mode`] module"]
#[doc(alias = "RSTATUSC_LINK_MODE")]
pub type RstatuscLinkMode = crate::Reg<rstatusc_link_mode::RstatuscLinkModeSpec>;
#[doc = "Region Status C"]
pub mod rstatusc_link_mode;
