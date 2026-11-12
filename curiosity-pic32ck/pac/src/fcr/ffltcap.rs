#[doc = "Register `FFLTCAP` reader"]
pub type R = crate::R<FfltcapSpec>;
#[doc = "Field `FLTADR` reader - Fault capture Mode - Physical Address"]
pub type FltadrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - Fault capture Mode - Physical Address"]
    #[inline(always)]
    pub fn fltadr(&self) -> FltadrR {
        FltadrR::new(self.bits & 0x0fff_ffff)
    }
}
#[doc = "Flash Fault Capture Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltcap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfltcapSpec;
impl crate::RegisterSpec for FfltcapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffltcap::R`](R) reader structure"]
impl crate::Readable for FfltcapSpec {}
#[doc = "`reset()` method sets FFLTCAP to value 0"]
impl crate::Resettable for FfltcapSpec {}
