#[doc = "Register `NONSECB` reader"]
pub type R = crate::R<NonsecbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Peripheral Non-Secure Status - Bridge B\n\nYou can [`read`](crate::Reg::read) this register and get [`nonsecb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NonsecbSpec;
impl crate::RegisterSpec for NonsecbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nonsecb::R`](R) reader structure"]
impl crate::Readable for NonsecbSpec {}
#[doc = "`reset()` method sets NONSECB to value 0"]
impl crate::Resettable for NonsecbSpec {}
