#[doc = "Register `CONFIGDATA` reader"]
pub type R = crate::R<ConfigdataSpec>;
#[doc = "Field `UTMIDATAWIDTH` reader - Indicates Selected UTMI+ DataWidth. Always 0 Indicates 8 Bits"]
pub type UtmidatawidthR = crate::BitReader;
#[doc = "Field `SOFTCONE` reader - Indicates Soft Connect Or Disconnect"]
pub type SoftconeR = crate::BitReader;
#[doc = "Field `DYNFIFOSIZING` reader - Dynamic FIFO Sizing Is Selected Or Not"]
pub type DynfifosizingR = crate::BitReader;
#[doc = "Field `HBTXE` reader - Indicates High Bandwidth TX ISO Endpoint Support Selected"]
pub type HbtxeR = crate::BitReader;
#[doc = "Field `HBRXE` reader - Indicates High Bandwidth RX ISO Endpoint Support Selected"]
pub type HbrxeR = crate::BitReader;
#[doc = "Field `BIGENDIAN` reader - Indicates Little Endian Ordering"]
pub type BigendianR = crate::BitReader;
#[doc = "Field `MPTXE` reader - Automatic Splitting Of Bulk Packets Is Selected"]
pub type MptxeR = crate::BitReader;
#[doc = "Field `MPRXE` reader - Automatic Splitting Of Bulk Amalgamation Is Selected"]
pub type MprxeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates Selected UTMI+ DataWidth. Always 0 Indicates 8 Bits"]
    #[inline(always)]
    pub fn utmidatawidth(&self) -> UtmidatawidthR {
        UtmidatawidthR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates Soft Connect Or Disconnect"]
    #[inline(always)]
    pub fn softcone(&self) -> SoftconeR {
        SoftconeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dynamic FIFO Sizing Is Selected Or Not"]
    #[inline(always)]
    pub fn dynfifosizing(&self) -> DynfifosizingR {
        DynfifosizingR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates High Bandwidth TX ISO Endpoint Support Selected"]
    #[inline(always)]
    pub fn hbtxe(&self) -> HbtxeR {
        HbtxeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates High Bandwidth RX ISO Endpoint Support Selected"]
    #[inline(always)]
    pub fn hbrxe(&self) -> HbrxeR {
        HbrxeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates Little Endian Ordering"]
    #[inline(always)]
    pub fn bigendian(&self) -> BigendianR {
        BigendianR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic Splitting Of Bulk Packets Is Selected"]
    #[inline(always)]
    pub fn mptxe(&self) -> MptxeR {
        MptxeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Automatic Splitting Of Bulk Amalgamation Is Selected"]
    #[inline(always)]
    pub fn mprxe(&self) -> MprxeR {
        MprxeR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "ENDPOINT0 Returns Information About The Selected Core Configuration. Only Applicable when INDEX Regsiter Is 0. That Is Endpoint0 Is Selected.\n\nYou can [`read`](crate::Reg::read) this register and get [`configdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigdataSpec;
impl crate::RegisterSpec for ConfigdataSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`configdata::R`](R) reader structure"]
impl crate::Readable for ConfigdataSpec {}
#[doc = "`reset()` method sets CONFIGDATA to value 0"]
impl crate::Resettable for ConfigdataSpec {}
