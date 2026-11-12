#[doc = "Register `BDSTAT` reader"]
pub type R = crate::R<BdstatSpec>;
#[doc = "Field `BDCON` reader - This register field contains the current Descriptor control information. Bits \\[31:16\\] of BD_CTRL field of current descriptor are indicated in this register field."]
pub type BdconR = crate::FieldReader<u16>;
#[doc = "Field `DMAACTV` reader - A value of '1' would indicate that Buffer Descriptor Processor (BDP) is not Idle. Value of '0' would indicate that BDP is in IDLE state"]
pub type DmaactvR = crate::BitReader;
#[doc = "Field `DMASTART` reader - This register field would indicate whether DMA has started or not, if bit it set '1', then it indicates DMA start has happened. Value of '0' would indicate that DMA hasn't started."]
pub type DmastartR = crate::BitReader;
#[doc = "Field `BDSTATE` reader - This register field contains current BDP state : 0 IDLE, 1 Descriptor fetch Request Pending, 2 BD loading, 3 Data phase , 4 Descriptor Done, 5 Fetched BD is disabled, 6 Wait for Control Buffer Available, 7 Wait for AHB Error Acknowledgement"]
pub type BdstateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - This register field contains the current Descriptor control information. Bits \\[31:16\\] of BD_CTRL field of current descriptor are indicated in this register field."]
    #[inline(always)]
    pub fn bdcon(&self) -> BdconR {
        BdconR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - A value of '1' would indicate that Buffer Descriptor Processor (BDP) is not Idle. Value of '0' would indicate that BDP is in IDLE state"]
    #[inline(always)]
    pub fn dmaactv(&self) -> DmaactvR {
        DmaactvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This register field would indicate whether DMA has started or not, if bit it set '1', then it indicates DMA start has happened. Value of '0' would indicate that DMA hasn't started."]
    #[inline(always)]
    pub fn dmastart(&self) -> DmastartR {
        DmastartR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - This register field contains current BDP state : 0 IDLE, 1 Descriptor fetch Request Pending, 2 BD loading, 3 Data phase , 4 Descriptor Done, 5 Fetched BD is disabled, 6 Wait for Control Buffer Available, 7 Wait for AHB Error Acknowledgement"]
    #[inline(always)]
    pub fn bdstate(&self) -> BdstateR {
        BdstateR::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
#[doc = "BD_STATUS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdstatSpec;
impl crate::RegisterSpec for BdstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdstat::R`](R) reader structure"]
impl crate::Readable for BdstatSpec {}
#[doc = "`reset()` method sets BDSTAT to value 0"]
impl crate::Resettable for BdstatSpec {}
