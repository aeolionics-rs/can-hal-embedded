#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RXBUFELM` reader - Receive Buffer Element Count bits"]
pub type RxbufelmR = crate::FieldReader<u16>;
#[doc = "Field `SRMT` reader - Register (SR) Empty bit"]
pub type SrmtR = crate::BitReader;
#[doc = "Field `SPIBUSY` reader - SPI activity status bit"]
pub type SpibusyR = crate::BitReader;
#[doc = "Field `TXBUFELM` reader - Transmit Buffer Element Count bits"]
pub type TxbufelmR = crate::FieldReader<u16>;
#[doc = "Field `SPITBF` reader - SPI Transmit Buffer Full Status bit"]
pub type SpitbfR = crate::BitReader;
#[doc = "Field `SPITBE` reader - SPI Transmit Buffer Empty status bit"]
pub type SpitbeR = crate::BitReader;
#[doc = "Field `SPIRBF` reader - SPI Receive Buffer Full status bit"]
pub type SpirbfR = crate::BitReader;
#[doc = "Field `SPIRBE` reader - RX Buffer Empty bit"]
pub type SpirbeR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - Receive Buffer Element Count bits"]
    #[inline(always)]
    pub fn rxbufelm(&self) -> RxbufelmR {
        RxbufelmR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - Register (SR) Empty bit"]
    #[inline(always)]
    pub fn srmt(&self) -> SrmtR {
        SrmtR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI activity status bit"]
    #[inline(always)]
    pub fn spibusy(&self) -> SpibusyR {
        SpibusyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:24 - Transmit Buffer Element Count bits"]
    #[inline(always)]
    pub fn txbufelm(&self) -> TxbufelmR {
        TxbufelmR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 26 - SPI Transmit Buffer Full Status bit"]
    #[inline(always)]
    pub fn spitbf(&self) -> SpitbfR {
        SpitbfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - SPI Transmit Buffer Empty status bit"]
    #[inline(always)]
    pub fn spitbe(&self) -> SpitbeR {
        SpitbeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SPI Receive Buffer Full status bit"]
    #[inline(always)]
    pub fn spirbf(&self) -> SpirbfR {
        SpirbfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - RX Buffer Empty bit"]
    #[inline(always)]
    pub fn spirbe(&self) -> SpirbeR {
        SpirbeR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SPI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x9000_2000"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x9000_2000;
}
