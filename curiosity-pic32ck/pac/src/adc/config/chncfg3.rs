#[doc = "Register `CHNCFG3` reader"]
pub type R = crate::R<Chncfg3Spec>;
#[doc = "Register `CHNCFG3` writer"]
pub type W = crate::W<Chncfg3Spec>;
#[doc = "Field `DIFF0` reader - Differential Mode enable for ADC module 0, analog input AIN0"]
pub type Diff0R = crate::BitReader;
#[doc = "Field `DIFF0` writer - Differential Mode enable for ADC module 0, analog input AIN0"]
pub type Diff0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF2` reader - Differential Mode enable for ADC module 0, analog input AIN2"]
pub type Diff2R = crate::BitReader;
#[doc = "Field `DIFF2` writer - Differential Mode enable for ADC module 0, analog input AIN2"]
pub type Diff2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF4` reader - Differential Mode enable for ADC module 0, analog input AIN4"]
pub type Diff4R = crate::BitReader;
#[doc = "Field `DIFF4` writer - Differential Mode enable for ADC module 0, analog input AIN4"]
pub type Diff4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIGN` reader - SIGN setting"]
pub type SignR = crate::FieldReader<u16>;
#[doc = "Field `SIGN` writer - SIGN setting"]
pub type SignW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bit 0 - Differential Mode enable for ADC module 0, analog input AIN0"]
    #[inline(always)]
    pub fn diff0(&self) -> Diff0R {
        Diff0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Differential Mode enable for ADC module 0, analog input AIN2"]
    #[inline(always)]
    pub fn diff2(&self) -> Diff2R {
        Diff2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Differential Mode enable for ADC module 0, analog input AIN4"]
    #[inline(always)]
    pub fn diff4(&self) -> Diff4R {
        Diff4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:29 - SIGN setting"]
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Differential Mode enable for ADC module 0, analog input AIN0"]
    #[inline(always)]
    pub fn diff0(&mut self) -> Diff0W<'_, Chncfg3Spec> {
        Diff0W::new(self, 0)
    }
    #[doc = "Bit 2 - Differential Mode enable for ADC module 0, analog input AIN2"]
    #[inline(always)]
    pub fn diff2(&mut self) -> Diff2W<'_, Chncfg3Spec> {
        Diff2W::new(self, 2)
    }
    #[doc = "Bit 4 - Differential Mode enable for ADC module 0, analog input AIN4"]
    #[inline(always)]
    pub fn diff4(&mut self) -> Diff4W<'_, Chncfg3Spec> {
        Diff4W::new(self, 4)
    }
    #[doc = "Bits 16:29 - SIGN setting"]
    #[inline(always)]
    pub fn sign(&mut self) -> SignW<'_, Chncfg3Spec> {
        SignW::new(self, 16)
    }
}
#[doc = "Channel Configuration3 (SIGN/DIFF)\n\nYou can [`read`](crate::Reg::read) this register and get [`chncfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chncfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chncfg3Spec;
impl crate::RegisterSpec for Chncfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chncfg3::R`](R) reader structure"]
impl crate::Readable for Chncfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`chncfg3::W`](W) writer structure"]
impl crate::Writable for Chncfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNCFG3 to value 0"]
impl crate::Resettable for Chncfg3Spec {}
