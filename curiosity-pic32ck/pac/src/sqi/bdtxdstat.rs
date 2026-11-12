#[doc = "Register `BDTXDSTAT` reader"]
pub type R = crate::R<BdtxdstatSpec>;
#[doc = "Register `BDTXDSTAT` writer"]
pub type W = crate::W<BdtxdstatSpec>;
#[doc = "Field `TXCURBUFLEN` reader - This register field gives the length of the current buffer transmit length"]
pub type TxcurbuflenR = crate::FieldReader<u16>;
#[doc = "Field `TXCURBUFLEN` writer - This register field gives the length of the current buffer transmit length"]
pub type TxcurbuflenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TXBUFCNT` reader - This register field gives the information about the internal transmit FIFO space."]
pub type TxbufcntR = crate::FieldReader;
#[doc = "Field `TXBUFCNT` writer - This register field gives the information about the internal transmit FIFO space."]
pub type TxbufcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - This register field gives the length of the current buffer transmit length"]
    #[inline(always)]
    pub fn txcurbuflen(&self) -> TxcurbuflenR {
        TxcurbuflenR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - This register field gives the information about the internal transmit FIFO space."]
    #[inline(always)]
    pub fn txbufcnt(&self) -> TxbufcntR {
        TxbufcntR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register field gives the length of the current buffer transmit length"]
    #[inline(always)]
    pub fn txcurbuflen(&mut self) -> TxcurbuflenW<'_, BdtxdstatSpec> {
        TxcurbuflenW::new(self, 0)
    }
    #[doc = "Bits 16:23 - This register field gives the information about the internal transmit FIFO space."]
    #[inline(always)]
    pub fn txbufcnt(&mut self) -> TxbufcntW<'_, BdtxdstatSpec> {
        TxbufcntW::new(self, 16)
    }
}
#[doc = "BD_TX_DMA_STATUS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdtxdstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtxdstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdtxdstatSpec;
impl crate::RegisterSpec for BdtxdstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdtxdstat::R`](R) reader structure"]
impl crate::Readable for BdtxdstatSpec {}
#[doc = "`write(|w| ..)` method takes [`bdtxdstat::W`](W) writer structure"]
impl crate::Writable for BdtxdstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BDTXDSTAT to value 0"]
impl crate::Resettable for BdtxdstatSpec {}
