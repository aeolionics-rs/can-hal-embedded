#[doc = "Register `FFLTSYN` reader"]
pub type R = crate::R<FfltsynSpec>;
#[doc = "Field `SECSYN` reader - Single Error Correction Syndrome"]
pub type SecsynR = crate::FieldReader<u16>;
#[doc = "Field `DEDSYN` reader - DED Syndrome"]
pub type DedsynR = crate::BitReader;
#[doc = "Field `SERR` reader - Single Error Corrected"]
pub type SerrR = crate::BitReader;
#[doc = "Field `DERR` reader - Double Error Detected"]
pub type DerrR = crate::BitReader;
#[doc = "Field `CERR` reader - ECC Control bit Error"]
pub type CerrR = crate::BitReader;
#[doc = "Field `CTLSTAT` reader - Parity vs ECC Control Status"]
pub type CtlstatR = crate::FieldReader;
#[doc = "Field `PERR` reader - Per Word Parity Error Status"]
pub type PerrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:8 - Single Error Correction Syndrome"]
    #[inline(always)]
    pub fn secsyn(&self) -> SecsynR {
        SecsynR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - DED Syndrome"]
    #[inline(always)]
    pub fn dedsyn(&self) -> DedsynR {
        DedsynR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Single Error Corrected"]
    #[inline(always)]
    pub fn serr(&self) -> SerrR {
        SerrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Double Error Detected"]
    #[inline(always)]
    pub fn derr(&self) -> DerrR {
        DerrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ECC Control bit Error"]
    #[inline(always)]
    pub fn cerr(&self) -> CerrR {
        CerrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Parity vs ECC Control Status"]
    #[inline(always)]
    pub fn ctlstat(&self) -> CtlstatR {
        CtlstatR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:31 - Per Word Parity Error Status"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Flash ECC Fault Syndrome Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltsyn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfltsynSpec;
impl crate::RegisterSpec for FfltsynSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffltsyn::R`](R) reader structure"]
impl crate::Readable for FfltsynSpec {}
#[doc = "`reset()` method sets FFLTSYN to value 0"]
impl crate::Resettable for FfltsynSpec {}
