#[doc = "Register `FLTCTRL` reader"]
pub type R = crate::R<FltctrlSpec>;
#[doc = "Register `FLTCTRL` writer"]
pub type W = crate::W<FltctrlSpec>;
#[doc = "Oversampling Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ovrsamselect {
    #[doc = "0: (If FMODE is 0) 4 samples (shift sum 1 bit to right, output data is 13-bits) / (If FMODE is 1) 2 samples to be averaged"]
    _4Samples = 0,
    #[doc = "1: (If FMODE is 0) 16 samples (shift sum 2 bits to right, output data is 14-bits) / (If FMODE is 1) 4 samples to be averaged"]
    _16Samples = 1,
    #[doc = "2: (If FMODE is 0) 64 samples (shift sum 3 bits to right, output data is 15-bits) / (If FMODE is 1) 8 samples to be averaged"]
    _64Samples = 2,
    #[doc = "3: (If FMODE is 0) 256 samples (shift sum 4 bits to right, output data is 16-bits) / (If FMODE is 1) 16 samples to be averaged"]
    _256Samples = 3,
    #[doc = "4: (If FMODE is 0) 2 samples (shift sum 0 bits to right, output data is 12.1 format) / (If FMODE is 1) 32 samples to be averaged"]
    _2Samples = 4,
    #[doc = "5: (If FMODE is 0) 8 samples (shift sum 1 bit to right, output data is 13.1 format) / (If FMODE is 1) 64 samples to be averaged"]
    _8Samples = 5,
    #[doc = "6: (If FMODE is 0) 32 samples (shift sum 2 bits to right, output data is 14.1 format) / (If FMODE is 1) 128 samples to be averaged"]
    _32Samples = 6,
    #[doc = "7: (If FMODE is 0) 128 samples (shift sum 3 bits to right, output data is 15.1 format) / (If FMODE is 1) 256 samples to be averaged"]
    _128Samples = 7,
}
impl From<Ovrsamselect> for u8 {
    #[inline(always)]
    fn from(variant: Ovrsamselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ovrsamselect {
    type Ux = u8;
}
impl crate::IsEnum for Ovrsamselect {}
#[doc = "Field `OVRSAM` reader - Oversampling Ratio"]
pub type OvrsamR = crate::FieldReader<Ovrsamselect>;
impl OvrsamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovrsamselect {
        match self.bits {
            0 => Ovrsamselect::_4Samples,
            1 => Ovrsamselect::_16Samples,
            2 => Ovrsamselect::_64Samples,
            3 => Ovrsamselect::_256Samples,
            4 => Ovrsamselect::_2Samples,
            5 => Ovrsamselect::_8Samples,
            6 => Ovrsamselect::_32Samples,
            7 => Ovrsamselect::_128Samples,
            _ => unreachable!(),
        }
    }
    #[doc = "(If FMODE is 0) 4 samples (shift sum 1 bit to right, output data is 13-bits) / (If FMODE is 1) 2 samples to be averaged"]
    #[inline(always)]
    pub fn is_4_samples(&self) -> bool {
        *self == Ovrsamselect::_4Samples
    }
    #[doc = "(If FMODE is 0) 16 samples (shift sum 2 bits to right, output data is 14-bits) / (If FMODE is 1) 4 samples to be averaged"]
    #[inline(always)]
    pub fn is_16_samples(&self) -> bool {
        *self == Ovrsamselect::_16Samples
    }
    #[doc = "(If FMODE is 0) 64 samples (shift sum 3 bits to right, output data is 15-bits) / (If FMODE is 1) 8 samples to be averaged"]
    #[inline(always)]
    pub fn is_64_samples(&self) -> bool {
        *self == Ovrsamselect::_64Samples
    }
    #[doc = "(If FMODE is 0) 256 samples (shift sum 4 bits to right, output data is 16-bits) / (If FMODE is 1) 16 samples to be averaged"]
    #[inline(always)]
    pub fn is_256_samples(&self) -> bool {
        *self == Ovrsamselect::_256Samples
    }
    #[doc = "(If FMODE is 0) 2 samples (shift sum 0 bits to right, output data is 12.1 format) / (If FMODE is 1) 32 samples to be averaged"]
    #[inline(always)]
    pub fn is_2_samples(&self) -> bool {
        *self == Ovrsamselect::_2Samples
    }
    #[doc = "(If FMODE is 0) 8 samples (shift sum 1 bit to right, output data is 13.1 format) / (If FMODE is 1) 64 samples to be averaged"]
    #[inline(always)]
    pub fn is_8_samples(&self) -> bool {
        *self == Ovrsamselect::_8Samples
    }
    #[doc = "(If FMODE is 0) 32 samples (shift sum 2 bits to right, output data is 14.1 format) / (If FMODE is 1) 128 samples to be averaged"]
    #[inline(always)]
    pub fn is_32_samples(&self) -> bool {
        *self == Ovrsamselect::_32Samples
    }
    #[doc = "(If FMODE is 0) 128 samples (shift sum 3 bits to right, output data is 15.1 format) / (If FMODE is 1) 256 samples to be averaged"]
    #[inline(always)]
    pub fn is_128_samples(&self) -> bool {
        *self == Ovrsamselect::_128Samples
    }
}
#[doc = "Field `OVRSAM` writer - Oversampling Ratio"]
pub type OvrsamW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ovrsamselect, crate::Safe>;
impl<'a, REG> OvrsamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "(If FMODE is 0) 4 samples (shift sum 1 bit to right, output data is 13-bits) / (If FMODE is 1) 2 samples to be averaged"]
    #[inline(always)]
    pub fn _4_samples(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrsamselect::_4Samples)
    }
    #[doc = "(If FMODE is 0) 16 samples (shift sum 2 bits to right, output data is 14-bits) / (If FMODE is 1) 4 samples to be averaged"]
    #[inline(always)]
    pub fn _16_samples(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrsamselect::_16Samples)
    }
    #[doc = "(If FMODE is 0) 64 samples (shift sum 3 bits to right, output data is 15-bits) / (If FMODE is 1) 8 samples to be averaged"]
    #[inline(always)]
    pub fn _64_samples(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrsamselect::_64Samples)
    }
    #[doc = "(If FMODE is 0) 256 samples (shift sum 4 bits to right, output data is 16-bits) / (If FMODE is 1) 16 samples to be averaged"]
    #[inline(always)]
    pub fn _256_samples(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrsamselect::_256Samples)
    }
    #[doc = "(If FMODE is 0) 2 samples (shift sum 0 bits to right, output data is 12.1 format) / (If FMODE is 1) 32 samples to be averaged"]
    #[inline(always)]
    pub fn _2_samples(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrsamselect::_2Samples)
    }
    #[doc = "(If FMODE is 0) 8 samples (shift sum 1 bit to right, output data is 13.1 format) / (If FMODE is 1) 64 samples to be averaged"]
    #[inline(always)]
    pub fn _8_samples(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrsamselect::_8Samples)
    }
    #[doc = "(If FMODE is 0) 32 samples (shift sum 2 bits to right, output data is 14.1 format) / (If FMODE is 1) 128 samples to be averaged"]
    #[inline(always)]
    pub fn _32_samples(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrsamselect::_32Samples)
    }
    #[doc = "(If FMODE is 0) 128 samples (shift sum 3 bits to right, output data is 15.1 format) / (If FMODE is 1) 256 samples to be averaged"]
    #[inline(always)]
    pub fn _128_samples(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrsamselect::_128Samples)
    }
}
#[doc = "Field `FMODE` reader - Filter Mode"]
pub type FmodeR = crate::BitReader;
#[doc = "Field `FMODE` writer - Filter Mode"]
pub type FmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA16EN` reader - 16bit Averaging Mode"]
pub type Data16enR = crate::BitReader;
#[doc = "Field `DATA16EN` writer - 16bit Averaging Mode"]
pub type Data16enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTEN` reader - Filter Enable"]
pub type FltenR = crate::BitReader;
#[doc = "Field `FLTEN` writer - Filter Enable"]
pub type FltenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTCHNID` reader - Channel ID"]
pub type FltchnidR = crate::FieldReader;
#[doc = "Field `FLTCHNID` writer - Channel ID"]
pub type FltchnidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Oversampling Ratio"]
    #[inline(always)]
    pub fn ovrsam(&self) -> OvrsamR {
        OvrsamR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Filter Mode"]
    #[inline(always)]
    pub fn fmode(&self) -> FmodeR {
        FmodeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 16bit Averaging Mode"]
    #[inline(always)]
    pub fn data16en(&self) -> Data16enR {
        Data16enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter Enable"]
    #[inline(always)]
    pub fn flten(&self) -> FltenR {
        FltenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Channel ID"]
    #[inline(always)]
    pub fn fltchnid(&self) -> FltchnidR {
        FltchnidR::new(((self.bits >> 10) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Oversampling Ratio"]
    #[inline(always)]
    pub fn ovrsam(&mut self) -> OvrsamW<'_, FltctrlSpec> {
        OvrsamW::new(self, 0)
    }
    #[doc = "Bit 3 - Filter Mode"]
    #[inline(always)]
    pub fn fmode(&mut self) -> FmodeW<'_, FltctrlSpec> {
        FmodeW::new(self, 3)
    }
    #[doc = "Bit 4 - 16bit Averaging Mode"]
    #[inline(always)]
    pub fn data16en(&mut self) -> Data16enW<'_, FltctrlSpec> {
        Data16enW::new(self, 4)
    }
    #[doc = "Bit 8 - Filter Enable"]
    #[inline(always)]
    pub fn flten(&mut self) -> FltenW<'_, FltctrlSpec> {
        FltenW::new(self, 8)
    }
    #[doc = "Bits 10:13 - Channel ID"]
    #[inline(always)]
    pub fn fltchnid(&mut self) -> FltchnidW<'_, FltctrlSpec> {
        FltchnidW::new(self, 10)
    }
}
#[doc = "Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fltctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FltctrlSpec;
impl crate::RegisterSpec for FltctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltctrl::R`](R) reader structure"]
impl crate::Readable for FltctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fltctrl::W`](W) writer structure"]
impl crate::Writable for FltctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLTCTRL to value 0"]
impl crate::Resettable for FltctrlSpec {}
