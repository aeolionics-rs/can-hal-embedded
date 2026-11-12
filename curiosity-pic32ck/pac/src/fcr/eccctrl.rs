#[doc = "Register `ECCCTRL` reader"]
pub type R = crate::R<EccctrlSpec>;
#[doc = "Register `ECCCTRL` writer"]
pub type W = crate::W<EccctrlSpec>;
#[doc = "Field `ECCCTL` reader - NVM ECC Mode Control- restricts one or more NVMOPs"]
pub type EccctlR = crate::FieldReader;
#[doc = "Field `ECCCTL` writer - NVM ECC Mode Control- restricts one or more NVMOPs"]
pub type EccctlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECCUNLCK` reader - NVM ECC Mode Control Unlock"]
pub type EccunlckR = crate::BitReader;
#[doc = "Field `ECCUNLCK` writer - NVM ECC Mode Control Unlock"]
pub type EccunlckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECCNT` reader - Flash SEC Count"]
pub type SeccntR = crate::FieldReader;
#[doc = "Field `SECCNT` writer - Flash SEC Count"]
pub type SeccntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 4:5 - NVM ECC Mode Control- restricts one or more NVMOPs"]
    #[inline(always)]
    pub fn eccctl(&self) -> EccctlR {
        EccctlR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - NVM ECC Mode Control Unlock"]
    #[inline(always)]
    pub fn eccunlck(&self) -> EccunlckR {
        EccunlckR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Flash SEC Count"]
    #[inline(always)]
    pub fn seccnt(&self) -> SeccntR {
        SeccntR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - NVM ECC Mode Control- restricts one or more NVMOPs"]
    #[inline(always)]
    pub fn eccctl(&mut self) -> EccctlW<'_, EccctrlSpec> {
        EccctlW::new(self, 4)
    }
    #[doc = "Bit 6 - NVM ECC Mode Control Unlock"]
    #[inline(always)]
    pub fn eccunlck(&mut self) -> EccunlckW<'_, EccctrlSpec> {
        EccunlckW::new(self, 6)
    }
    #[doc = "Bits 8:15 - Flash SEC Count"]
    #[inline(always)]
    pub fn seccnt(&mut self) -> SeccntW<'_, EccctrlSpec> {
        SeccntW::new(self, 8)
    }
}
#[doc = "ECC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccctrlSpec;
impl crate::RegisterSpec for EccctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccctrl::R`](R) reader structure"]
impl crate::Readable for EccctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`eccctrl::W`](W) writer structure"]
impl crate::Writable for EccctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECCCTRL to value 0x40"]
impl crate::Resettable for EccctrlSpec {
    const RESET_VALUE: u32 = 0x40;
}
