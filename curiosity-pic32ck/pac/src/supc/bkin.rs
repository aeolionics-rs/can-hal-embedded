#[doc = "Register `BKIN` reader"]
pub type R = crate::R<BkinSpec>;
#[doc = "Field `BKIN` reader - Backup Input Value"]
pub type BkinR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Backup Input Value"]
    #[inline(always)]
    pub fn bkin(&self) -> BkinR {
        BkinR::new((self.bits & 3) as u8)
    }
}
#[doc = "Backup Input Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bkin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BkinSpec;
impl crate::RegisterSpec for BkinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkin::R`](R) reader structure"]
impl crate::Readable for BkinSpec {}
#[doc = "`reset()` method sets BKIN to value 0"]
impl crate::Resettable for BkinSpec {}
