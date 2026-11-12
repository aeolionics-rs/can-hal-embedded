#[doc = "Register `RSTATUSB_WATERMARK_MODE` reader"]
pub type R = crate::R<RstatusbWatermarkModeSpec>;
#[doc = "Field `SIZE` reader - Watermark Based Region Size in Bytes"]
pub type SizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Watermark Based Region Size in Bytes"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(self.bits)
    }
}
#[doc = "Region Status B\n\nYou can [`read`](crate::Reg::read) this register and get [`rstatusb_watermark_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstatusbWatermarkModeSpec;
impl crate::RegisterSpec for RstatusbWatermarkModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstatusb_watermark_mode::R`](R) reader structure"]
impl crate::Readable for RstatusbWatermarkModeSpec {}
#[doc = "`reset()` method sets RSTATUSB_WATERMARK_MODE to value 0"]
impl crate::Resettable for RstatusbWatermarkModeSpec {}
