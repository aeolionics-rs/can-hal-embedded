#[doc = "Register `UBWP` reader"]
pub type R = crate::R<UbwpSpec>;
#[doc = "Register `UBWP` writer"]
pub type W = crate::W<UbwpSpec>;
#[doc = "Field `UBWP` reader - Upper Boot Pages Write Protect Bits"]
pub type UbwpR = crate::FieldReader<u16>;
#[doc = "Field `UBWP` writer - Upper Boot Pages Write Protect Bits"]
pub type UbwpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UBWPLOCK` reader - UBWP Register Lock Bit"]
pub type UbwplockR = crate::BitReader;
#[doc = "Field `UBWPLOCK` writer - UBWP Register Lock Bit"]
pub type UbwplockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Upper Boot Pages Write Protect Bits"]
    #[inline(always)]
    pub fn ubwp(&self) -> UbwpR {
        UbwpR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - UBWP Register Lock Bit"]
    #[inline(always)]
    pub fn ubwplock(&self) -> UbwplockR {
        UbwplockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Upper Boot Pages Write Protect Bits"]
    #[inline(always)]
    pub fn ubwp(&mut self) -> UbwpW<'_, UbwpSpec> {
        UbwpW::new(self, 0)
    }
    #[doc = "Bit 31 - UBWP Register Lock Bit"]
    #[inline(always)]
    pub fn ubwplock(&mut self) -> UbwplockW<'_, UbwpSpec> {
        UbwplockW::new(self, 31)
    }
}
#[doc = "Upper BFM Write Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ubwp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ubwp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UbwpSpec;
impl crate::RegisterSpec for UbwpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ubwp::R`](R) reader structure"]
impl crate::Readable for UbwpSpec {}
#[doc = "`write(|w| ..)` method takes [`ubwp::W`](W) writer structure"]
impl crate::Writable for UbwpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UBWP to value 0xffff"]
impl crate::Resettable for UbwpSpec {
    const RESET_VALUE: u32 = 0xffff;
}
