#[doc = "Register `FIFOSPACE` reader"]
pub type R = crate::R<FifospaceSpec>;
#[doc = "Field `TXSPACE` reader - Tx FIFO Empty Space"]
pub type TxspaceR = crate::FieldReader;
#[doc = "Field `RXSPACE` reader - Rx FIFO Filled Space"]
pub type RxspaceR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Tx FIFO Empty Space"]
    #[inline(always)]
    pub fn txspace(&self) -> TxspaceR {
        TxspaceR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Rx FIFO Filled Space"]
    #[inline(always)]
    pub fn rxspace(&self) -> RxspaceR {
        RxspaceR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[doc = "SPIM FIFO Space\n\nYou can [`read`](crate::Reg::read) this register and get [`fifospace::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifospaceSpec;
impl crate::RegisterSpec for FifospaceSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fifospace::R`](R) reader structure"]
impl crate::Readable for FifospaceSpec {}
#[doc = "`reset()` method sets FIFOSPACE to value 0"]
impl crate::Resettable for FifospaceSpec {}
