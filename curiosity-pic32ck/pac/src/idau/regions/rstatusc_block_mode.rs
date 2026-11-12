#[doc = "Register `RSTATUSC_BLOCK_MODE` reader"]
pub type R = crate::R<RstatuscBlockModeSpec>;
#[doc = "Field `CST` reader - Constant Block Configuration"]
pub type CstR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Constant Block Configuration"]
    #[inline(always)]
    pub fn cst(&self) -> CstR {
        CstR::new(self.bits)
    }
}
#[doc = "Region Status C\n\nYou can [`read`](crate::Reg::read) this register and get [`rstatusc_block_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstatuscBlockModeSpec;
impl crate::RegisterSpec for RstatuscBlockModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstatusc_block_mode::R`](R) reader structure"]
impl crate::Readable for RstatuscBlockModeSpec {}
#[doc = "`reset()` method sets RSTATUSC_BLOCK_MODE to value 0"]
impl crate::Resettable for RstatuscBlockModeSpec {}
