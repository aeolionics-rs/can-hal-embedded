#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `FPSEEN` reader - Frame Pulse Event Enable Bit"]
pub type FpseenR = crate::BitReader;
#[doc = "Field `FPSEEN` writer - Frame Pulse Event Enable Bit"]
pub type FpseenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Frame Pulse Event Enable Bit"]
    #[inline(always)]
    pub fn fpseen(&self) -> FpseenR {
        FpseenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Pulse Event Enable Bit"]
    #[inline(always)]
    pub fn fpseen(&mut self) -> FpseenW<'_, EvctrlSpec> {
        FpseenW::new(self, 0)
    }
}
#[doc = "SPI Event Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
