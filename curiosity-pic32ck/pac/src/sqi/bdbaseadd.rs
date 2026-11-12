#[doc = "Register `BDBASEADD` reader"]
pub type R = crate::R<BdbaseaddSpec>;
#[doc = "Register `BDBASEADD` writer"]
pub type W = crate::W<BdbaseaddSpec>;
#[doc = "Field `BDADDR` reader - This register field contains the base address of the DMA. This register must be updated only when the DMA is IDLE."]
pub type BdaddrR = crate::FieldReader<u32>;
#[doc = "Field `BDADDR` writer - This register field contains the base address of the DMA. This register must be updated only when the DMA is IDLE."]
pub type BdaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register field contains the base address of the DMA. This register must be updated only when the DMA is IDLE."]
    #[inline(always)]
    pub fn bdaddr(&self) -> BdaddrR {
        BdaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register field contains the base address of the DMA. This register must be updated only when the DMA is IDLE."]
    #[inline(always)]
    pub fn bdaddr(&mut self) -> BdaddrW<'_, BdbaseaddSpec> {
        BdaddrW::new(self, 0)
    }
}
#[doc = "BD_BASE_ADDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdbaseadd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdbaseadd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdbaseaddSpec;
impl crate::RegisterSpec for BdbaseaddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdbaseadd::R`](R) reader structure"]
impl crate::Readable for BdbaseaddSpec {}
#[doc = "`write(|w| ..)` method takes [`bdbaseadd::W`](W) writer structure"]
impl crate::Writable for BdbaseaddSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BDBASEADD to value 0"]
impl crate::Resettable for BdbaseaddSpec {}
