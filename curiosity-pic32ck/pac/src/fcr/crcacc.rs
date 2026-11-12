#[doc = "Register `CRCACC` reader"]
pub type R = crate::R<CrcaccSpec>;
#[doc = "Field `CRCACC` reader - CRC Accumulator Result"]
pub type CrcaccR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Accumulator Result"]
    #[inline(always)]
    pub fn crcacc(&self) -> CrcaccR {
        CrcaccR::new(self.bits)
    }
}
#[doc = "CRC Accumulator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcacc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcaccSpec;
impl crate::RegisterSpec for CrcaccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcacc::R`](R) reader structure"]
impl crate::Readable for CrcaccSpec {}
#[doc = "`reset()` method sets CRCACC to value 0"]
impl crate::Resettable for CrcaccSpec {}
