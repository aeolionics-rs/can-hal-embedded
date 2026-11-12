#[doc = "Register `RSTATUSB_BLOCK_MODE` reader"]
pub type R = crate::R<RstatusbBlockModeSpec>;
#[doc = "Field `NONSEC` reader - Block Based Region Non-Secure State"]
pub type NonsecR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Block Based Region Non-Secure State"]
    #[inline(always)]
    pub fn nonsec(&self) -> NonsecR {
        NonsecR::new(self.bits)
    }
}
#[doc = "Region Status B\n\nYou can [`read`](crate::Reg::read) this register and get [`rstatusb_block_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstatusbBlockModeSpec;
impl crate::RegisterSpec for RstatusbBlockModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstatusb_block_mode::R`](R) reader structure"]
impl crate::Readable for RstatusbBlockModeSpec {}
#[doc = "`reset()` method sets RSTATUSB_BLOCK_MODE to value 0"]
impl crate::Resettable for RstatusbBlockModeSpec {}
