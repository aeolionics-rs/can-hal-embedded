#[doc = "Register `PHY4C` reader"]
pub type R = crate::R<Phy4cSpec>;
#[doc = "Register `PHY4C` writer"]
pub type W = crate::W<Phy4cSpec>;
#[doc = "Field `VBUSVALTUNE` reader - VBus_valid reference tuning"]
pub type VbusvaltuneR = crate::FieldReader;
#[doc = "Field `VBUSVALTUNE` writer - VBus_valid reference tuning"]
pub type VbusvaltuneW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BSESSVALIDTUNE10` reader - Lower 2 B_sessionvalid reference tune"]
pub type Bsessvalidtune10R = crate::FieldReader;
#[doc = "Field `BSESSVALIDTUNE10` writer - Lower 2 B_sessionvalid reference tune"]
pub type Bsessvalidtune10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - VBus_valid reference tuning"]
    #[inline(always)]
    pub fn vbusvaltune(&self) -> VbusvaltuneR {
        VbusvaltuneR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:7 - Lower 2 B_sessionvalid reference tune"]
    #[inline(always)]
    pub fn bsessvalidtune10(&self) -> Bsessvalidtune10R {
        Bsessvalidtune10R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - VBus_valid reference tuning"]
    #[inline(always)]
    pub fn vbusvaltune(&mut self) -> VbusvaltuneW<'_, Phy4cSpec> {
        VbusvaltuneW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Lower 2 B_sessionvalid reference tune"]
    #[inline(always)]
    pub fn bsessvalidtune10(&mut self) -> Bsessvalidtune10W<'_, Phy4cSpec> {
        Bsessvalidtune10W::new(self, 6)
    }
}
#[doc = "USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy4c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy4c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phy4cSpec;
impl crate::RegisterSpec for Phy4cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy4c::R`](R) reader structure"]
impl crate::Readable for Phy4cSpec {}
#[doc = "`write(|w| ..)` method takes [`phy4c::W`](W) writer structure"]
impl crate::Writable for Phy4cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY4C to value 0"]
impl crate::Resettable for Phy4cSpec {}
