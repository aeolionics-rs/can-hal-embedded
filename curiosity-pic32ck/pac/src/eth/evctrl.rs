#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `CMPEO` reader - Compare Event Out enable"]
pub type CmpeoR = crate::BitReader;
#[doc = "Field `CMPEO` writer - Compare Event Out enable"]
pub type CmpeoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Compare Event Out enable"]
    #[inline(always)]
    pub fn cmpeo(&self) -> CmpeoR {
        CmpeoR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Event Out enable"]
    #[inline(always)]
    pub fn cmpeo(&mut self) -> CmpeoW<'_, EvctrlSpec> {
        CmpeoW::new(self, 0)
    }
}
#[doc = "Event Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {}
