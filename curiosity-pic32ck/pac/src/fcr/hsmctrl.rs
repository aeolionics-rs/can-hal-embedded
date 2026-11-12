#[doc = "Register `HSMCTRL` reader"]
pub type R = crate::R<HsmctrlSpec>;
#[doc = "Register `HSMCTRL` writer"]
pub type W = crate::W<HsmctrlSpec>;
#[doc = "Field `ECCDIS` reader - ECC Disable for HSM Pages in CFM"]
pub type EccdisR = crate::BitReader;
#[doc = "Field `ECCDIS` writer - ECC Disable for HSM Pages in CFM"]
pub type EccdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYZ` reader - Flash Key Locations read"]
pub type KeyzR = crate::BitReader;
#[doc = "Field `KEYZ` writer - Flash Key Locations read"]
pub type KeyzW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ECC Disable for HSM Pages in CFM"]
    #[inline(always)]
    pub fn eccdis(&self) -> EccdisR {
        EccdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Key Locations read"]
    #[inline(always)]
    pub fn keyz(&self) -> KeyzR {
        KeyzR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC Disable for HSM Pages in CFM"]
    #[inline(always)]
    pub fn eccdis(&mut self) -> EccdisW<'_, HsmctrlSpec> {
        EccdisW::new(self, 0)
    }
    #[doc = "Bit 1 - Flash Key Locations read"]
    #[inline(always)]
    pub fn keyz(&mut self) -> KeyzW<'_, HsmctrlSpec> {
        KeyzW::new(self, 1)
    }
}
#[doc = "HSM CFM PAGES Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsmctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsmctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsmctrlSpec;
impl crate::RegisterSpec for HsmctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsmctrl::R`](R) reader structure"]
impl crate::Readable for HsmctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hsmctrl::W`](W) writer structure"]
impl crate::Writable for HsmctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSMCTRL to value 0"]
impl crate::Resettable for HsmctrlSpec {}
