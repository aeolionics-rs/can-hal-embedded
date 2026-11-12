#[doc = "Register `PHY1C` reader"]
pub type R = crate::R<Phy1cSpec>;
#[doc = "Register `PHY1C` writer"]
pub type W = crate::W<Phy1cSpec>;
#[doc = "Field `ODTBYPASS` reader - ODT auto-refresh bypass"]
pub type OdtbypassR = crate::BitReader;
#[doc = "Field `ODTBYPASS` writer - ODT auto-refresh bypass"]
pub type OdtbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSLSDIFF` reader - Turn off FS/LS differential receiver in suspend mode"]
pub type FslsdiffR = crate::BitReader;
#[doc = "Field `FSLSDIFF` writer - Turn off FS/LS differential receiver in suspend mode"]
pub type FslsdiffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - ODT auto-refresh bypass"]
    #[inline(always)]
    pub fn odtbypass(&self) -> OdtbypassR {
        OdtbypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Turn off FS/LS differential receiver in suspend mode"]
    #[inline(always)]
    pub fn fslsdiff(&self) -> FslsdiffR {
        FslsdiffR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ODT auto-refresh bypass"]
    #[inline(always)]
    pub fn odtbypass(&mut self) -> OdtbypassW<'_, Phy1cSpec> {
        OdtbypassW::new(self, 1)
    }
    #[doc = "Bit 7 - Turn off FS/LS differential receiver in suspend mode"]
    #[inline(always)]
    pub fn fslsdiff(&mut self) -> FslsdiffW<'_, Phy1cSpec> {
        FslsdiffW::new(self, 7)
    }
}
#[doc = "USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phy1cSpec;
impl crate::RegisterSpec for Phy1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy1c::R`](R) reader structure"]
impl crate::Readable for Phy1cSpec {}
#[doc = "`write(|w| ..)` method takes [`phy1c::W`](W) writer structure"]
impl crate::Writable for Phy1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY1C to value 0x82"]
impl crate::Resettable for Phy1cSpec {
    const RESET_VALUE: u32 = 0x82;
}
