#[doc = "Register `NONSECC` reader"]
pub type R = crate::R<NonseccSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Peripheral Non-Secure Status - Bridge C\n\nYou can [`read`](crate::Reg::read) this register and get [`nonsecc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NonseccSpec;
impl crate::RegisterSpec for NonseccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nonsecc::R`](R) reader structure"]
impl crate::Readable for NonseccSpec {}
#[doc = "`reset()` method sets NONSECC to value 0"]
impl crate::Resettable for NonseccSpec {}
