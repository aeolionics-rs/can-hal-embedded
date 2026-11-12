#[doc = "Register `FIFOSIZE` reader"]
pub type R = crate::R<FifosizeSpec>;
#[doc = "Field `TXFIFOSIZE` reader - Tx FIFO Size"]
pub type TxfifosizeR = crate::FieldReader;
#[doc = "Field `RXFIFOSIZE` reader - Rx FIFO Size"]
pub type RxfifosizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Tx FIFO Size"]
    #[inline(always)]
    pub fn txfifosize(&self) -> TxfifosizeR {
        TxfifosizeR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Rx FIFO Size"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RxfifosizeR {
        RxfifosizeR::new((self.bits >> 4) & 0x0f)
    }
}
#[doc = "ENDPOINTX Read Only Register That Returns The Sizes Of The FIFO's Associated With The Selected Additional TX/RX Endpoints. INDEX Regsiter should be set 1-15\n\nYou can [`read`](crate::Reg::read) this register and get [`fifosize::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifosizeSpec;
impl crate::RegisterSpec for FifosizeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fifosize::R`](R) reader structure"]
impl crate::Readable for FifosizeSpec {}
#[doc = "`reset()` method sets FIFOSIZE to value 0"]
impl crate::Resettable for FifosizeSpec {}
