#[doc = "Register `SMC_WPSR` reader"]
pub type R = crate::R<SmcWpsrSpec>;
#[doc = "Field `WPVS` reader - Write Protection Violation Status"]
pub type WpvsR = crate::BitReader;
#[doc = "Field `WPVSRC` reader - Write Protection Violation Source"]
pub type WpvsrcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WpvsR {
        WpvsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:23 - Write Protection Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WpvsrcR {
        WpvsrcR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
#[doc = "SMC Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_wpsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcWpsrSpec;
impl crate::RegisterSpec for SmcWpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smc_wpsr::R`](R) reader structure"]
impl crate::Readable for SmcWpsrSpec {}
#[doc = "`reset()` method sets SMC_WPSR to value 0"]
impl crate::Resettable for SmcWpsrSpec {}
