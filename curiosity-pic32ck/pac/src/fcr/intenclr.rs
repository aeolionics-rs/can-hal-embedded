#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `SERR` reader - Flash SEC Interrupt Clear Enable"]
pub type SerrR = crate::BitReader;
#[doc = "Field `SERR` writer - Flash SEC Interrupt Clear Enable"]
pub type SerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DERR` reader - ECC Double Error Detected Clear Enable"]
pub type DerrR = crate::BitReader;
#[doc = "Field `DERR` writer - ECC Double Error Detected Clear Enable"]
pub type DerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CRCDONE` reader - CRC Calculation Done Clear Enable"]
pub type CrcdoneR = crate::BitReader;
#[doc = "Field `CRCDONE` writer - CRC Calculation Done Clear Enable"]
pub type CrcdoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CRCERR` reader - CRC Error Clear Enable"]
pub type CrcerrR = crate::BitReader;
#[doc = "Field `CRCERR` writer - CRC Error Clear Enable"]
pub type CrcerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FLTCAP` reader - ECC Fault Capture Clear Enable"]
pub type FltcapR = crate::BitReader;
#[doc = "Field `FLTCAP` writer - ECC Fault Capture Clear Enable"]
pub type FltcapW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flash SEC Interrupt Clear Enable"]
    #[inline(always)]
    pub fn serr(&self) -> SerrR {
        SerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC Double Error Detected Clear Enable"]
    #[inline(always)]
    pub fn derr(&self) -> DerrR {
        DerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - CRC Calculation Done Clear Enable"]
    #[inline(always)]
    pub fn crcdone(&self) -> CrcdoneR {
        CrcdoneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CRC Error Clear Enable"]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - ECC Fault Capture Clear Enable"]
    #[inline(always)]
    pub fn fltcap(&self) -> FltcapR {
        FltcapR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash SEC Interrupt Clear Enable"]
    #[inline(always)]
    pub fn serr(&mut self) -> SerrW<'_, IntenclrSpec> {
        SerrW::new(self, 0)
    }
    #[doc = "Bit 1 - ECC Double Error Detected Clear Enable"]
    #[inline(always)]
    pub fn derr(&mut self) -> DerrW<'_, IntenclrSpec> {
        DerrW::new(self, 1)
    }
    #[doc = "Bit 8 - CRC Calculation Done Clear Enable"]
    #[inline(always)]
    pub fn crcdone(&mut self) -> CrcdoneW<'_, IntenclrSpec> {
        CrcdoneW::new(self, 8)
    }
    #[doc = "Bit 9 - CRC Error Clear Enable"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CrcerrW<'_, IntenclrSpec> {
        CrcerrW::new(self, 9)
    }
    #[doc = "Bit 16 - ECC Fault Capture Clear Enable"]
    #[inline(always)]
    pub fn fltcap(&mut self) -> FltcapW<'_, IntenclrSpec> {
        FltcapW::new(self, 16)
    }
}
#[doc = "Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0303;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
