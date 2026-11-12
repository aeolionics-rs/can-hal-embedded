#[doc = "Register `RCAUSE` reader"]
pub type R = crate::R<RcauseSpec>;
#[doc = "Field `POR` reader - Power On Reset"]
pub type PorR = crate::BitReader;
#[doc = "Field `PORCORE` reader - Brown Out CORE Detector Reset"]
pub type PorcoreR = crate::BitReader;
#[doc = "Field `BORVDDREG` reader - Brown Out VDDREG Detector Reset"]
pub type BorvddregR = crate::BitReader;
#[doc = "Field `BORVDDA` reader - Brown Out VDDA Detector Reset"]
pub type BorvddaR = crate::BitReader;
#[doc = "Field `BORVDDIO` reader - Brown Out VDDIO Detector Reset"]
pub type BorvddioR = crate::BitReader;
#[doc = "Field `EXT` reader - External Reset"]
pub type ExtR = crate::BitReader;
#[doc = "Field `WDT` reader - Watchdog Reset"]
pub type WdtR = crate::BitReader;
#[doc = "Field `SYST` reader - System Reset Request"]
pub type SystR = crate::BitReader;
#[doc = "Field `BACKUP` reader - Backup Reset"]
pub type BackupR = crate::BitReader;
#[doc = "Field `LOCKUP` reader - Lockup Reset"]
pub type LockupR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Power On Reset"]
    #[inline(always)]
    pub fn por(&self) -> PorR {
        PorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Brown Out CORE Detector Reset"]
    #[inline(always)]
    pub fn porcore(&self) -> PorcoreR {
        PorcoreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Brown Out VDDREG Detector Reset"]
    #[inline(always)]
    pub fn borvddreg(&self) -> BorvddregR {
        BorvddregR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Brown Out VDDA Detector Reset"]
    #[inline(always)]
    pub fn borvdda(&self) -> BorvddaR {
        BorvddaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Brown Out VDDIO Detector Reset"]
    #[inline(always)]
    pub fn borvddio(&self) -> BorvddioR {
        BorvddioR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Reset"]
    #[inline(always)]
    pub fn ext(&self) -> ExtR {
        ExtR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watchdog Reset"]
    #[inline(always)]
    pub fn wdt(&self) -> WdtR {
        WdtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - System Reset Request"]
    #[inline(always)]
    pub fn syst(&self) -> SystR {
        SystR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Backup Reset"]
    #[inline(always)]
    pub fn backup(&self) -> BackupR {
        BackupR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Lockup Reset"]
    #[inline(always)]
    pub fn lockup(&self) -> LockupR {
        LockupR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Reset Cause\n\nYou can [`read`](crate::Reg::read) this register and get [`rcause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcauseSpec;
impl crate::RegisterSpec for RcauseSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rcause::R`](R) reader structure"]
impl crate::Readable for RcauseSpec {}
#[doc = "`reset()` method sets RCAUSE to value 0"]
impl crate::Resettable for RcauseSpec {}
