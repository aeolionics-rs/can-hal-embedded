#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - USBHS Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - USBHS Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - USBHS Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - USBHS Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDVAL` reader - Override Valud of ID"]
pub type IdvalR = crate::BitReader;
#[doc = "Field `IDVAL` writer - Override Valud of ID"]
pub type IdvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDOVEN` reader - ID Source Select"]
pub type IdovenR = crate::BitReader;
#[doc = "Field `IDOVEN` writer - ID Source Select"]
pub type IdovenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFCLKSEL` reader - Select USB PLL Reference Clock Speed"]
pub type RefclkselR = crate::BitReader;
#[doc = "Field `REFCLKSEL` writer - Select USB PLL Reference Clock Speed"]
pub type RefclkselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USBHS Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USBHS Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Override Valud of ID"]
    #[inline(always)]
    pub fn idval(&self) -> IdvalR {
        IdvalR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ID Source Select"]
    #[inline(always)]
    pub fn idoven(&self) -> IdovenR {
        IdovenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Select USB PLL Reference Clock Speed"]
    #[inline(always)]
    pub fn refclksel(&self) -> RefclkselR {
        RefclkselR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USBHS Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - USBHS Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 8 - Override Valud of ID"]
    #[inline(always)]
    pub fn idval(&mut self) -> IdvalW<'_, CtrlaSpec> {
        IdvalW::new(self, 8)
    }
    #[doc = "Bit 9 - ID Source Select"]
    #[inline(always)]
    pub fn idoven(&mut self) -> IdovenW<'_, CtrlaSpec> {
        IdovenW::new(self, 9)
    }
    #[doc = "Bit 10 - Select USB PLL Reference Clock Speed"]
    #[inline(always)]
    pub fn refclksel(&mut self) -> RefclkselW<'_, CtrlaSpec> {
        RefclkselW::new(self, 10)
    }
}
#[doc = "USBHS Control Register A. (All bits except ENABLE and SWRST are Enable protected)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
