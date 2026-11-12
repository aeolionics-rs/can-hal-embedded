#[repr(C)]
#[doc = "CONFIG\\[%s\\]"]
#[doc(alias = "CONFIG")]
pub struct Config {
    corctrl: Corctrl,
    chncfg1: Chncfg1,
    chncfg2: Chncfg2,
    chncfg3: Chncfg3,
    chncfg4: Chncfg4,
    chncfg5: Chncfg5,
    _reserved_6_calctrl: [u8; 0x04],
    evctrl: Evctrl,
}
impl Config {
    #[doc = "0x00 - SARCORE Control"]
    #[inline(always)]
    pub const fn corctrl(&self) -> &Corctrl {
        &self.corctrl
    }
    #[doc = "0x04 - Channel Configuration 1 (LVL/CMPEN)"]
    #[inline(always)]
    pub const fn chncfg1(&self) -> &Chncfg1 {
        &self.chncfg1
    }
    #[doc = "0x08 - Channel Configuration 2(FRACT/CSS)"]
    #[inline(always)]
    pub const fn chncfg2(&self) -> &Chncfg2 {
        &self.chncfg2
    }
    #[doc = "0x0c - Channel Configuration3 (SIGN/DIFF)"]
    #[inline(always)]
    pub const fn chncfg3(&self) -> &Chncfg3 {
        &self.chncfg3
    }
    #[doc = "0x10 - Channel Configuration 4 (TRGSRC)"]
    #[inline(always)]
    pub const fn chncfg4(&self) -> &Chncfg4 {
        &self.chncfg4
    }
    #[doc = "0x14 - Channel Configuration 5 (TRGSRC)"]
    #[inline(always)]
    pub const fn chncfg5(&self) -> &Chncfg5 {
        &self.chncfg5
    }
    #[doc = "0x18 - SARCORE Calibration Value"]
    #[inline(always)]
    pub const fn calctrl_fuses_adc_sarcore_12bit_v7c0_mode(&self) -> &CalctrlFusesAdcSarcore12bitV7c0Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - SARCORE Calibration Value"]
    #[inline(always)]
    pub const fn calctrl_fuses_adc_sarcore_12bit_v7a0_mode(&self) -> &CalctrlFusesAdcSarcore12bitV7a0Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - SARCORE Calibration Value"]
    #[inline(always)]
    pub const fn calctrl(&self) -> &Calctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
}
#[doc = "CORCTRL (rw) register accessor: SARCORE Control\n\nYou can [`read`](crate::Reg::read) this register and get [`corctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`corctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@corctrl`] module"]
#[doc(alias = "CORCTRL")]
pub type Corctrl = crate::Reg<corctrl::CorctrlSpec>;
#[doc = "SARCORE Control"]
pub mod corctrl;
#[doc = "CHNCFG1 (rw) register accessor: Channel Configuration 1 (LVL/CMPEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`chncfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chncfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chncfg1`] module"]
#[doc(alias = "CHNCFG1")]
pub type Chncfg1 = crate::Reg<chncfg1::Chncfg1Spec>;
#[doc = "Channel Configuration 1 (LVL/CMPEN)"]
pub mod chncfg1;
#[doc = "CHNCFG2 (rw) register accessor: Channel Configuration 2(FRACT/CSS)\n\nYou can [`read`](crate::Reg::read) this register and get [`chncfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chncfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chncfg2`] module"]
#[doc(alias = "CHNCFG2")]
pub type Chncfg2 = crate::Reg<chncfg2::Chncfg2Spec>;
#[doc = "Channel Configuration 2(FRACT/CSS)"]
pub mod chncfg2;
#[doc = "CHNCFG3 (rw) register accessor: Channel Configuration3 (SIGN/DIFF)\n\nYou can [`read`](crate::Reg::read) this register and get [`chncfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chncfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chncfg3`] module"]
#[doc(alias = "CHNCFG3")]
pub type Chncfg3 = crate::Reg<chncfg3::Chncfg3Spec>;
#[doc = "Channel Configuration3 (SIGN/DIFF)"]
pub mod chncfg3;
#[doc = "CHNCFG4 (rw) register accessor: Channel Configuration 4 (TRGSRC)\n\nYou can [`read`](crate::Reg::read) this register and get [`chncfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chncfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chncfg4`] module"]
#[doc(alias = "CHNCFG4")]
pub type Chncfg4 = crate::Reg<chncfg4::Chncfg4Spec>;
#[doc = "Channel Configuration 4 (TRGSRC)"]
pub mod chncfg4;
#[doc = "CHNCFG5 (rw) register accessor: Channel Configuration 5 (TRGSRC)\n\nYou can [`read`](crate::Reg::read) this register and get [`chncfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chncfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chncfg5`] module"]
#[doc(alias = "CHNCFG5")]
pub type Chncfg5 = crate::Reg<chncfg5::Chncfg5Spec>;
#[doc = "Channel Configuration 5 (TRGSRC)"]
pub mod chncfg5;
#[doc = "CALCTRL (rw) register accessor: SARCORE Calibration Value\n\nYou can [`read`](crate::Reg::read) this register and get [`calctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calctrl`] module"]
#[doc(alias = "CALCTRL")]
pub type Calctrl = crate::Reg<calctrl::CalctrlSpec>;
#[doc = "SARCORE Calibration Value"]
pub mod calctrl;
#[doc = "CALCTRL_FUSES_ADC_SARCORE_12BIT_V7A0_MODE (rw) register accessor: SARCORE Calibration Value\n\nYou can [`read`](crate::Reg::read) this register and get [`calctrl_fuses_adc_sarcore_12bit_v7a0_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calctrl_fuses_adc_sarcore_12bit_v7a0_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calctrl_fuses_adc_sarcore_12bit_v7a0_mode`] module"]
#[doc(alias = "CALCTRL_FUSES_ADC_SARCORE_12BIT_V7A0_MODE")]
pub type CalctrlFusesAdcSarcore12bitV7a0Mode = crate::Reg<calctrl_fuses_adc_sarcore_12bit_v7a0_mode::CalctrlFusesAdcSarcore12bitV7a0ModeSpec>;
#[doc = "SARCORE Calibration Value"]
pub mod calctrl_fuses_adc_sarcore_12bit_v7a0_mode;
#[doc = "CALCTRL_FUSES_ADC_SARCORE_12BIT_V7C0_MODE (rw) register accessor: SARCORE Calibration Value\n\nYou can [`read`](crate::Reg::read) this register and get [`calctrl_fuses_adc_sarcore_12bit_v7c0_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calctrl_fuses_adc_sarcore_12bit_v7c0_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calctrl_fuses_adc_sarcore_12bit_v7c0_mode`] module"]
#[doc(alias = "CALCTRL_FUSES_ADC_SARCORE_12BIT_V7C0_MODE")]
pub type CalctrlFusesAdcSarcore12bitV7c0Mode = crate::Reg<calctrl_fuses_adc_sarcore_12bit_v7c0_mode::CalctrlFusesAdcSarcore12bitV7c0ModeSpec>;
#[doc = "SARCORE Calibration Value"]
pub mod calctrl_fuses_adc_sarcore_12bit_v7c0_mode;
#[doc = "EVCTRL (rw) register accessor: Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`] module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "Event Control"]
pub mod evctrl;
