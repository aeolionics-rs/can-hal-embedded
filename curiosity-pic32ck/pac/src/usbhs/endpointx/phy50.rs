#[doc = "Register `PHY50` reader"]
pub type R = crate::R<Phy50Spec>;
#[doc = "Register `PHY50` writer"]
pub type W = crate::W<Phy50Spec>;
#[doc = "Field `BSESSVALIDTUNE2` reader - MSB of B_sessionvalid reference tune"]
pub type Bsessvalidtune2R = crate::BitReader;
#[doc = "Field `BSESSVALIDTUNE2` writer - MSB of B_sessionvalid reference tune"]
pub type Bsessvalidtune2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASESSVALIDTUNE` reader - A_sessionvalid reference tune"]
pub type AsessvalidtuneR = crate::FieldReader;
#[doc = "Field `ASESSVALIDTUNE` writer - A_sessionvalid reference tune"]
pub type AsessvalidtuneW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMPCURREF` reader - Compensation current tuning reference"]
pub type CompcurrefR = crate::FieldReader;
#[doc = "Field `COMPCURREF` writer - Compensation current tuning reference"]
pub type CompcurrefW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - MSB of B_sessionvalid reference tune"]
    #[inline(always)]
    pub fn bsessvalidtune2(&self) -> Bsessvalidtune2R {
        Bsessvalidtune2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - A_sessionvalid reference tune"]
    #[inline(always)]
    pub fn asessvalidtune(&self) -> AsessvalidtuneR {
        AsessvalidtuneR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 5:7 - Compensation current tuning reference"]
    #[inline(always)]
    pub fn compcurref(&self) -> CompcurrefR {
        CompcurrefR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MSB of B_sessionvalid reference tune"]
    #[inline(always)]
    pub fn bsessvalidtune2(&mut self) -> Bsessvalidtune2W<'_, Phy50Spec> {
        Bsessvalidtune2W::new(self, 0)
    }
    #[doc = "Bits 1:3 - A_sessionvalid reference tune"]
    #[inline(always)]
    pub fn asessvalidtune(&mut self) -> AsessvalidtuneW<'_, Phy50Spec> {
        AsessvalidtuneW::new(self, 1)
    }
    #[doc = "Bits 5:7 - Compensation current tuning reference"]
    #[inline(always)]
    pub fn compcurref(&mut self) -> CompcurrefW<'_, Phy50Spec> {
        CompcurrefW::new(self, 5)
    }
}
#[doc = "USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phy50Spec;
impl crate::RegisterSpec for Phy50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy50::R`](R) reader structure"]
impl crate::Readable for Phy50Spec {}
#[doc = "`write(|w| ..)` method takes [`phy50::W`](W) writer structure"]
impl crate::Writable for Phy50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY50 to value 0"]
impl crate::Resettable for Phy50Spec {}
