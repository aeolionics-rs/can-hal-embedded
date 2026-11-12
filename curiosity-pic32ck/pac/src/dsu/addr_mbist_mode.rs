#[doc = "Register `ADDR_MBIST_MODE` reader"]
pub type R = crate::R<AddrMbistModeSpec>;
#[doc = "Register `ADDR_MBIST_MODE` writer"]
pub type W = crate::W<AddrMbistModeSpec>;
#[doc = "Access Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Amodselect {
    #[doc = "0: STATUSA.FAIL rises upon first error and algorithm stops (STATUSA.DONE rises)."]
    ExitOnError = 0,
    #[doc = "1: STATUSA.FAIL rises when an error is detected and algorith stops until STATUSA.FAIL is cleared. Once cleared, the algorithm is resumed going to next test step."]
    PauseOnError = 1,
}
impl From<Amodselect> for u8 {
    #[inline(always)]
    fn from(variant: Amodselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Amodselect {
    type Ux = u8;
}
impl crate::IsEnum for Amodselect {}
#[doc = "Field `AMOD` reader - Access Mode"]
pub type AmodR = crate::FieldReader<Amodselect>;
impl AmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Amodselect> {
        match self.bits {
            0 => Some(Amodselect::ExitOnError),
            1 => Some(Amodselect::PauseOnError),
            _ => None,
        }
    }
    #[doc = "STATUSA.FAIL rises upon first error and algorithm stops (STATUSA.DONE rises)."]
    #[inline(always)]
    pub fn is_exit_on_error(&self) -> bool {
        *self == Amodselect::ExitOnError
    }
    #[doc = "STATUSA.FAIL rises when an error is detected and algorith stops until STATUSA.FAIL is cleared. Once cleared, the algorithm is resumed going to next test step."]
    #[inline(always)]
    pub fn is_pause_on_error(&self) -> bool {
        *self == Amodselect::PauseOnError
    }
}
#[doc = "Field `AMOD` writer - Access Mode"]
pub type AmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Amodselect>;
impl<'a, REG> AmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "STATUSA.FAIL rises upon first error and algorithm stops (STATUSA.DONE rises)."]
    #[inline(always)]
    pub fn exit_on_error(self) -> &'a mut crate::W<REG> {
        self.variant(Amodselect::ExitOnError)
    }
    #[doc = "STATUSA.FAIL rises when an error is detected and algorith stops until STATUSA.FAIL is cleared. Once cleared, the algorithm is resumed going to next test step."]
    #[inline(always)]
    pub fn pause_on_error(self) -> &'a mut crate::W<REG> {
        self.variant(Amodselect::PauseOnError)
    }
}
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - Access Mode"]
    #[inline(always)]
    pub fn amod(&self) -> AmodR {
        AmodR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - Access Mode"]
    #[inline(always)]
    pub fn amod(&mut self) -> AmodW<'_, AddrMbistModeSpec> {
        AmodW::new(self, 0)
    }
    #[doc = "Bits 2:31 - Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, AddrMbistModeSpec> {
        AddrW::new(self, 2)
    }
}
#[doc = "Address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_mbist_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_mbist_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrMbistModeSpec;
impl crate::RegisterSpec for AddrMbistModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr_mbist_mode::R`](R) reader structure"]
impl crate::Readable for AddrMbistModeSpec {}
#[doc = "`write(|w| ..)` method takes [`addr_mbist_mode::W`](W) writer structure"]
impl crate::Writable for AddrMbistModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR_MBIST_MODE to value 0"]
impl crate::Resettable for AddrMbistModeSpec {}
