#[doc = "Register `VPLEN` reader"]
pub type R = crate::R<VplenSpec>;
#[doc = "Register `VPLEN` writer"]
pub type W = crate::W<VplenSpec>;
#[doc = "Field `VPLEN` reader - duration of VBus pulsing charge in units of 546.1us"]
pub type VplenR = crate::FieldReader;
#[doc = "Field `VPLEN` writer - duration of VBus pulsing charge in units of 546.1us"]
pub type VplenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - duration of VBus pulsing charge in units of 546.1us"]
    #[inline(always)]
    pub fn vplen(&self) -> VplenR {
        VplenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - duration of VBus pulsing charge in units of 546.1us"]
    #[inline(always)]
    pub fn vplen(&mut self) -> VplenW<'_, VplenSpec> {
        VplenW::new(self, 0)
    }
}
#[doc = "Sets The Duration Of The VBus Pulsing Charge\n\nYou can [`read`](crate::Reg::read) this register and get [`vplen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vplen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VplenSpec;
impl crate::RegisterSpec for VplenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vplen::R`](R) reader structure"]
impl crate::Readable for VplenSpec {}
#[doc = "`write(|w| ..)` method takes [`vplen::W`](W) writer structure"]
impl crate::Writable for VplenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VPLEN to value 0x3c"]
impl crate::Resettable for VplenSpec {
    const RESET_VALUE: u8 = 0x3c;
}
