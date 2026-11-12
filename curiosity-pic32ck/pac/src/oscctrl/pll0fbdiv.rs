#[doc = "Register `PLL0FBDIV` reader"]
pub type R = crate::R<Pll0fbdivSpec>;
#[doc = "Register `PLL0FBDIV` writer"]
pub type W = crate::W<Pll0fbdivSpec>;
#[doc = "Field `FBDIV` reader - PLL Feed-Back Divider Factor"]
pub type FbdivR = crate::FieldReader<u16>;
#[doc = "Field `FBDIV` writer - PLL Feed-Back Divider Factor"]
pub type FbdivW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - PLL Feed-Back Divider Factor"]
    #[inline(always)]
    pub fn fbdiv(&self) -> FbdivR {
        FbdivR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - PLL Feed-Back Divider Factor"]
    #[inline(always)]
    pub fn fbdiv(&mut self) -> FbdivW<'_, Pll0fbdivSpec> {
        FbdivW::new(self, 0)
    }
}
#[doc = "PLL Feed-Back Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0fbdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0fbdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll0fbdivSpec;
impl crate::RegisterSpec for Pll0fbdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0fbdiv::R`](R) reader structure"]
impl crate::Readable for Pll0fbdivSpec {}
#[doc = "`write(|w| ..)` method takes [`pll0fbdiv::W`](W) writer structure"]
impl crate::Writable for Pll0fbdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL0FBDIV to value 0"]
impl crate::Resettable for Pll0fbdivSpec {}
