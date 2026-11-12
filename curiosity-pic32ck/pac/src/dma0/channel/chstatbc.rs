#[doc = "Register `CHSTATBC` reader"]
pub type R = crate::R<ChstatbcSpec>;
#[doc = "Field `BBTC` reader - Bytes Transfered in the Block Counter"]
pub type BbtcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - Bytes Transfered in the Block Counter"]
    #[inline(always)]
    pub fn bbtc(&self) -> BbtcR {
        BbtcR::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "CHANNEL STATUS BLOCK COUNT REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatbc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChstatbcSpec;
impl crate::RegisterSpec for ChstatbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chstatbc::R`](R) reader structure"]
impl crate::Readable for ChstatbcSpec {}
#[doc = "`reset()` method sets CHSTATBC to value 0"]
impl crate::Resettable for ChstatbcSpec {}
