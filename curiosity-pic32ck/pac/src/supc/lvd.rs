#[doc = "Register `LVD` reader"]
pub type R = crate::R<LvdSpec>;
#[doc = "Register `LVD` writer"]
pub type W = crate::W<LvdSpec>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dirselect {
    #[doc = "0: Rising detection"]
    Rising = 0,
    #[doc = "1: Falling detection"]
    Falling = 1,
}
impl From<Dirselect> for bool {
    #[inline(always)]
    fn from(variant: Dirselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Direction"]
pub type DirR = crate::BitReader<Dirselect>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dirselect {
        match self.bits {
            false => Dirselect::Rising,
            true => Dirselect::Falling,
        }
    }
    #[doc = "Rising detection"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Dirselect::Rising
    }
    #[doc = "Falling detection"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Dirselect::Falling
    }
}
#[doc = "Field `DIR` writer - Direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dirselect>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising detection"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Dirselect::Rising)
    }
    #[doc = "Falling detection"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Dirselect::Falling)
    }
}
#[doc = "Field `OEVEN` reader - Output Event Enable"]
pub type OevenR = crate::BitReader;
#[doc = "Field `OEVEN` writer - Output Event Enable"]
pub type OevenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEVEL` reader - Threshold Level. See 'pwr_smor_\\[nn\\]_v1 DOS' - level section to get details."]
pub type LevelR = crate::FieldReader;
#[doc = "Field `LEVEL` writer - Threshold Level. See 'pwr_smor_\\[nn\\]_v1 DOS' - level section to get details."]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Event Enable"]
    #[inline(always)]
    pub fn oeven(&self) -> OevenR {
        OevenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Threshold Level. See 'pwr_smor_\\[nn\\]_v1 DOS' - level section to get details."]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, LvdSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, LvdSpec> {
        DirW::new(self, 2)
    }
    #[doc = "Bit 3 - Output Event Enable"]
    #[inline(always)]
    pub fn oeven(&mut self) -> OevenW<'_, LvdSpec> {
        OevenW::new(self, 3)
    }
    #[doc = "Bit 4 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, LvdSpec> {
        RunstdbyW::new(self, 4)
    }
    #[doc = "Bits 16:19 - Threshold Level. See 'pwr_smor_\\[nn\\]_v1 DOS' - level section to get details."]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<'_, LvdSpec> {
        LevelW::new(self, 16)
    }
}
#[doc = "LVD Control\n\nYou can [`read`](crate::Reg::read) this register and get [`lvd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvdSpec;
impl crate::RegisterSpec for LvdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lvd::R`](R) reader structure"]
impl crate::Readable for LvdSpec {}
#[doc = "`write(|w| ..)` method takes [`lvd::W`](W) writer structure"]
impl crate::Writable for LvdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LVD to value 0"]
impl crate::Resettable for LvdSpec {}
