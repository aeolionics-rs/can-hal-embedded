#[doc = "Register `SECLOCKB` reader"]
pub type R = crate::R<SeclockbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Peripheral Security Lock Status - Bridge B\n\nYou can [`read`](crate::Reg::read) this register and get [`seclockb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeclockbSpec;
impl crate::RegisterSpec for SeclockbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seclockb::R`](R) reader structure"]
impl crate::Readable for SeclockbSpec {}
#[doc = "`reset()` method sets SECLOCKB to value 0"]
impl crate::Resettable for SeclockbSpec {}
