#[doc = "Register `PFFDATA` reader"]
pub type R = crate::R<PffdataSpec>;
#[doc = "Field `PFFDATA` reader - SARCORE Conversion data from the APB FIFO"]
pub type PffdataR = crate::FieldReader<u16>;
#[doc = "Field `PFFCHNID` reader - Channel ID from APB FIFO"]
pub type PffchnidR = crate::FieldReader;
#[doc = "Field `PFFCORID` reader - Core ID from APB FIFO"]
pub type PffcoridR = crate::FieldReader;
#[doc = "Field `PFFSIGN` reader - Channel Sign from the APB FIFO"]
pub type PffsignR = crate::BitReader;
#[doc = "Field `PFFFRACT` reader - Fractional Setting from APB FIFO"]
pub type PfffractR = crate::BitReader;
#[doc = "Field `PFFCNT` reader - Current Data Entries in APB FIFO"]
pub type PffcntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - SARCORE Conversion data from the APB FIFO"]
    #[inline(always)]
    pub fn pffdata(&self) -> PffdataR {
        PffdataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Channel ID from APB FIFO"]
    #[inline(always)]
    pub fn pffchnid(&self) -> PffchnidR {
        PffchnidR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Core ID from APB FIFO"]
    #[inline(always)]
    pub fn pffcorid(&self) -> PffcoridR {
        PffcoridR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Channel Sign from the APB FIFO"]
    #[inline(always)]
    pub fn pffsign(&self) -> PffsignR {
        PffsignR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Fractional Setting from APB FIFO"]
    #[inline(always)]
    pub fn pfffract(&self) -> PfffractR {
        PfffractR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Current Data Entries in APB FIFO"]
    #[inline(always)]
    pub fn pffcnt(&self) -> PffcntR {
        PffcntR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "APB FIFO Output Data\n\nYou can [`read`](crate::Reg::read) this register and get [`pffdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PffdataSpec;
impl crate::RegisterSpec for PffdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pffdata::R`](R) reader structure"]
impl crate::Readable for PffdataSpec {}
#[doc = "`reset()` method sets PFFDATA to value 0"]
impl crate::Resettable for PffdataSpec {}
