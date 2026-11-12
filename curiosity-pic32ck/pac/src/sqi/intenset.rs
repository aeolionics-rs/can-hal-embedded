#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "SQI Interrupt Enable Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sqiselect {
    #[doc = "0: Interrupt disabled"]
    Low = 0,
    #[doc = "1: Interrupt enabled"]
    High = 1,
}
impl From<Sqiselect> for bool {
    #[inline(always)]
    fn from(variant: Sqiselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SQI` reader - SQI Interrupt Enable Set"]
pub type SqiR = crate::BitReader<Sqiselect>;
impl SqiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sqiselect {
        match self.bits {
            false => Sqiselect::Low,
            true => Sqiselect::High,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sqiselect::Low
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sqiselect::High
    }
}
#[doc = "Field `SQI` writer - SQI Interrupt Enable Set"]
pub type SqiW<'a, REG> = crate::BitWriter<'a, REG, Sqiselect>;
impl<'a, REG> SqiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sqiselect::Low)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sqiselect::High)
    }
}
impl R {
    #[doc = "Bit 0 - SQI Interrupt Enable Set"]
    #[inline(always)]
    pub fn sqi(&self) -> SqiR {
        SqiR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SQI Interrupt Enable Set"]
    #[inline(always)]
    pub fn sqi(&mut self) -> SqiW<'_, IntensetSpec> {
        SqiW::new(self, 0)
    }
}
#[doc = "Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
