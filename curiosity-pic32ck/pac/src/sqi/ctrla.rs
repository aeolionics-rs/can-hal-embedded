#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Run in Standby\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Runstdbyselect {
    #[doc = "0: Module is disabled in Standby Sleep mode"]
    Low = 0,
    #[doc = "1: Module continues to run in Standby Sleep mode"]
    High = 1,
}
impl From<Runstdbyselect> for bool {
    #[inline(always)]
    fn from(variant: Runstdbyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader<Runstdbyselect>;
impl RunstdbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Runstdbyselect {
        match self.bits {
            false => Runstdbyselect::Low,
            true => Runstdbyselect::High,
        }
    }
    #[doc = "Module is disabled in Standby Sleep mode"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Runstdbyselect::Low
    }
    #[doc = "Module continues to run in Standby Sleep mode"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Runstdbyselect::High
    }
}
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter1S<'a, REG, Runstdbyselect>;
impl<'a, REG> RunstdbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Module is disabled in Standby Sleep mode"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Runstdbyselect::Low)
    }
    #[doc = "Module continues to run in Standby Sleep mode"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Runstdbyselect::High)
    }
}
impl R {
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, CtrlaSpec> {
        RunstdbyW::new(self, 6)
    }
}
#[doc = "Control A register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x40;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {}
