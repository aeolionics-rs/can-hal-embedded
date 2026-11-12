#[doc = "Register `AUDCTRL` reader"]
pub type R = crate::R<AudctrlSpec>;
#[doc = "Register `AUDCTRL` writer"]
pub type W = crate::W<AudctrlSpec>;
#[doc = "Audio Protocol Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Audmodselect {
    #[doc = "0: I2S/I8S Standard Mode"]
    Stdmode = 0,
    #[doc = "1: I2S/I8S Left Or Right Justified"]
    Ljrjmode = 1,
    #[doc = "3: PCM/DSP Mode"]
    Pcmmode = 3,
}
impl From<Audmodselect> for u8 {
    #[inline(always)]
    fn from(variant: Audmodselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Audmodselect {
    type Ux = u8;
}
impl crate::IsEnum for Audmodselect {}
#[doc = "Field `AUDMOD` reader - Audio Protocol Mode"]
pub type AudmodR = crate::FieldReader<Audmodselect>;
impl AudmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Audmodselect> {
        match self.bits {
            0 => Some(Audmodselect::Stdmode),
            1 => Some(Audmodselect::Ljrjmode),
            3 => Some(Audmodselect::Pcmmode),
            _ => None,
        }
    }
    #[doc = "I2S/I8S Standard Mode"]
    #[inline(always)]
    pub fn is_stdmode(&self) -> bool {
        *self == Audmodselect::Stdmode
    }
    #[doc = "I2S/I8S Left Or Right Justified"]
    #[inline(always)]
    pub fn is_ljrjmode(&self) -> bool {
        *self == Audmodselect::Ljrjmode
    }
    #[doc = "PCM/DSP Mode"]
    #[inline(always)]
    pub fn is_pcmmode(&self) -> bool {
        *self == Audmodselect::Pcmmode
    }
}
#[doc = "Field `AUDMOD` writer - Audio Protocol Mode"]
pub type AudmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Audmodselect>;
impl<'a, REG> AudmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2S/I8S Standard Mode"]
    #[inline(always)]
    pub fn stdmode(self) -> &'a mut crate::W<REG> {
        self.variant(Audmodselect::Stdmode)
    }
    #[doc = "I2S/I8S Left Or Right Justified"]
    #[inline(always)]
    pub fn ljrjmode(self) -> &'a mut crate::W<REG> {
        self.variant(Audmodselect::Ljrjmode)
    }
    #[doc = "PCM/DSP Mode"]
    #[inline(always)]
    pub fn pcmmode(self) -> &'a mut crate::W<REG> {
        self.variant(Audmodselect::Pcmmode)
    }
}
#[doc = "Field `AUDMONO` reader - Transmit audio data format"]
pub type AudmonoR = crate::BitReader;
#[doc = "Field `AUDMONO` writer - Transmit audio data format"]
pub type AudmonoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Audio Protocol Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Audfmtselect {
    #[doc = "0: Legacy I2S Mode"]
    I2slegacy = 0,
    #[doc = "1: I2S Raw Audio Format"]
    I2sraw = 1,
    #[doc = "2: I2S Other AM824 Format"]
    I2sother824 = 2,
    #[doc = "4: I8S Other Format"]
    I8sother = 4,
    #[doc = "5: I8S Raw Audio Format"]
    I8sraw = 5,
    #[doc = "6: I8S Other AM824 Format"]
    I8sother824 = 6,
}
impl From<Audfmtselect> for u8 {
    #[inline(always)]
    fn from(variant: Audfmtselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Audfmtselect {
    type Ux = u8;
}
impl crate::IsEnum for Audfmtselect {}
#[doc = "Field `AUDFMT` reader - Audio Protocol Format"]
pub type AudfmtR = crate::FieldReader<Audfmtselect>;
impl AudfmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Audfmtselect> {
        match self.bits {
            0 => Some(Audfmtselect::I2slegacy),
            1 => Some(Audfmtselect::I2sraw),
            2 => Some(Audfmtselect::I2sother824),
            4 => Some(Audfmtselect::I8sother),
            5 => Some(Audfmtselect::I8sraw),
            6 => Some(Audfmtselect::I8sother824),
            _ => None,
        }
    }
    #[doc = "Legacy I2S Mode"]
    #[inline(always)]
    pub fn is_i2slegacy(&self) -> bool {
        *self == Audfmtselect::I2slegacy
    }
    #[doc = "I2S Raw Audio Format"]
    #[inline(always)]
    pub fn is_i2sraw(&self) -> bool {
        *self == Audfmtselect::I2sraw
    }
    #[doc = "I2S Other AM824 Format"]
    #[inline(always)]
    pub fn is_i2sother824(&self) -> bool {
        *self == Audfmtselect::I2sother824
    }
    #[doc = "I8S Other Format"]
    #[inline(always)]
    pub fn is_i8sother(&self) -> bool {
        *self == Audfmtselect::I8sother
    }
    #[doc = "I8S Raw Audio Format"]
    #[inline(always)]
    pub fn is_i8sraw(&self) -> bool {
        *self == Audfmtselect::I8sraw
    }
    #[doc = "I8S Other AM824 Format"]
    #[inline(always)]
    pub fn is_i8sother824(&self) -> bool {
        *self == Audfmtselect::I8sother824
    }
}
#[doc = "Field `AUDFMT` writer - Audio Protocol Format"]
pub type AudfmtW<'a, REG> = crate::FieldWriter<'a, REG, 3, Audfmtselect>;
impl<'a, REG> AudfmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Legacy I2S Mode"]
    #[inline(always)]
    pub fn i2slegacy(self) -> &'a mut crate::W<REG> {
        self.variant(Audfmtselect::I2slegacy)
    }
    #[doc = "I2S Raw Audio Format"]
    #[inline(always)]
    pub fn i2sraw(self) -> &'a mut crate::W<REG> {
        self.variant(Audfmtselect::I2sraw)
    }
    #[doc = "I2S Other AM824 Format"]
    #[inline(always)]
    pub fn i2sother824(self) -> &'a mut crate::W<REG> {
        self.variant(Audfmtselect::I2sother824)
    }
    #[doc = "I8S Other Format"]
    #[inline(always)]
    pub fn i8sother(self) -> &'a mut crate::W<REG> {
        self.variant(Audfmtselect::I8sother)
    }
    #[doc = "I8S Raw Audio Format"]
    #[inline(always)]
    pub fn i8sraw(self) -> &'a mut crate::W<REG> {
        self.variant(Audfmtselect::I8sraw)
    }
    #[doc = "I8S Other AM824 Format"]
    #[inline(always)]
    pub fn i8sother824(self) -> &'a mut crate::W<REG> {
        self.variant(Audfmtselect::I8sother824)
    }
}
#[doc = "Field `AUDWDMODE` reader - Serial Word Length bits (Ignored when AUDEN=0)"]
pub type AudwdmodeR = crate::FieldReader;
#[doc = "Field `AUDWDMODE` writer - Serial Word Length bits (Ignored when AUDEN=0)"]
pub type AudwdmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Audio Protocol Mode"]
    #[inline(always)]
    pub fn audmod(&self) -> AudmodR {
        AudmodR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Transmit audio data format"]
    #[inline(always)]
    pub fn audmono(&self) -> AudmonoR {
        AudmonoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Audio Protocol Format"]
    #[inline(always)]
    pub fn audfmt(&self) -> AudfmtR {
        AudfmtR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Serial Word Length bits (Ignored when AUDEN=0)"]
    #[inline(always)]
    pub fn audwdmode(&self) -> AudwdmodeR {
        AudwdmodeR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Audio Protocol Mode"]
    #[inline(always)]
    pub fn audmod(&mut self) -> AudmodW<'_, AudctrlSpec> {
        AudmodW::new(self, 0)
    }
    #[doc = "Bit 3 - Transmit audio data format"]
    #[inline(always)]
    pub fn audmono(&mut self) -> AudmonoW<'_, AudctrlSpec> {
        AudmonoW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Audio Protocol Format"]
    #[inline(always)]
    pub fn audfmt(&mut self) -> AudfmtW<'_, AudctrlSpec> {
        AudfmtW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Serial Word Length bits (Ignored when AUDEN=0)"]
    #[inline(always)]
    pub fn audwdmode(&mut self) -> AudwdmodeW<'_, AudctrlSpec> {
        AudwdmodeW::new(self, 8)
    }
}
#[doc = "SPI Control Audio Register\n\nYou can [`read`](crate::Reg::read) this register and get [`audctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudctrlSpec;
impl crate::RegisterSpec for AudctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audctrl::R`](R) reader structure"]
impl crate::Readable for AudctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`audctrl::W`](W) writer structure"]
impl crate::Writable for AudctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDCTRL to value 0"]
impl crate::Resettable for AudctrlSpec {}
