#[doc = "Register `CTHSBT` reader"]
pub type R = crate::R<CthsbtSpec>;
#[doc = "Register `CTHSBT` writer"]
pub type W = crate::W<CthsbtSpec>;
#[doc = "Field `HSTMEOUTADD` reader - this register represents the value to be added to the mimumum high speed timeout period of 736 bit times"]
pub type HstmeoutaddR = crate::FieldReader;
#[doc = "Field `HSTMEOUTADD` writer - this register represents the value to be added to the mimumum high speed timeout period of 736 bit times"]
pub type HstmeoutaddW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - this register represents the value to be added to the mimumum high speed timeout period of 736 bit times"]
    #[inline(always)]
    pub fn hstmeoutadd(&self) -> HstmeoutaddR {
        HstmeoutaddR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - this register represents the value to be added to the mimumum high speed timeout period of 736 bit times"]
    #[inline(always)]
    pub fn hstmeoutadd(&mut self) -> HstmeoutaddW<'_, CthsbtSpec> {
        HstmeoutaddW::new(self, 0)
    }
}
#[doc = "This Register Represents The Value To Be Added To The Minimum High Speed Timeout Period Of 736 Bit Times.\n\nYou can [`read`](crate::Reg::read) this register and get [`cthsbt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cthsbt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CthsbtSpec;
impl crate::RegisterSpec for CthsbtSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cthsbt::R`](R) reader structure"]
impl crate::Readable for CthsbtSpec {}
#[doc = "`write(|w| ..)` method takes [`cthsbt::W`](W) writer structure"]
impl crate::Writable for CthsbtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTHSBT to value 0"]
impl crate::Resettable for CthsbtSpec {}
