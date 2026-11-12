#[doc = "Register `XOSCCTRLB` reader"]
pub type R = crate::R<XoscctrlbSpec>;
#[doc = "Register `XOSCCTRLB` writer"]
pub type W = crate::W<XoscctrlbSpec>;
#[doc = "Field `USRCFG` reader - User Configuration Control Bits"]
pub type UsrcfgR = crate::FieldReader;
#[doc = "Field `USRCFG` writer - User Configuration Control Bits"]
pub type UsrcfgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRTLOCK` reader - Write Lock for CTRLB register"]
pub type WrtlockR = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock for CTRLB register"]
pub type WrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - User Configuration Control Bits"]
    #[inline(always)]
    pub fn usrcfg(&self) -> UsrcfgR {
        UsrcfgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Write Lock for CTRLB register"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WrtlockR {
        WrtlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - User Configuration Control Bits"]
    #[inline(always)]
    pub fn usrcfg(&mut self) -> UsrcfgW<'_, XoscctrlbSpec> {
        UsrcfgW::new(self, 0)
    }
    #[doc = "Bit 31 - Write Lock for CTRLB register"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WrtlockW<'_, XoscctrlbSpec> {
        WrtlockW::new(self, 31)
    }
}
#[doc = "External Multipurpose Crystal Oscillator Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`xoscctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xoscctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XoscctrlbSpec;
impl crate::RegisterSpec for XoscctrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xoscctrlb::R`](R) reader structure"]
impl crate::Readable for XoscctrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`xoscctrlb::W`](W) writer structure"]
impl crate::Writable for XoscctrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XOSCCTRLB to value 0"]
impl crate::Resettable for XoscctrlbSpec {}
