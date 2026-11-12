#[doc = "Register `NONSECA` reader"]
pub type R = crate::R<NonsecaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Peripheral Non-Secure Status - Bridge A\n\nYou can [`read`](crate::Reg::read) this register and get [`nonseca::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NonsecaSpec;
impl crate::RegisterSpec for NonsecaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nonseca::R`](R) reader structure"]
impl crate::Readable for NonsecaSpec {}
#[doc = "`reset()` method sets NONSECA to value 0"]
impl crate::Resettable for NonsecaSpec {}
