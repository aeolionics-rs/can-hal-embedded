#[doc = "Register `TYPE0` reader"]
pub type R = crate::R<Type0Spec>;
#[doc = "Register `TYPE0` writer"]
pub type W = crate::W<Type0Spec>;
#[doc = "Operating Speed Of The Target Device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Speedselect {
    #[doc = "1: Device In High Speed"]
    High = 1,
    #[doc = "2: Device In Full Speed"]
    Full = 2,
    #[doc = "3: Device In Low Speed"]
    Low = 3,
}
impl From<Speedselect> for u8 {
    #[inline(always)]
    fn from(variant: Speedselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Speedselect {
    type Ux = u8;
}
impl crate::IsEnum for Speedselect {}
#[doc = "Field `SPEED` reader - Operating Speed Of The Target Device"]
pub type SpeedR = crate::FieldReader<Speedselect>;
impl SpeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Speedselect> {
        match self.bits {
            1 => Some(Speedselect::High),
            2 => Some(Speedselect::Full),
            3 => Some(Speedselect::Low),
            _ => None,
        }
    }
    #[doc = "Device In High Speed"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Speedselect::High
    }
    #[doc = "Device In Full Speed"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Speedselect::Full
    }
    #[doc = "Device In Low Speed"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Speedselect::Low
    }
}
#[doc = "Field `SPEED` writer - Operating Speed Of The Target Device"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2, Speedselect>;
impl<'a, REG> SpeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Device In High Speed"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Speedselect::High)
    }
    #[doc = "Device In Full Speed"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Speedselect::Full)
    }
    #[doc = "Device In Low Speed"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Speedselect::Low)
    }
}
impl R {
    #[doc = "Bits 6:7 - Operating Speed Of The Target Device"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 6:7 - Operating Speed Of The Target Device"]
    #[inline(always)]
    pub fn speed(&mut self) -> SpeedW<'_, Type0Spec> {
        SpeedW::new(self, 6)
    }
}
#[doc = "ENDPOINT0 Host Mode Only: Operating Speed Of The Targeted Device\n\nYou can [`read`](crate::Reg::read) this register and get [`type0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`type0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Type0Spec;
impl crate::RegisterSpec for Type0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`type0::R`](R) reader structure"]
impl crate::Readable for Type0Spec {}
#[doc = "`write(|w| ..)` method takes [`type0::W`](W) writer structure"]
impl crate::Writable for Type0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TYPE0 to value 0"]
impl crate::Resettable for Type0Spec {}
