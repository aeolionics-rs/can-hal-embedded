#[doc = "Register `RAMINFO` reader"]
pub type R = crate::R<RaminfoSpec>;
#[doc = "Field `RAMBITS` reader - width of the ram address bus"]
pub type RambitsR = crate::FieldReader;
#[doc = "Field `DMACHANS` reader - number of dma channels implemented in the design"]
pub type DmachansR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - width of the ram address bus"]
    #[inline(always)]
    pub fn rambits(&self) -> RambitsR {
        RambitsR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - number of dma channels implemented in the design"]
    #[inline(always)]
    pub fn dmachans(&self) -> DmachansR {
        DmachansR::new((self.bits >> 4) & 0x0f)
    }
}
#[doc = "Details About Width Of RAM and DMA Channels\n\nYou can [`read`](crate::Reg::read) this register and get [`raminfo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RaminfoSpec;
impl crate::RegisterSpec for RaminfoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`raminfo::R`](R) reader structure"]
impl crate::Readable for RaminfoSpec {}
#[doc = "`reset()` method sets RAMINFO to value 0x8d"]
impl crate::Resettable for RaminfoSpec {
    const RESET_VALUE: u8 = 0x8d;
}
