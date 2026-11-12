#[doc = "Register `STATUSD` reader"]
pub type R = crate::R<StatusdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Peripheral write protection status - Bridge D\n\nYou can [`read`](crate::Reg::read) this register and get [`statusd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusdSpec;
impl crate::RegisterSpec for StatusdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusd::R`](R) reader structure"]
impl crate::Readable for StatusdSpec {}
#[doc = "`reset()` method sets STATUSD to value 0"]
impl crate::Resettable for StatusdSpec {}
