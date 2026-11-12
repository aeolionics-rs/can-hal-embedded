#[doc = "Register `NONSECD` reader"]
pub type R = crate::R<NonsecdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Peripheral Non-Secure Status - Bridge D\n\nYou can [`read`](crate::Reg::read) this register and get [`nonsecd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NonsecdSpec;
impl crate::RegisterSpec for NonsecdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nonsecd::R`](R) reader structure"]
impl crate::Readable for NonsecdSpec {}
#[doc = "`reset()` method sets NONSECD to value 0"]
impl crate::Resettable for NonsecdSpec {}
