#[doc = "Register `CMPCTRL` reader"]
pub type R = crate::R<CmpctrlSpec>;
#[doc = "Register `CMPCTRL` writer"]
pub type W = crate::W<CmpctrlSpec>;
#[doc = "Field `ADCMPLO` reader - Low Limit of Digital Comparator"]
pub type AdcmploR = crate::FieldReader<u16>;
#[doc = "Field `ADCMPLO` writer - Low Limit of Digital Comparator"]
pub type AdcmploW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CMPEN` reader - Comparator Enable"]
pub type CmpenR = crate::BitReader;
#[doc = "Field `CMPEN` writer - Comparator Enable"]
pub type CmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IELOLO` reader - Enable VAL < CMPLO"]
pub type IeloloR = crate::BitReader;
#[doc = "Field `IELOLO` writer - Enable VAL < CMPLO"]
pub type IeloloW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IELOHI` reader - Enable VAL >= CMPLO"]
pub type IelohiR = crate::BitReader;
#[doc = "Field `IELOHI` writer - Enable VAL >= CMPLO"]
pub type IelohiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEBTWN` reader - Enable CMPLO <= VAL < CMPHI"]
pub type IebtwnR = crate::BitReader;
#[doc = "Field `IEBTWN` writer - Enable CMPLO <= VAL < CMPHI"]
pub type IebtwnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCMPHI` reader - High Limit of Digital Comparator"]
pub type AdcmphiR = crate::FieldReader<u16>;
#[doc = "Field `ADCMPHI` writer - High Limit of Digital Comparator"]
pub type AdcmphiW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `IEHILO` reader - Enable VAL < CMPHI"]
pub type IehiloR = crate::BitReader;
#[doc = "Field `IEHILO` writer - Enable VAL < CMPHI"]
pub type IehiloW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEHIHI` reader - Enable VAL >= CMPHI"]
pub type IehihiR = crate::BitReader;
#[doc = "Field `IEHIHI` writer - Enable VAL >= CMPHI"]
pub type IehihiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Low Limit of Digital Comparator"]
    #[inline(always)]
    pub fn adcmplo(&self) -> AdcmploR {
        AdcmploR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Comparator Enable"]
    #[inline(always)]
    pub fn cmpen(&self) -> CmpenR {
        CmpenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable VAL < CMPLO"]
    #[inline(always)]
    pub fn ielolo(&self) -> IeloloR {
        IeloloR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable VAL >= CMPLO"]
    #[inline(always)]
    pub fn ielohi(&self) -> IelohiR {
        IelohiR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable CMPLO <= VAL < CMPHI"]
    #[inline(always)]
    pub fn iebtwn(&self) -> IebtwnR {
        IebtwnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - High Limit of Digital Comparator"]
    #[inline(always)]
    pub fn adcmphi(&self) -> AdcmphiR {
        AdcmphiR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - Enable VAL < CMPHI"]
    #[inline(always)]
    pub fn iehilo(&self) -> IehiloR {
        IehiloR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable VAL >= CMPHI"]
    #[inline(always)]
    pub fn iehihi(&self) -> IehihiR {
        IehihiR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Low Limit of Digital Comparator"]
    #[inline(always)]
    pub fn adcmplo(&mut self) -> AdcmploW<'_, CmpctrlSpec> {
        AdcmploW::new(self, 0)
    }
    #[doc = "Bit 12 - Comparator Enable"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CmpenW<'_, CmpctrlSpec> {
        CmpenW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable VAL < CMPLO"]
    #[inline(always)]
    pub fn ielolo(&mut self) -> IeloloW<'_, CmpctrlSpec> {
        IeloloW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable VAL >= CMPLO"]
    #[inline(always)]
    pub fn ielohi(&mut self) -> IelohiW<'_, CmpctrlSpec> {
        IelohiW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable CMPLO <= VAL < CMPHI"]
    #[inline(always)]
    pub fn iebtwn(&mut self) -> IebtwnW<'_, CmpctrlSpec> {
        IebtwnW::new(self, 15)
    }
    #[doc = "Bits 16:27 - High Limit of Digital Comparator"]
    #[inline(always)]
    pub fn adcmphi(&mut self) -> AdcmphiW<'_, CmpctrlSpec> {
        AdcmphiW::new(self, 16)
    }
    #[doc = "Bit 28 - Enable VAL < CMPHI"]
    #[inline(always)]
    pub fn iehilo(&mut self) -> IehiloW<'_, CmpctrlSpec> {
        IehiloW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable VAL >= CMPHI"]
    #[inline(always)]
    pub fn iehihi(&mut self) -> IehihiW<'_, CmpctrlSpec> {
        IehihiW::new(self, 29)
    }
}
#[doc = "Comparator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpctrlSpec;
impl crate::RegisterSpec for CmpctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpctrl::R`](R) reader structure"]
impl crate::Readable for CmpctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpctrl::W`](W) writer structure"]
impl crate::Writable for CmpctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPCTRL to value 0"]
impl crate::Resettable for CmpctrlSpec {}
