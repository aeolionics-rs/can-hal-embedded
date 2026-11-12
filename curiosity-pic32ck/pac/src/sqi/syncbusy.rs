#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Software Reset Synchronization Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrstselect {
    #[doc = "0: SWRST synchronization is not busy"]
    Low = 0,
    #[doc = "1: SWRST synchronization is busy"]
    High = 1,
}
impl From<Swrstselect> for bool {
    #[inline(always)]
    fn from(variant: Swrstselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` reader - Software Reset Synchronization Busy"]
pub type SwrstR = crate::BitReader<Swrstselect>;
impl SwrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrstselect {
        match self.bits {
            false => Swrstselect::Low,
            true => Swrstselect::High,
        }
    }
    #[doc = "SWRST synchronization is not busy"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Swrstselect::Low
    }
    #[doc = "SWRST synchronization is busy"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Swrstselect::High
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset Synchronization Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
}
#[doc = "Syncbusy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
