#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `PHYRDY` reader - PHY Is Ready for USBCORE Activity"]
pub type PhyrdyR = crate::BitReader;
#[doc = "Field `PHYON` reader - Power On of PHY Complete"]
pub type PhyonR = crate::BitReader;
#[doc = "Field `VREGRDY` reader - USB Voltage Regulator Status"]
pub type VregrdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PHY Is Ready for USBCORE Activity"]
    #[inline(always)]
    pub fn phyrdy(&self) -> PhyrdyR {
        PhyrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power On of PHY Complete"]
    #[inline(always)]
    pub fn phyon(&self) -> PhyonR {
        PhyonR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB Voltage Regulator Status"]
    #[inline(always)]
    pub fn vregrdy(&self) -> VregrdyR {
        VregrdyR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "USBHS Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
