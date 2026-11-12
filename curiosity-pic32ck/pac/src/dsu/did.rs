#[doc = "Register `DID` reader"]
pub type R = crate::R<DidSpec>;
#[doc = "Field `MARKER` reader - Marker Bit"]
pub type MarkerR = crate::BitReader;
#[doc = "Field `MANID` reader - Manufacturer ID"]
pub type ManidR = crate::FieldReader<u16>;
#[doc = "Field `DEVSEL` reader - Device Select"]
pub type DevselR = crate::FieldReader;
#[doc = "Field `PRODUCT` reader - Product"]
pub type ProductR = crate::FieldReader;
#[doc = "Field `REVISION` reader - Revision"]
pub type RevisionR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Marker Bit"]
    #[inline(always)]
    pub fn marker(&self) -> MarkerR {
        MarkerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:11 - Manufacturer ID"]
    #[inline(always)]
    pub fn manid(&self) -> ManidR {
        ManidR::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    #[doc = "Bits 12:19 - Device Select"]
    #[inline(always)]
    pub fn devsel(&self) -> DevselR {
        DevselR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:27 - Product"]
    #[inline(always)]
    pub fn product(&self) -> ProductR {
        ProductR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - Revision"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Device Identification\n\nYou can [`read`](crate::Reg::read) this register and get [`did::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DidSpec;
impl crate::RegisterSpec for DidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`did::R`](R) reader structure"]
impl crate::Readable for DidSpec {}
#[doc = "`reset()` method sets DID to value 0x0950_6053"]
impl crate::Resettable for DidSpec {
    const RESET_VALUE: u32 = 0x0950_6053;
}
