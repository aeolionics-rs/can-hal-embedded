#[doc = "Register `EPINFO` reader"]
pub type R = crate::R<EpinfoSpec>;
#[doc = "Field `TXENDPOINTS` reader - number of tx endpoints"]
pub type TxendpointsR = crate::FieldReader;
#[doc = "Field `RXENDPOINTS` reader - number of rx endpoints"]
pub type RxendpointsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - number of tx endpoints"]
    #[inline(always)]
    pub fn txendpoints(&self) -> TxendpointsR {
        TxendpointsR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - number of rx endpoints"]
    #[inline(always)]
    pub fn rxendpoints(&self) -> RxendpointsR {
        RxendpointsR::new((self.bits >> 4) & 0x0f)
    }
}
#[doc = "Allows Read-Back Of The Number Of TX And RX Endpoints\n\nYou can [`read`](crate::Reg::read) this register and get [`epinfo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpinfoSpec;
impl crate::RegisterSpec for EpinfoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`epinfo::R`](R) reader structure"]
impl crate::Readable for EpinfoSpec {}
#[doc = "`reset()` method sets EPINFO to value 0xff"]
impl crate::Resettable for EpinfoSpec {
    const RESET_VALUE: u8 = 0xff;
}
