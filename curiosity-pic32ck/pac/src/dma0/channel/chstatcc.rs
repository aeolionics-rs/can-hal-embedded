#[doc = "Register `CHSTATCC` reader"]
pub type R = crate::R<ChstatccSpec>;
#[doc = "Field `CBTC` reader - Bytes Transfered in the Cell Counter"]
pub type CbtcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Bytes Transfered in the Cell Counter"]
    #[inline(always)]
    pub fn cbtc(&self) -> CbtcR {
        CbtcR::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "CHANNEL STATUS CELL COUNT REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatcc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChstatccSpec;
impl crate::RegisterSpec for ChstatccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chstatcc::R`](R) reader structure"]
impl crate::Readable for ChstatccSpec {}
#[doc = "`reset()` method sets CHSTATCC to value 0"]
impl crate::Resettable for ChstatccSpec {}
