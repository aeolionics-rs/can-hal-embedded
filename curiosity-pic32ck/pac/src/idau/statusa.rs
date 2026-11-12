#[doc = "Register `STATUSA` reader"]
pub type R = crate::R<StatusaSpec>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `WLCK` reader - Write Lock"]
pub type WlckR = crate::BitReader;
#[doc = "Field `NBRG` reader - Number Of IDAU Regions"]
pub type NbrgR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Lock"]
    #[inline(always)]
    pub fn wlck(&self) -> WlckR {
        WlckR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:16 - Number Of IDAU Regions"]
    #[inline(always)]
    pub fn nbrg(&self) -> NbrgR {
        NbrgR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
#[doc = "Status A\n\nYou can [`read`](crate::Reg::read) this register and get [`statusa::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusaSpec;
impl crate::RegisterSpec for StatusaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusa::R`](R) reader structure"]
impl crate::Readable for StatusaSpec {}
#[doc = "`reset()` method sets STATUSA to value 0"]
impl crate::Resettable for StatusaSpec {}
