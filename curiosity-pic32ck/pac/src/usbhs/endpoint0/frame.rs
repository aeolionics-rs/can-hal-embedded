#[doc = "Register `FRAME` reader"]
pub type R = crate::R<FrameSpec>;
#[doc = "Field `FRMNUM` reader - Frame Number"]
pub type FrmnumR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Frame Number"]
    #[inline(always)]
    pub fn frmnum(&self) -> FrmnumR {
        FrmnumR::new(self.bits & 0x07ff)
    }
}
#[doc = "Holds The Last Received Frame Number\n\nYou can [`read`](crate::Reg::read) this register and get [`frame::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameSpec;
impl crate::RegisterSpec for FrameSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`frame::R`](R) reader structure"]
impl crate::Readable for FrameSpec {}
#[doc = "`reset()` method sets FRAME to value 0"]
impl crate::Resettable for FrameSpec {}
