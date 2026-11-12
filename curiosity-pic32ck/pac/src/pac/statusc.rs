#[doc = "Register `STATUSC` reader"]
pub type R = crate::R<StatuscSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Peripheral write protection status - Bridge C\n\nYou can [`read`](crate::Reg::read) this register and get [`statusc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatuscSpec;
impl crate::RegisterSpec for StatuscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusc::R`](R) reader structure"]
impl crate::Readable for StatuscSpec {}
#[doc = "`reset()` method sets STATUSC to value 0"]
impl crate::Resettable for StatuscSpec {}
