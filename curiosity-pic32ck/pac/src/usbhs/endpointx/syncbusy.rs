#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - Software reset busy bit"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `ENABLE` reader - Enable busy bit"]
pub type EnableR = crate::BitReader;
#[doc = "Field `T1MSEN` reader - T1MS enable busy bit"]
pub type T1msenR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software reset busy bit"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable busy bit"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - T1MS enable busy bit"]
    #[inline(always)]
    pub fn t1msen(&self) -> T1msenR {
        T1msenR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "USBHS Syncbusy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
