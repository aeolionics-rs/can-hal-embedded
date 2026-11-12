#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RAMINV` reader - RAM Inversion Bit"]
pub type RaminvR = crate::BitReader;
#[doc = "Field `DRP` reader - Data Remanence Prevention Ongoing"]
pub type DrpR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RAM Inversion Bit"]
    #[inline(always)]
    pub fn raminv(&self) -> RaminvR {
        RaminvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Remanence Prevention Ongoing"]
    #[inline(always)]
    pub fn drp(&self) -> DrpR {
        DrpR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
