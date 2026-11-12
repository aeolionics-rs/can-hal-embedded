#[doc = "Register `STDBYCFG` reader"]
pub type R = crate::R<StdbycfgSpec>;
#[doc = "Register `STDBYCFG` writer"]
pub type W = crate::W<StdbycfgSpec>;
#[doc = "Ram Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramcfgselect {
    #[doc = "0: All the RAMs are retained"]
    Ret = 0,
    #[doc = "1: Only the first 32K bytes are retained"]
    Off = 1,
}
impl From<Ramcfgselect> for bool {
    #[inline(always)]
    fn from(variant: Ramcfgselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMCFG` reader - Ram Configuration"]
pub type RamcfgR = crate::BitReader<Ramcfgselect>;
impl RamcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramcfgselect {
        match self.bits {
            false => Ramcfgselect::Ret,
            true => Ramcfgselect::Off,
        }
    }
    #[doc = "All the RAMs are retained"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        *self == Ramcfgselect::Ret
    }
    #[doc = "Only the first 32K bytes are retained"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ramcfgselect::Off
    }
}
#[doc = "Field `RAMCFG` writer - Ram Configuration"]
pub type RamcfgW<'a, REG> = crate::BitWriter<'a, REG, Ramcfgselect>;
impl<'a, REG> RamcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All the RAMs are retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut crate::W<REG> {
        self.variant(Ramcfgselect::Ret)
    }
    #[doc = "Only the first 32K bytes are retained"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Ramcfgselect::Off)
    }
}
#[doc = "Field `LPRAM` reader - Low Power RAM Enable"]
pub type LpramR = crate::BitReader;
#[doc = "Field `LPRAM` writer - Low Power RAM Enable"]
pub type LpramW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Ram Configuration"]
    #[inline(always)]
    pub fn ramcfg(&self) -> RamcfgR {
        RamcfgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Power RAM Enable"]
    #[inline(always)]
    pub fn lpram(&self) -> LpramR {
        LpramR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ram Configuration"]
    #[inline(always)]
    pub fn ramcfg(&mut self) -> RamcfgW<'_, StdbycfgSpec> {
        RamcfgW::new(self, 0)
    }
    #[doc = "Bit 2 - Low Power RAM Enable"]
    #[inline(always)]
    pub fn lpram(&mut self) -> LpramW<'_, StdbycfgSpec> {
        LpramW::new(self, 2)
    }
}
#[doc = "Standby Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`stdbycfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stdbycfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StdbycfgSpec;
impl crate::RegisterSpec for StdbycfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`stdbycfg::R`](R) reader structure"]
impl crate::Readable for StdbycfgSpec {}
#[doc = "`write(|w| ..)` method takes [`stdbycfg::W`](W) writer structure"]
impl crate::Writable for StdbycfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STDBYCFG to value 0x04"]
impl crate::Resettable for StdbycfgSpec {
    const RESET_VALUE: u8 = 0x04;
}
