#[doc = "Register `TXFIFOADD` reader"]
pub type R = crate::R<TxfifoaddSpec>;
#[doc = "Register `TXFIFOADD` writer"]
pub type W = crate::W<TxfifoaddSpec>;
#[doc = "Field `ADDR` reader - start address of the endpoint FIFO"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - start address of the endpoint FIFO"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - start address of the endpoint FIFO"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits & 0x1fff)
    }
}
impl W {
    #[doc = "Bits 0:12 - start address of the endpoint FIFO"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, TxfifoaddSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Controls The Start Address Of the Selected TX Endpoint FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifoadd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfifoadd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfifoaddSpec;
impl crate::RegisterSpec for TxfifoaddSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`txfifoadd::R`](R) reader structure"]
impl crate::Readable for TxfifoaddSpec {}
#[doc = "`write(|w| ..)` method takes [`txfifoadd::W`](W) writer structure"]
impl crate::Writable for TxfifoaddSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXFIFOADD to value 0"]
impl crate::Resettable for TxfifoaddSpec {}
