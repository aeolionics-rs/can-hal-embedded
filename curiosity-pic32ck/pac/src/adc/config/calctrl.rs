#[doc = "Register `CALCTRL` reader"]
pub type R = crate::R<CalctrlSpec>;
#[doc = "Register `CALCTRL` writer"]
pub type W = crate::W<CalctrlSpec>;
#[doc = "Field `CALBITS` reader - Calibration Values"]
pub type CalbitsR = crate::FieldReader<u32>;
#[doc = "Field `CALBITS` writer - Calibration Values"]
pub type CalbitsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Calibration Values"]
    #[inline(always)]
    pub fn calbits(&self) -> CalbitsR {
        CalbitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Calibration Values"]
    #[inline(always)]
    pub fn calbits(&mut self) -> CalbitsW<'_, CalctrlSpec> {
        CalbitsW::new(self, 0)
    }
}
#[doc = "SARCORE Calibration Value\n\nYou can [`read`](crate::Reg::read) this register and get [`calctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalctrlSpec;
impl crate::RegisterSpec for CalctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calctrl::R`](R) reader structure"]
impl crate::Readable for CalctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`calctrl::W`](W) writer structure"]
impl crate::Writable for CalctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALCTRL to value 0"]
impl crate::Resettable for CalctrlSpec {}
