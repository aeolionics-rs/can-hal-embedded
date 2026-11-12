#[repr(C)]
#[doc = "MODE2_ALARM\\[%s\\]"]
#[doc(alias = "MODE2_ALARM")]
pub struct Mode2Alarm {
    alarm: Alarm,
    mask: Mask,
}
impl Mode2Alarm {
    #[doc = "0x00 - MODE2_ALARM Alarm n Value"]
    #[inline(always)]
    pub const fn alarm(&self) -> &Alarm {
        &self.alarm
    }
    #[doc = "0x04 - MODE2_ALARM Alarm n Mask"]
    #[inline(always)]
    pub const fn mask(&self) -> &Mask {
        &self.mask
    }
}
#[doc = "ALARM (rw) register accessor: MODE2_ALARM Alarm n Value\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm`] module"]
#[doc(alias = "ALARM")]
pub type Alarm = crate::Reg<alarm::AlarmSpec>;
#[doc = "MODE2_ALARM Alarm n Value"]
pub mod alarm;
#[doc = "MASK (rw) register accessor: MODE2_ALARM Alarm n Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`] module"]
#[doc(alias = "MASK")]
pub type Mask = crate::Reg<mask::MaskSpec>;
#[doc = "MODE2_ALARM Alarm n Mask"]
pub mod mask;
