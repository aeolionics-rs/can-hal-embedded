#[doc = "Register `DFLLDIFF` reader"]
pub type R = crate::R<DflldiffSpec>;
#[doc = "Field `DIFF` reader - Multiplication Ratio Difference"]
pub type DiffR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Multiplication Ratio Difference"]
    #[inline(always)]
    pub fn diff(&self) -> DiffR {
        DiffR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DFLL48M Diff\n\nYou can [`read`](crate::Reg::read) this register and get [`dflldiff::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DflldiffSpec;
impl crate::RegisterSpec for DflldiffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dflldiff::R`](R) reader structure"]
impl crate::Readable for DflldiffSpec {}
#[doc = "`reset()` method sets DFLLDIFF to value 0"]
impl crate::Resettable for DflldiffSpec {}
