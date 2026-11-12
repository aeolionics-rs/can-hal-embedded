#[doc = "Register `PLL0REFDIV` reader"]
pub type R = crate::R<Pll0refdivSpec>;
#[doc = "Register `PLL0REFDIV` writer"]
pub type W = crate::W<Pll0refdivSpec>;
#[doc = "Field `REFDIV` reader - PLL reference division factor"]
pub type RefdivR = crate::FieldReader;
#[doc = "Field `REFDIV` writer - PLL reference division factor"]
pub type RefdivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - PLL reference division factor"]
    #[inline(always)]
    pub fn refdiv(&self) -> RefdivR {
        RefdivR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLL reference division factor"]
    #[inline(always)]
    pub fn refdiv(&mut self) -> RefdivW<'_, Pll0refdivSpec> {
        RefdivW::new(self, 0)
    }
}
#[doc = "PLL reference divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0refdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0refdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll0refdivSpec;
impl crate::RegisterSpec for Pll0refdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0refdiv::R`](R) reader structure"]
impl crate::Readable for Pll0refdivSpec {}
#[doc = "`write(|w| ..)` method takes [`pll0refdiv::W`](W) writer structure"]
impl crate::Writable for Pll0refdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL0REFDIV to value 0"]
impl crate::Resettable for Pll0refdivSpec {}
