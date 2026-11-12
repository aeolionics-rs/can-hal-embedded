#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    dbgctrl: Dbgctrl,
    crcpolya: Crcpolya,
    crcpolyb: Crcpolyb,
    _reserved5: [u8; 0x04],
    intstat3: Intstat3,
    intstat2: Intstat2,
    intstat1: Intstat1,
    _reserved8: [u8; 0x2c],
    channel: (),
}
impl RegisterBlock {
    #[doc = "0x00 - DMA CONTROL A REGISTER"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - DMA CONTROL B REGISTER"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x08 - DEBUG CONTROL REGISTER"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x0c - DMA CRC POLYNOMIAL A REGISTER"]
    #[inline(always)]
    pub const fn crcpolya(&self) -> &Crcpolya {
        &self.crcpolya
    }
    #[doc = "0x10 - DMA CRC POLYNOMIAL B REGISTER"]
    #[inline(always)]
    pub const fn crcpolyb(&self) -> &Crcpolyb {
        &self.crcpolyb
    }
    #[doc = "0x18 - DMA INTERRUPT PRIORITY 3 STATUS REGISTER"]
    #[inline(always)]
    pub const fn intstat3(&self) -> &Intstat3 {
        &self.intstat3
    }
    #[doc = "0x1c - DMA INTERRUPT PRIORITY 2 STATUS REGISTER"]
    #[inline(always)]
    pub const fn intstat2(&self) -> &Intstat2 {
        &self.intstat2
    }
    #[doc = "0x20 - DMA INTERRUPT PRIORITY 1 STATUS REGISTER"]
    #[inline(always)]
    pub const fn intstat1(&self) -> &Intstat1 {
        &self.intstat1
    }
    #[doc = "0x50..0x2b0 - CHANNEL\\[%s\\]"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(80).add(80 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x2b0 - CHANNEL\\[%s\\]"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        (0..8).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(80).add(80 * n).cast() })
    }
}
#[doc = "CTRLA (rw) register accessor: DMA CONTROL A REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "DMA CONTROL A REGISTER"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: DMA CONTROL B REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "DMA CONTROL B REGISTER"]
pub mod ctrlb;
#[doc = "DBGCTRL (rw) register accessor: DEBUG CONTROL REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`] module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "DEBUG CONTROL REGISTER"]
pub mod dbgctrl;
#[doc = "CRCPOLYA (rw) register accessor: DMA CRC POLYNOMIAL A REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpolya::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpolya::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpolya`] module"]
#[doc(alias = "CRCPOLYA")]
pub type Crcpolya = crate::Reg<crcpolya::CrcpolyaSpec>;
#[doc = "DMA CRC POLYNOMIAL A REGISTER"]
pub mod crcpolya;
#[doc = "CRCPOLYB (rw) register accessor: DMA CRC POLYNOMIAL B REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpolyb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpolyb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpolyb`] module"]
#[doc(alias = "CRCPOLYB")]
pub type Crcpolyb = crate::Reg<crcpolyb::CrcpolybSpec>;
#[doc = "DMA CRC POLYNOMIAL B REGISTER"]
pub mod crcpolyb;
#[doc = "INTSTAT3 (r) register accessor: DMA INTERRUPT PRIORITY 3 STATUS REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat3`] module"]
#[doc(alias = "INTSTAT3")]
pub type Intstat3 = crate::Reg<intstat3::Intstat3Spec>;
#[doc = "DMA INTERRUPT PRIORITY 3 STATUS REGISTER"]
pub mod intstat3;
#[doc = "INTSTAT2 (r) register accessor: DMA INTERRUPT PRIORITY 2 STATUS REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat2`] module"]
#[doc(alias = "INTSTAT2")]
pub type Intstat2 = crate::Reg<intstat2::Intstat2Spec>;
#[doc = "DMA INTERRUPT PRIORITY 2 STATUS REGISTER"]
pub mod intstat2;
#[doc = "INTSTAT1 (r) register accessor: DMA INTERRUPT PRIORITY 1 STATUS REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat1`] module"]
#[doc(alias = "INTSTAT1")]
pub type Intstat1 = crate::Reg<intstat1::Intstat1Spec>;
#[doc = "DMA INTERRUPT PRIORITY 1 STATUS REGISTER"]
pub mod intstat1;
#[doc = "CHANNEL\\[%s\\]"]
pub use self::channel::Channel;
#[doc = r"Cluster"]
#[doc = "CHANNEL\\[%s\\]"]
pub mod channel;
