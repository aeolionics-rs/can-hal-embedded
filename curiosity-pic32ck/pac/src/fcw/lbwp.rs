#[doc = "Register `LBWP` reader"]
pub type R = crate::R<LbwpSpec>;
#[doc = "Register `LBWP` writer"]
pub type W = crate::W<LbwpSpec>;
#[doc = "Field `LBWP` reader - Lower Boot Pages Write Protect Bits"]
pub type LbwpR = crate::FieldReader<u16>;
#[doc = "Field `LBWP` writer - Lower Boot Pages Write Protect Bits"]
pub type LbwpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LBWPLOCK` reader - LBWP Lock Bit"]
pub type LbwplockR = crate::BitReader;
#[doc = "Field `LBWPLOCK` writer - LBWP Lock Bit"]
pub type LbwplockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Lower Boot Pages Write Protect Bits"]
    #[inline(always)]
    pub fn lbwp(&self) -> LbwpR {
        LbwpR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - LBWP Lock Bit"]
    #[inline(always)]
    pub fn lbwplock(&self) -> LbwplockR {
        LbwplockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower Boot Pages Write Protect Bits"]
    #[inline(always)]
    pub fn lbwp(&mut self) -> LbwpW<'_, LbwpSpec> {
        LbwpW::new(self, 0)
    }
    #[doc = "Bit 31 - LBWP Lock Bit"]
    #[inline(always)]
    pub fn lbwplock(&mut self) -> LbwplockW<'_, LbwpSpec> {
        LbwplockW::new(self, 31)
    }
}
#[doc = "Lower BFM Write Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lbwp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lbwp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LbwpSpec;
impl crate::RegisterSpec for LbwpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lbwp::R`](R) reader structure"]
impl crate::Readable for LbwpSpec {}
#[doc = "`write(|w| ..)` method takes [`lbwp::W`](W) writer structure"]
impl crate::Writable for LbwpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LBWP to value 0xffff"]
impl crate::Resettable for LbwpSpec {
    const RESET_VALUE: u32 = 0xffff;
}
