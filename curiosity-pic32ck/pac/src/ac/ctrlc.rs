#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Field `WIDTH` reader - Sample and Hold Clock Pulse Width"]
pub type WidthR = crate::FieldReader<u16>;
#[doc = "Field `WIDTH` writer - Sample and Hold Clock Pulse Width"]
pub type WidthW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PER` reader - Sample and Hold Clock Period"]
pub type PerR = crate::FieldReader<u16>;
#[doc = "Field `PER` writer - Sample and Hold Clock Period"]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Prescaling Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescalerselect {
    #[doc = "0: Sampling rate is GCLK_AC (No division)"]
    Div1 = 0,
    #[doc = "1: Sampling rate is GCLK_AC/2"]
    Div2 = 1,
    #[doc = "2: Sampling rate is GCLK_AC/4"]
    Div4 = 2,
    #[doc = "3: Sampling rate is GCLK_AC/8"]
    Div8 = 3,
    #[doc = "4: Sampling rate is GCLK_AC/16"]
    Div16 = 4,
    #[doc = "5: Sampling rate is GCLK_AC/32"]
    Div32 = 5,
    #[doc = "6: Sampling rate is GCLK_AC/64"]
    Div64 = 6,
    #[doc = "7: Sampling rate is GCLK_AC/128"]
    Div128 = 7,
}
impl From<Prescalerselect> for u8 {
    #[inline(always)]
    fn from(variant: Prescalerselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescalerselect {
    type Ux = u8;
}
impl crate::IsEnum for Prescalerselect {}
#[doc = "Field `PRESCALER` reader - Prescaling Factor"]
pub type PrescalerR = crate::FieldReader<Prescalerselect>;
impl PrescalerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prescalerselect {
        match self.bits {
            0 => Prescalerselect::Div1,
            1 => Prescalerselect::Div2,
            2 => Prescalerselect::Div4,
            3 => Prescalerselect::Div8,
            4 => Prescalerselect::Div16,
            5 => Prescalerselect::Div32,
            6 => Prescalerselect::Div64,
            7 => Prescalerselect::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "Sampling rate is GCLK_AC (No division)"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Prescalerselect::Div1
    }
    #[doc = "Sampling rate is GCLK_AC/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Prescalerselect::Div2
    }
    #[doc = "Sampling rate is GCLK_AC/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Prescalerselect::Div4
    }
    #[doc = "Sampling rate is GCLK_AC/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Prescalerselect::Div8
    }
    #[doc = "Sampling rate is GCLK_AC/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Prescalerselect::Div16
    }
    #[doc = "Sampling rate is GCLK_AC/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Prescalerselect::Div32
    }
    #[doc = "Sampling rate is GCLK_AC/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Prescalerselect::Div64
    }
    #[doc = "Sampling rate is GCLK_AC/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Prescalerselect::Div128
    }
}
#[doc = "Field `PRESCALER` writer - Prescaling Factor"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prescalerselect, crate::Safe>;
impl<'a, REG> PrescalerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sampling rate is GCLK_AC (No division)"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div1)
    }
    #[doc = "Sampling rate is GCLK_AC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div2)
    }
    #[doc = "Sampling rate is GCLK_AC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div4)
    }
    #[doc = "Sampling rate is GCLK_AC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div8)
    }
    #[doc = "Sampling rate is GCLK_AC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div16)
    }
    #[doc = "Sampling rate is GCLK_AC/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div32)
    }
    #[doc = "Sampling rate is GCLK_AC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div64)
    }
    #[doc = "Sampling rate is GCLK_AC/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div128)
    }
}
impl R {
    #[doc = "Bits 0:9 - Sample and Hold Clock Pulse Width"]
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - Sample and Hold Clock Period"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:26 - Prescaling Factor"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sample and Hold Clock Pulse Width"]
    #[inline(always)]
    pub fn width(&mut self) -> WidthW<'_, CtrlcSpec> {
        WidthW::new(self, 0)
    }
    #[doc = "Bits 12:21 - Sample and Hold Clock Period"]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<'_, CtrlcSpec> {
        PerW::new(self, 12)
    }
    #[doc = "Bits 24:26 - Prescaling Factor"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PrescalerW<'_, CtrlcSpec> {
        PrescalerW::new(self, 24)
    }
}
#[doc = "Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlcSpec;
impl crate::RegisterSpec for CtrlcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CtrlcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CtrlcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CtrlcSpec {}
