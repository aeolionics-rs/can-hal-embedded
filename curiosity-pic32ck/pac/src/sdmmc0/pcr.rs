#[doc = "Register `PCR` reader"]
pub type R = crate::R<PcrSpec>;
#[doc = "Register `PCR` writer"]
pub type W = crate::W<PcrSpec>;
#[doc = "SD Bus Power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdbpwrselect {
    #[doc = "0: Power off"]
    Off = 0,
    #[doc = "1: Power on"]
    On = 1,
}
impl From<Sdbpwrselect> for bool {
    #[inline(always)]
    fn from(variant: Sdbpwrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDBPWR` reader - SD Bus Power"]
pub type SdbpwrR = crate::BitReader<Sdbpwrselect>;
impl SdbpwrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdbpwrselect {
        match self.bits {
            false => Sdbpwrselect::Off,
            true => Sdbpwrselect::On,
        }
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Sdbpwrselect::Off
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Sdbpwrselect::On
    }
}
#[doc = "Field `SDBPWR` writer - SD Bus Power"]
pub type SdbpwrW<'a, REG> = crate::BitWriter<'a, REG, Sdbpwrselect>;
impl<'a, REG> SdbpwrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbpwrselect::Off)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbpwrselect::On)
    }
}
impl R {
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    pub fn sdbpwr(&self) -> SdbpwrR {
        SdbpwrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    pub fn sdbpwr(&mut self) -> SdbpwrW<'_, PcrSpec> {
        SdbpwrW::new(self, 0)
    }
}
#[doc = "Power Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcrSpec;
impl crate::RegisterSpec for PcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pcr::R`](R) reader structure"]
impl crate::Readable for PcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pcr::W`](W) writer structure"]
impl crate::Writable for PcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCR to value 0x0e"]
impl crate::Resettable for PcrSpec {
    const RESET_VALUE: u8 = 0x0e;
}
