#[doc = "Register `SYNCB` reader"]
pub type R = crate::R<SyncbSpec>;
#[doc = "Field `SWRST` reader - Software Reset Sync Busy"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `ENABLE` reader - Enable Sync Busy"]
pub type EnableR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software Reset Sync Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Sync Busy"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Sync Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbSpec;
impl crate::RegisterSpec for SyncbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncb::R`](R) reader structure"]
impl crate::Readable for SyncbSpec {}
#[doc = "`reset()` method sets SYNCB to value 0"]
impl crate::Resettable for SyncbSpec {}
