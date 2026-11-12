#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `ADCHSEL` reader - Software Trigger Channel Select"]
pub type AdchselR = crate::FieldReader;
#[doc = "Field `ADCHSEL` writer - Software Trigger Channel Select"]
pub type AdchselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADCORSEL` reader - Software Trigger Core Select"]
pub type AdcorselR = crate::FieldReader;
#[doc = "Field `ADCORSEL` writer - Software Trigger Core Select"]
pub type AdcorselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RQCNVRT` reader - Request Channel Convert"]
pub type RqcnvrtR = crate::BitReader;
#[doc = "Field `RQCNVRT` writer - Request Channel Convert"]
pub type RqcnvrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMP` reader - Channel Sample"]
pub type SampR = crate::BitReader;
#[doc = "Field `SAMP` writer - Channel Sample"]
pub type SampW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSWTRG` reader - Global Software Trigger"]
pub type GswtrgR = crate::BitReader;
#[doc = "Field `GSWTRG` writer - Global Software Trigger"]
pub type GswtrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSWTRG` reader - Level Global Trigger"]
pub type LswtrgR = crate::BitReader;
#[doc = "Field `LSWTRG` writer - Level Global Trigger"]
pub type LswtrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSUSP` reader - Trigger Suspend"]
pub type TrgsuspR = crate::BitReader;
#[doc = "Field `TRGSUSP` writer - Trigger Suspend"]
pub type TrgsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRGEN` reader - Synchronous Trigger Enable"]
pub type StrgenR = crate::BitReader;
#[doc = "Field `STRGEN` writer - Synchronous Trigger Enable"]
pub type StrgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWCNVEN` reader - Software Conversion Enable"]
pub type SwcnvenR = crate::BitReader;
#[doc = "Field `SWCNVEN` writer - Software Conversion Enable"]
pub type SwcnvenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Software Trigger Channel Select"]
    #[inline(always)]
    pub fn adchsel(&self) -> AdchselR {
        AdchselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Software Trigger Core Select"]
    #[inline(always)]
    pub fn adcorsel(&self) -> AdcorselR {
        AdcorselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Request Channel Convert"]
    #[inline(always)]
    pub fn rqcnvrt(&self) -> RqcnvrtR {
        RqcnvrtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Sample"]
    #[inline(always)]
    pub fn samp(&self) -> SampR {
        SampR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Software Trigger"]
    #[inline(always)]
    pub fn gswtrg(&self) -> GswtrgR {
        GswtrgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level Global Trigger"]
    #[inline(always)]
    pub fn lswtrg(&self) -> LswtrgR {
        LswtrgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Trigger Suspend"]
    #[inline(always)]
    pub fn trgsusp(&self) -> TrgsuspR {
        TrgsuspR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronous Trigger Enable"]
    #[inline(always)]
    pub fn strgen(&self) -> StrgenR {
        StrgenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Software Conversion Enable"]
    #[inline(always)]
    pub fn swcnven(&self) -> SwcnvenR {
        SwcnvenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Software Trigger Channel Select"]
    #[inline(always)]
    pub fn adchsel(&mut self) -> AdchselW<'_, CtrlbSpec> {
        AdchselW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Software Trigger Core Select"]
    #[inline(always)]
    pub fn adcorsel(&mut self) -> AdcorselW<'_, CtrlbSpec> {
        AdcorselW::new(self, 4)
    }
    #[doc = "Bit 6 - Request Channel Convert"]
    #[inline(always)]
    pub fn rqcnvrt(&mut self) -> RqcnvrtW<'_, CtrlbSpec> {
        RqcnvrtW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel Sample"]
    #[inline(always)]
    pub fn samp(&mut self) -> SampW<'_, CtrlbSpec> {
        SampW::new(self, 7)
    }
    #[doc = "Bit 8 - Global Software Trigger"]
    #[inline(always)]
    pub fn gswtrg(&mut self) -> GswtrgW<'_, CtrlbSpec> {
        GswtrgW::new(self, 8)
    }
    #[doc = "Bit 9 - Level Global Trigger"]
    #[inline(always)]
    pub fn lswtrg(&mut self) -> LswtrgW<'_, CtrlbSpec> {
        LswtrgW::new(self, 9)
    }
    #[doc = "Bit 10 - Trigger Suspend"]
    #[inline(always)]
    pub fn trgsusp(&mut self) -> TrgsuspW<'_, CtrlbSpec> {
        TrgsuspW::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronous Trigger Enable"]
    #[inline(always)]
    pub fn strgen(&mut self) -> StrgenW<'_, CtrlbSpec> {
        StrgenW::new(self, 11)
    }
    #[doc = "Bit 15 - Software Conversion Enable"]
    #[inline(always)]
    pub fn swcnven(&mut self) -> SwcnvenW<'_, CtrlbSpec> {
        SwcnvenW::new(self, 15)
    }
}
#[doc = "CONTROL B REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}
