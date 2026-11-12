#[doc = "Register `PERMR` reader"]
pub type R = crate::R<PermrSpec>;
#[doc = "Field `DATA` reader - Permutation Scrambler Data Output"]
pub type DataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Permutation Scrambler Data Output"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 7) as u8)
    }
}
#[doc = "Permutation Read\n\nYou can [`read`](crate::Reg::read) this register and get [`permr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PermrSpec;
impl crate::RegisterSpec for PermrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`permr::R`](R) reader structure"]
impl crate::Readable for PermrSpec {}
#[doc = "`reset()` method sets PERMR to value 0"]
impl crate::Resettable for PermrSpec {}
