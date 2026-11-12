#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `BOR` reader - BOR Synchronization Busy"]
pub type BorR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - BOR Synchronization Busy"]
    #[inline(always)]
    pub fn bor(&self) -> BorR {
        BorR::new((self.bits & 1) != 0)
    }
}
#[doc = "Synchronisation Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
