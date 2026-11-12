#[repr(C)]
#[doc = "TXFUNADDR\\[%s\\]"]
#[doc(alias = "TXFUNADDR")]
pub struct Txfunaddr {
    txfuncaddrep: Txfuncaddrep,
    _reserved1: [u8; 0x01],
    txhubaddrep: Txhubaddrep,
    txhubport: Txhubport,
    rxfuncaddrep: Rxfuncaddrep,
    _reserved4: [u8; 0x01],
    rxhubaddrep: Rxhubaddrep,
    rxhubport: Rxhubport,
}
impl Txfunaddr {
    #[doc = "0x00 - Relevant In Host Mode Only. Address Of The Target Function."]
    #[inline(always)]
    pub const fn txfuncaddrep(&self) -> &Txfuncaddrep {
        &self.txfuncaddrep
    }
    #[doc = "0x02 - Relevant In Host Mode Only. HUB Address"]
    #[inline(always)]
    pub const fn txhubaddrep(&self) -> &Txhubaddrep {
        &self.txhubaddrep
    }
    #[doc = "0x03 - Relevant In Host Mode Only. HUB Address"]
    #[inline(always)]
    pub const fn txhubport(&self) -> &Txhubport {
        &self.txhubport
    }
    #[doc = "0x04 - Relevant In Host Mode Only. Address Of The Target Function."]
    #[inline(always)]
    pub const fn rxfuncaddrep(&self) -> &Rxfuncaddrep {
        &self.rxfuncaddrep
    }
    #[doc = "0x06 - Relevant In Host Mode Only. HUB Address"]
    #[inline(always)]
    pub const fn rxhubaddrep(&self) -> &Rxhubaddrep {
        &self.rxhubaddrep
    }
    #[doc = "0x07 - Relevant In Host Mode Only. HUB Address"]
    #[inline(always)]
    pub const fn rxhubport(&self) -> &Rxhubport {
        &self.rxhubport
    }
}
#[doc = "TXFUNCADDREP (rw) register accessor: Relevant In Host Mode Only. Address Of The Target Function.\n\nYou can [`read`](crate::Reg::read) this register and get [`txfuncaddrep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfuncaddrep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfuncaddrep`] module"]
#[doc(alias = "TXFUNCADDREP")]
pub type Txfuncaddrep = crate::Reg<txfuncaddrep::TxfuncaddrepSpec>;
#[doc = "Relevant In Host Mode Only. Address Of The Target Function."]
pub mod txfuncaddrep;
#[doc = "TXHUBADDREP (rw) register accessor: Relevant In Host Mode Only. HUB Address\n\nYou can [`read`](crate::Reg::read) this register and get [`txhubaddrep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txhubaddrep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubaddrep`] module"]
#[doc(alias = "TXHUBADDREP")]
pub type Txhubaddrep = crate::Reg<txhubaddrep::TxhubaddrepSpec>;
#[doc = "Relevant In Host Mode Only. HUB Address"]
pub mod txhubaddrep;
#[doc = "TXHUBPORT (rw) register accessor: Relevant In Host Mode Only. HUB Address\n\nYou can [`read`](crate::Reg::read) this register and get [`txhubport::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txhubport::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubport`] module"]
#[doc(alias = "TXHUBPORT")]
pub type Txhubport = crate::Reg<txhubport::TxhubportSpec>;
#[doc = "Relevant In Host Mode Only. HUB Address"]
pub mod txhubport;
#[doc = "RXFUNCADDREP (rw) register accessor: Relevant In Host Mode Only. Address Of The Target Function.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfuncaddrep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfuncaddrep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfuncaddrep`] module"]
#[doc(alias = "RXFUNCADDREP")]
pub type Rxfuncaddrep = crate::Reg<rxfuncaddrep::RxfuncaddrepSpec>;
#[doc = "Relevant In Host Mode Only. Address Of The Target Function."]
pub mod rxfuncaddrep;
#[doc = "RXHUBADDREP (rw) register accessor: Relevant In Host Mode Only. HUB Address\n\nYou can [`read`](crate::Reg::read) this register and get [`rxhubaddrep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxhubaddrep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubaddrep`] module"]
#[doc(alias = "RXHUBADDREP")]
pub type Rxhubaddrep = crate::Reg<rxhubaddrep::RxhubaddrepSpec>;
#[doc = "Relevant In Host Mode Only. HUB Address"]
pub mod rxhubaddrep;
#[doc = "RXHUBPORT (rw) register accessor: Relevant In Host Mode Only. HUB Address\n\nYou can [`read`](crate::Reg::read) this register and get [`rxhubport::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxhubport::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubport`] module"]
#[doc(alias = "RXHUBPORT")]
pub type Rxhubport = crate::Reg<rxhubport::RxhubportSpec>;
#[doc = "Relevant In Host Mode Only. HUB Address"]
pub mod rxhubport;
