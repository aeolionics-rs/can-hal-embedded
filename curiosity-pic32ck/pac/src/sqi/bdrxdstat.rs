#[doc = "Register `BDRXDSTAT` reader"]
pub type R = crate::R<BdrxdstatSpec>;
#[doc = "Field `RXCURBUFLEN` reader - This register field gives the length of the current buffer receive length"]
pub type RxcurbuflenR = crate::FieldReader<u16>;
#[doc = "Field `RXBUFCNT` reader - This register field gives the information about the internal receive FIFO space."]
pub type RxbufcntR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - This register field gives the length of the current buffer receive length"]
    #[inline(always)]
    pub fn rxcurbuflen(&self) -> RxcurbuflenR {
        RxcurbuflenR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - This register field gives the information about the internal receive FIFO space."]
    #[inline(always)]
    pub fn rxbufcnt(&self) -> RxbufcntR {
        RxbufcntR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "BD_RX_DMA_STATUS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdrxdstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdrxdstatSpec;
impl crate::RegisterSpec for BdrxdstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdrxdstat::R`](R) reader structure"]
impl crate::Readable for BdrxdstatSpec {}
#[doc = "`reset()` method sets BDRXDSTAT to value 0"]
impl crate::Resettable for BdrxdstatSpec {}
