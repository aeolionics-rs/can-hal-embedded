#[doc = "Register `BDCURADD` reader"]
pub type R = crate::R<BdcuraddSpec>;
#[doc = "Field `BDCURRADDR` reader - This register field contains the current descriptor address being processed by Buffer Descriptor Processor (BDP)"]
pub type BdcurraddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register field contains the current descriptor address being processed by Buffer Descriptor Processor (BDP)"]
    #[inline(always)]
    pub fn bdcurraddr(&self) -> BdcurraddrR {
        BdcurraddrR::new(self.bits)
    }
}
#[doc = "BD_CURR_ADDR Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcuradd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdcuraddSpec;
impl crate::RegisterSpec for BdcuraddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdcuradd::R`](R) reader structure"]
impl crate::Readable for BdcuraddSpec {}
#[doc = "`reset()` method sets BDCURADD to value 0"]
impl crate::Resettable for BdcuraddSpec {}
