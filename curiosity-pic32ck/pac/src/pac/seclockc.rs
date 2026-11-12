#[doc = "Register `SECLOCKC` reader"]
pub type R = crate::R<SeclockcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Peripheral Security Lock Status - Bridge C\n\nYou can [`read`](crate::Reg::read) this register and get [`seclockc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeclockcSpec;
impl crate::RegisterSpec for SeclockcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seclockc::R`](R) reader structure"]
impl crate::Readable for SeclockcSpec {}
#[doc = "`reset()` method sets SECLOCKC to value 0"]
impl crate::Resettable for SeclockcSpec {}
