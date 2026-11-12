#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `DFLLENABLE` reader - DFLL48M ENABLE Synchronization Busy"]
pub type DfllenableR = crate::BitReader;
#[doc = "Field `DFLLCTRLB` reader - DFLLCTRLB Synchronization Busy"]
pub type DfllctrlbR = crate::BitReader;
#[doc = "Field `DFLLTUNE` reader - DFLLTUNE Synchronization Busy"]
pub type DflltuneR = crate::BitReader;
#[doc = "Field `DFLLDIFF` reader - DFLLDIFF Synchronization Busy"]
pub type DflldiffR = crate::BitReader;
#[doc = "Field `DFLLMUL` reader - DFLLMUL Synchronization Busy"]
pub type DfllmulR = crate::BitReader;
#[doc = "Field `FRACDIV0` reader - FRACDIV0 Synchronization Busy"]
pub type Fracdiv0R = crate::BitReader;
#[doc = "Field `FRACDIV1` reader - FRACDIV1 Synchronization Busy"]
pub type Fracdiv1R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - DFLL48M ENABLE Synchronization Busy"]
    #[inline(always)]
    pub fn dfllenable(&self) -> DfllenableR {
        DfllenableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DFLLCTRLB Synchronization Busy"]
    #[inline(always)]
    pub fn dfllctrlb(&self) -> DfllctrlbR {
        DfllctrlbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DFLLTUNE Synchronization Busy"]
    #[inline(always)]
    pub fn dflltune(&self) -> DflltuneR {
        DflltuneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DFLLDIFF Synchronization Busy"]
    #[inline(always)]
    pub fn dflldiff(&self) -> DflldiffR {
        DflldiffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DFLLMUL Synchronization Busy"]
    #[inline(always)]
    pub fn dfllmul(&self) -> DfllmulR {
        DfllmulR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FRACDIV0 Synchronization Busy"]
    #[inline(always)]
    pub fn fracdiv0(&self) -> Fracdiv0R {
        Fracdiv0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FRACDIV1 Synchronization Busy"]
    #[inline(always)]
    pub fn fracdiv1(&self) -> Fracdiv1R {
        Fracdiv1R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
