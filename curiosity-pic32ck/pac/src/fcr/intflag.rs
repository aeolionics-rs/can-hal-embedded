#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `SERR` reader - Flash Single Error Corrected Flag"]
pub type SerrR = crate::BitReader;
#[doc = "Field `SERR` writer - Flash Single Error Corrected Flag"]
pub type SerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DERR` reader - ECC Double Error Detected Flag"]
pub type DerrR = crate::BitReader;
#[doc = "Field `DERR` writer - ECC Double Error Detected Flag"]
pub type DerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CRCDONE` reader - CRC Calculation Done Flag"]
pub type CrcdoneR = crate::BitReader;
#[doc = "Field `CRCDONE` writer - CRC Calculation Done Flag"]
pub type CrcdoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CRCERR` reader - CRC Error Flag"]
pub type CrcerrR = crate::BitReader;
#[doc = "Field `CRCERR` writer - CRC Error Flag"]
pub type CrcerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FLTCAP` reader - ECC Fault Capture Flag"]
pub type FltcapR = crate::BitReader;
#[doc = "Field `FLTCAP` writer - ECC Fault Capture Flag"]
pub type FltcapW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flash Single Error Corrected Flag"]
    #[inline(always)]
    pub fn serr(&self) -> SerrR {
        SerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC Double Error Detected Flag"]
    #[inline(always)]
    pub fn derr(&self) -> DerrR {
        DerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - CRC Calculation Done Flag"]
    #[inline(always)]
    pub fn crcdone(&self) -> CrcdoneR {
        CrcdoneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CRC Error Flag"]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - ECC Fault Capture Flag"]
    #[inline(always)]
    pub fn fltcap(&self) -> FltcapR {
        FltcapR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Single Error Corrected Flag"]
    #[inline(always)]
    pub fn serr(&mut self) -> SerrW<'_, IntflagSpec> {
        SerrW::new(self, 0)
    }
    #[doc = "Bit 1 - ECC Double Error Detected Flag"]
    #[inline(always)]
    pub fn derr(&mut self) -> DerrW<'_, IntflagSpec> {
        DerrW::new(self, 1)
    }
    #[doc = "Bit 8 - CRC Calculation Done Flag"]
    #[inline(always)]
    pub fn crcdone(&mut self) -> CrcdoneW<'_, IntflagSpec> {
        CrcdoneW::new(self, 8)
    }
    #[doc = "Bit 9 - CRC Error Flag"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CrcerrW<'_, IntflagSpec> {
        CrcerrW::new(self, 9)
    }
    #[doc = "Bit 16 - ECC Fault Capture Flag"]
    #[inline(always)]
    pub fn fltcap(&mut self) -> FltcapW<'_, IntflagSpec> {
        FltcapW::new(self, 16)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0303;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
