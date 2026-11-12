#[doc = "Register `SECLOCKD` reader"]
pub type R = crate::R<SeclockdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Peripheral Security Lock Status - Bridge D\n\nYou can [`read`](crate::Reg::read) this register and get [`seclockd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeclockdSpec;
impl crate::RegisterSpec for SeclockdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seclockd::R`](R) reader structure"]
impl crate::Readable for SeclockdSpec {}
#[doc = "`reset()` method sets SECLOCKD to value 0"]
impl crate::Resettable for SeclockdSpec {}
