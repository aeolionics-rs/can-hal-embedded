#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `SERR` reader - Flash Single Error Corrected Set Enable"]
pub type SerrR = crate::BitReader;
#[doc = "Field `SERR` writer - Flash Single Error Corrected Set Enable"]
pub type SerrW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `DERR` reader - ECC Double Error Detected Set Enable"]
pub type DerrR = crate::BitReader;
#[doc = "Field `DERR` writer - ECC Double Error Detected Set Enable"]
pub type DerrW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `CRCDONE` reader - CRC Calculation Done Set Enable"]
pub type CrcdoneR = crate::BitReader;
#[doc = "Field `CRCDONE` writer - CRC Calculation Done Set Enable"]
pub type CrcdoneW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `CRCERR` reader - CRC Error Interrupt Set Enable"]
pub type CrcerrR = crate::BitReader;
#[doc = "Field `CRCERR` writer - CRC Error Interrupt Set Enable"]
pub type CrcerrW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `FLTCAP` reader - ECC Fault Capture Interrupt Set Enable"]
pub type FltcapR = crate::BitReader;
#[doc = "Field `FLTCAP` writer - ECC Fault Capture Interrupt Set Enable"]
pub type FltcapW<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flash Single Error Corrected Set Enable"]
    #[inline(always)]
    pub fn serr(&self) -> SerrR {
        SerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC Double Error Detected Set Enable"]
    #[inline(always)]
    pub fn derr(&self) -> DerrR {
        DerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - CRC Calculation Done Set Enable"]
    #[inline(always)]
    pub fn crcdone(&self) -> CrcdoneR {
        CrcdoneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CRC Error Interrupt Set Enable"]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - ECC Fault Capture Interrupt Set Enable"]
    #[inline(always)]
    pub fn fltcap(&self) -> FltcapR {
        FltcapR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Single Error Corrected Set Enable"]
    #[inline(always)]
    pub fn serr(&mut self) -> SerrW<'_, IntensetSpec> {
        SerrW::new(self, 0)
    }
    #[doc = "Bit 1 - ECC Double Error Detected Set Enable"]
    #[inline(always)]
    pub fn derr(&mut self) -> DerrW<'_, IntensetSpec> {
        DerrW::new(self, 1)
    }
    #[doc = "Bit 8 - CRC Calculation Done Set Enable"]
    #[inline(always)]
    pub fn crcdone(&mut self) -> CrcdoneW<'_, IntensetSpec> {
        CrcdoneW::new(self, 8)
    }
    #[doc = "Bit 9 - CRC Error Interrupt Set Enable"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CrcerrW<'_, IntensetSpec> {
        CrcerrW::new(self, 9)
    }
    #[doc = "Bit 16 - ECC Fault Capture Interrupt Set Enable"]
    #[inline(always)]
    pub fn fltcap(&mut self) -> FltcapW<'_, IntensetSpec> {
        FltcapW::new(self, 16)
    }
}
#[doc = "Interrupt Enable SET Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0303;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
