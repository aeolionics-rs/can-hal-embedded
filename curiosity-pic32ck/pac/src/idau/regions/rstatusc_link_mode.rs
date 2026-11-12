#[doc = "Register `RSTATUSC_LINK_MODE` reader"]
pub type R = crate::R<RstatuscLinkModeSpec>;
#[doc = "Field `RGN` reader - Linked IDAU Region ID"]
pub type RgnR = crate::FieldReader;
#[doc = "Field `BLK` reader - Linked IDAU Region Block ID"]
pub type BlkR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Linked IDAU Region ID"]
    #[inline(always)]
    pub fn rgn(&self) -> RgnR {
        RgnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Linked IDAU Region Block ID"]
    #[inline(always)]
    pub fn blk(&self) -> BlkR {
        BlkR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[doc = "Region Status C\n\nYou can [`read`](crate::Reg::read) this register and get [`rstatusc_link_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstatuscLinkModeSpec;
impl crate::RegisterSpec for RstatuscLinkModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstatusc_link_mode::R`](R) reader structure"]
impl crate::Readable for RstatuscLinkModeSpec {}
#[doc = "`reset()` method sets RSTATUSC_LINK_MODE to value 0"]
impl crate::Resettable for RstatuscLinkModeSpec {}
