#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `CANCEL` reader - Cancel Command"]
pub type CancelR = crate::BitReader;
#[doc = "Field `CANCEL` writer - Cancel Command"]
pub type CancelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cancel Command"]
    #[inline(always)]
    pub fn cancel(&self) -> CancelR {
        CancelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cancel Command"]
    #[inline(always)]
    pub fn cancel(&mut self) -> CancelW<'_, CtrlbSpec> {
        CancelW::new(self, 0)
    }
}
#[doc = "CONTROL REGISTER B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}
