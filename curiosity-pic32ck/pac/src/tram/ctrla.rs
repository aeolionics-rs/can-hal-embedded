#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPERS` reader - Tamper Erase"]
pub type TampersR = crate::BitReader;
#[doc = "Field `TAMPERS` writer - Tamper Erase"]
pub type TampersW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRP` reader - Data Remanence Prevention"]
pub type DrpR = crate::BitReader;
#[doc = "Field `DRP` writer - Data Remanence Prevention"]
pub type DrpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SILACC` reader - Silent Access"]
pub type SilaccR = crate::BitReader;
#[doc = "Field `SILACC` writer - Silent Access"]
pub type SilaccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper Erase"]
    #[inline(always)]
    pub fn tampers(&self) -> TampersR {
        TampersR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Data Remanence Prevention"]
    #[inline(always)]
    pub fn drp(&self) -> DrpR {
        DrpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Silent Access"]
    #[inline(always)]
    pub fn silacc(&self) -> SilaccR {
        SilaccR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 4 - Tamper Erase"]
    #[inline(always)]
    pub fn tampers(&mut self) -> TampersW<'_, CtrlaSpec> {
        TampersW::new(self, 4)
    }
    #[doc = "Bit 6 - Data Remanence Prevention"]
    #[inline(always)]
    pub fn drp(&mut self) -> DrpW<'_, CtrlaSpec> {
        DrpW::new(self, 6)
    }
    #[doc = "Bit 7 - Silent Access"]
    #[inline(always)]
    pub fn silacc(&mut self) -> SilaccW<'_, CtrlaSpec> {
        SilaccW::new(self, 7)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {}
