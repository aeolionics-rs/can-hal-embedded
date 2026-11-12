#[doc = "Register `DACCTRL` reader"]
pub type R = crate::R<DacctrlSpec>;
#[doc = "Register `DACCTRL` writer"]
pub type W = crate::W<DacctrlSpec>;
#[doc = "Field `VALUE0` reader - DAC0 Output Value"]
pub type Value0R = crate::FieldReader;
#[doc = "Field `VALUE0` writer - DAC0 Output Value"]
pub type Value0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SHEN0` reader - DAC0 Sample and Hold Enable Operating Mode"]
pub type Shen0R = crate::BitReader;
#[doc = "Field `SHEN0` writer - DAC0 Sample and Hold Enable Operating Mode"]
pub type Shen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VALUE1` reader - DAC1 Output Value"]
pub type Value1R = crate::FieldReader;
#[doc = "Field `VALUE1` writer - DAC1 Output Value"]
pub type Value1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SHEN1` reader - DAC1 Sample and Hold Enable Operating Mode"]
pub type Shen1R = crate::BitReader;
#[doc = "Field `SHEN1` writer - DAC1 Sample and Hold Enable Operating Mode"]
pub type Shen1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - DAC0 Output Value"]
    #[inline(always)]
    pub fn value0(&self) -> Value0R {
        Value0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 15 - DAC0 Sample and Hold Enable Operating Mode"]
    #[inline(always)]
    pub fn shen0(&self) -> Shen0R {
        Shen0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - DAC1 Output Value"]
    #[inline(always)]
    pub fn value1(&self) -> Value1R {
        Value1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - DAC1 Sample and Hold Enable Operating Mode"]
    #[inline(always)]
    pub fn shen1(&self) -> Shen1R {
        Shen1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - DAC0 Output Value"]
    #[inline(always)]
    pub fn value0(&mut self) -> Value0W<'_, DacctrlSpec> {
        Value0W::new(self, 0)
    }
    #[doc = "Bit 15 - DAC0 Sample and Hold Enable Operating Mode"]
    #[inline(always)]
    pub fn shen0(&mut self) -> Shen0W<'_, DacctrlSpec> {
        Shen0W::new(self, 15)
    }
    #[doc = "Bits 16:22 - DAC1 Output Value"]
    #[inline(always)]
    pub fn value1(&mut self) -> Value1W<'_, DacctrlSpec> {
        Value1W::new(self, 16)
    }
    #[doc = "Bit 31 - DAC1 Sample and Hold Enable Operating Mode"]
    #[inline(always)]
    pub fn shen1(&mut self) -> Shen1W<'_, DacctrlSpec> {
        Shen1W::new(self, 31)
    }
}
#[doc = "Dac Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dacctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacctrlSpec;
impl crate::RegisterSpec for DacctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dacctrl::R`](R) reader structure"]
impl crate::Readable for DacctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dacctrl::W`](W) writer structure"]
impl crate::Writable for DacctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DACCTRL to value 0"]
impl crate::Resettable for DacctrlSpec {}
