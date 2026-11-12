#[doc = "Register `CLKSELCTRL` reader"]
pub type R = crate::R<ClkselctrlSpec>;
#[doc = "Register `CLKSELCTRL` writer"]
pub type W = crate::W<ClkselctrlSpec>;
#[doc = "RTC Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcselselect {
    #[doc = "0: 32.768kHz from 32kHz internal ULP oscillator"]
    Ulp32k = 0,
    #[doc = "1: 1.024kHz from 32kHz internal ULP oscillator"]
    Ulp1k = 1,
    #[doc = "2: 32.768kHz from 32.768kHz external crystal oscillator"]
    Xosc32k = 2,
    #[doc = "3: 1.024kHz from 32.768kHz internal oscillator"]
    Xosc1k = 3,
}
impl From<Rtcselselect> for u8 {
    #[inline(always)]
    fn from(variant: Rtcselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcselselect {
    type Ux = u8;
}
impl crate::IsEnum for Rtcselselect {}
#[doc = "Field `RTCSEL` reader - RTC Clock Selection"]
pub type RtcselR = crate::FieldReader<Rtcselselect>;
impl RtcselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcselselect {
        match self.bits {
            0 => Rtcselselect::Ulp32k,
            1 => Rtcselselect::Ulp1k,
            2 => Rtcselselect::Xosc32k,
            3 => Rtcselselect::Xosc1k,
            _ => unreachable!(),
        }
    }
    #[doc = "32.768kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn is_ulp32k(&self) -> bool {
        *self == Rtcselselect::Ulp32k
    }
    #[doc = "1.024kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn is_ulp1k(&self) -> bool {
        *self == Rtcselselect::Ulp1k
    }
    #[doc = "32.768kHz from 32.768kHz external crystal oscillator"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == Rtcselselect::Xosc32k
    }
    #[doc = "1.024kHz from 32.768kHz internal oscillator"]
    #[inline(always)]
    pub fn is_xosc1k(&self) -> bool {
        *self == Rtcselselect::Xosc1k
    }
}
#[doc = "Field `RTCSEL` writer - RTC Clock Selection"]
pub type RtcselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtcselselect, crate::Safe>;
impl<'a, REG> RtcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32.768kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn ulp32k(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcselselect::Ulp32k)
    }
    #[doc = "1.024kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn ulp1k(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcselselect::Ulp1k)
    }
    #[doc = "32.768kHz from 32.768kHz external crystal oscillator"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcselselect::Xosc32k)
    }
    #[doc = "1.024kHz from 32.768kHz internal oscillator"]
    #[inline(always)]
    pub fn xosc1k(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcselselect::Xosc1k)
    }
}
#[doc = "HSM Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hsmselselect {
    #[doc = "0: 32.768kHz from 32kHz internal ULP oscillator"]
    Ulp32k = 0,
    #[doc = "2: 32.768kHz from 32.768kHz external crystal oscillator"]
    Xosc32k = 2,
}
impl From<Hsmselselect> for u8 {
    #[inline(always)]
    fn from(variant: Hsmselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hsmselselect {
    type Ux = u8;
}
impl crate::IsEnum for Hsmselselect {}
#[doc = "Field `HSMSEL` reader - HSM Clock Selection"]
pub type HsmselR = crate::FieldReader<Hsmselselect>;
impl HsmselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hsmselselect> {
        match self.bits {
            0 => Some(Hsmselselect::Ulp32k),
            2 => Some(Hsmselselect::Xosc32k),
            _ => None,
        }
    }
    #[doc = "32.768kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn is_ulp32k(&self) -> bool {
        *self == Hsmselselect::Ulp32k
    }
    #[doc = "32.768kHz from 32.768kHz external crystal oscillator"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == Hsmselselect::Xosc32k
    }
}
#[doc = "Field `HSMSEL` writer - HSM Clock Selection"]
pub type HsmselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hsmselselect>;
impl<'a, REG> HsmselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32.768kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn ulp32k(self) -> &'a mut crate::W<REG> {
        self.variant(Hsmselselect::Ulp32k)
    }
    #[doc = "32.768kHz from 32.768kHz external crystal oscillator"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut crate::W<REG> {
        self.variant(Hsmselselect::Xosc32k)
    }
}
impl R {
    #[doc = "Bits 0:1 - RTC Clock Selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RtcselR {
        RtcselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - HSM Clock Selection"]
    #[inline(always)]
    pub fn hsmsel(&self) -> HsmselR {
        HsmselR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RTC Clock Selection"]
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RtcselW<'_, ClkselctrlSpec> {
        RtcselW::new(self, 0)
    }
    #[doc = "Bits 4:5 - HSM Clock Selection"]
    #[inline(always)]
    pub fn hsmsel(&mut self) -> HsmselW<'_, ClkselctrlSpec> {
        HsmselW::new(self, 4)
    }
}
#[doc = "Clock Selection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`clkselctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkselctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselctrlSpec;
impl crate::RegisterSpec for ClkselctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkselctrl::R`](R) reader structure"]
impl crate::Readable for ClkselctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkselctrl::W`](W) writer structure"]
impl crate::Writable for ClkselctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKSELCTRL to value 0"]
impl crate::Resettable for ClkselctrlSpec {}
