#[doc = "Register `CHLLCFGSTAT` reader"]
pub type R = crate::R<ChllcfgstatSpec>;
#[doc = "Field `CTRLB` reader - CTRLB Register Descriptor Load"]
pub type CtrlbR = crate::BitReader;
#[doc = "Field `EVCTRL` reader - EVCTRL Register Descriptor Load"]
pub type EvctrlR = crate::BitReader;
#[doc = "Field `SSA` reader - Source Start Address Descriptor Load"]
pub type SsaR = crate::BitReader;
#[doc = "Field `DSA` reader - Destination Start Address Descriptor Load"]
pub type DsaR = crate::BitReader;
#[doc = "Field `SSTRD` reader - Source Cell Stride Size Descriptor Load"]
pub type SstrdR = crate::BitReader;
#[doc = "Field `DSTRD` reader - Destination Cell Stride Size Descriptor Load"]
pub type DstrdR = crate::BitReader;
#[doc = "Field `XSIZ` reader - Transfer Size Descriptor Load"]
pub type XsizR = crate::BitReader;
#[doc = "Field `PDAT` reader - Match Pattern Descriptor Load"]
pub type PdatR = crate::BitReader;
#[doc = "Field `CTRLCRC` reader - Control CRC Descriptor Load"]
pub type CtrlcrcR = crate::BitReader;
#[doc = "Field `CRCDAT` reader - CRC Data Descriptor Load"]
pub type CrcdatR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CTRLB Register Descriptor Load"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CtrlbR {
        CtrlbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EVCTRL Register Descriptor Load"]
    #[inline(always)]
    pub fn evctrl(&self) -> EvctrlR {
        EvctrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source Start Address Descriptor Load"]
    #[inline(always)]
    pub fn ssa(&self) -> SsaR {
        SsaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination Start Address Descriptor Load"]
    #[inline(always)]
    pub fn dsa(&self) -> DsaR {
        DsaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Source Cell Stride Size Descriptor Load"]
    #[inline(always)]
    pub fn sstrd(&self) -> SstrdR {
        SstrdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Destination Cell Stride Size Descriptor Load"]
    #[inline(always)]
    pub fn dstrd(&self) -> DstrdR {
        DstrdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Size Descriptor Load"]
    #[inline(always)]
    pub fn xsiz(&self) -> XsizR {
        XsizR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Match Pattern Descriptor Load"]
    #[inline(always)]
    pub fn pdat(&self) -> PdatR {
        PdatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Control CRC Descriptor Load"]
    #[inline(always)]
    pub fn ctrlcrc(&self) -> CtrlcrcR {
        CtrlcrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CRC Data Descriptor Load"]
    #[inline(always)]
    pub fn crcdat(&self) -> CrcdatR {
        CrcdatR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "CHANNEL LINKED LIST CONFIGURATION STATUS REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chllcfgstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChllcfgstatSpec;
impl crate::RegisterSpec for ChllcfgstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chllcfgstat::R`](R) reader structure"]
impl crate::Readable for ChllcfgstatSpec {}
#[doc = "`reset()` method sets CHLLCFGSTAT to value 0"]
impl crate::Resettable for ChllcfgstatSpec {}
