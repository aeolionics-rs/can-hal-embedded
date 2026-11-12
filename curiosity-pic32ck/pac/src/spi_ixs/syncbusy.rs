#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRSTBSY` reader - Software reset busy bit --- Synchronizing Busy bit for swrst"]
pub type SwrstbsyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software reset busy bit --- Synchronizing Busy bit for swrst"]
    #[inline(always)]
    pub fn swrstbsy(&self) -> SwrstbsyR {
        SwrstbsyR::new((self.bits & 1) != 0)
    }
}
#[doc = "SPI Sync Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
