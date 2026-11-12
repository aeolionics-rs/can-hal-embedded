#[doc = "Register `SECLOCKA` reader"]
pub type R = crate::R<SeclockaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Peripheral Security Lock Status - Bridge A\n\nYou can [`read`](crate::Reg::read) this register and get [`seclocka::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeclockaSpec;
impl crate::RegisterSpec for SeclockaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seclocka::R`](R) reader structure"]
impl crate::Readable for SeclockaSpec {}
#[doc = "`reset()` method sets SECLOCKA to value 0"]
impl crate::Resettable for SeclockaSpec {}
