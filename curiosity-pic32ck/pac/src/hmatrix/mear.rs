#[doc = "Register `MEAR[%s]` reader"]
pub type R = crate::R<MearSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Master Error Address\n\nYou can [`read`](crate::Reg::read) this register and get [`mear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MearSpec;
impl crate::RegisterSpec for MearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mear::R`](R) reader structure"]
impl crate::Readable for MearSpec {}
#[doc = "`reset()` method sets MEAR[%s] to value 0"]
impl crate::Resettable for MearSpec {}
