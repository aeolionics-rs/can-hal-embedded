#[doc = "Register `FFLTCTRL` reader"]
pub type R = crate::R<FfltctrlSpec>;
#[doc = "Register `FFLTCTRL` writer"]
pub type W = crate::W<FfltctrlSpec>;
#[doc = "Field `FLTRST` reader - Fault Reset"]
pub type FltrstR = crate::BitReader;
#[doc = "Field `FLTRST` writer - Fault Reset"]
pub type FltrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTEN` reader - ECC Fault Enable bit"]
pub type FltenR = crate::BitReader;
#[doc = "Field `FLTEN` writer - ECC Fault Enable bit"]
pub type FltenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTLFLT` reader - ECC/Parity Control Fault bits"]
pub type CtlfltR = crate::FieldReader;
#[doc = "Field `CTLFLT` writer - ECC/Parity Control Fault bits"]
pub type CtlfltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLTMD` reader - Fault Mode Control"]
pub type FltmdR = crate::FieldReader;
#[doc = "Field `FLTMD` writer - Fault Mode Control"]
pub type FltmdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Fault Reset"]
    #[inline(always)]
    pub fn fltrst(&self) -> FltrstR {
        FltrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC Fault Enable bit"]
    #[inline(always)]
    pub fn flten(&self) -> FltenR {
        FltenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:10 - ECC/Parity Control Fault bits"]
    #[inline(always)]
    pub fn ctlflt(&self) -> CtlfltR {
        CtlfltR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Fault Mode Control"]
    #[inline(always)]
    pub fn fltmd(&self) -> FltmdR {
        FltmdR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Reset"]
    #[inline(always)]
    pub fn fltrst(&mut self) -> FltrstW<'_, FfltctrlSpec> {
        FltrstW::new(self, 0)
    }
    #[doc = "Bit 1 - ECC Fault Enable bit"]
    #[inline(always)]
    pub fn flten(&mut self) -> FltenW<'_, FfltctrlSpec> {
        FltenW::new(self, 1)
    }
    #[doc = "Bits 8:10 - ECC/Parity Control Fault bits"]
    #[inline(always)]
    pub fn ctlflt(&mut self) -> CtlfltW<'_, FfltctrlSpec> {
        CtlfltW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Fault Mode Control"]
    #[inline(always)]
    pub fn fltmd(&mut self) -> FltmdW<'_, FfltctrlSpec> {
        FltmdW::new(self, 12)
    }
}
#[doc = "Flash ECC Fault Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffltctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfltctrlSpec;
impl crate::RegisterSpec for FfltctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffltctrl::R`](R) reader structure"]
impl crate::Readable for FfltctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ffltctrl::W`](W) writer structure"]
impl crate::Writable for FfltctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FFLTCTRL to value 0"]
impl crate::Resettable for FfltctrlSpec {}
