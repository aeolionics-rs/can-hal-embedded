#[doc = "Register `FFLTPAR` reader"]
pub type R = crate::R<FfltparSpec>;
#[doc = "Field `SECIN` reader - The Single Error Parity bits from Flash"]
pub type SecinR = crate::FieldReader<u16>;
#[doc = "Field `DEDIN` reader - The Overall Parity from Flash"]
pub type DedinR = crate::BitReader;
#[doc = "Field `SECOUT` reader - The Calculated Single Error Parity bits"]
pub type SecoutR = crate::FieldReader<u16>;
#[doc = "Field `DEDOUT` reader - The Calculated Overall Parity used in Double Error Detection"]
pub type DedoutR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - The Single Error Parity bits from Flash"]
    #[inline(always)]
    pub fn secin(&self) -> SecinR {
        SecinR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - The Overall Parity from Flash"]
    #[inline(always)]
    pub fn dedin(&self) -> DedinR {
        DedinR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - The Calculated Single Error Parity bits"]
    #[inline(always)]
    pub fn secout(&self) -> SecoutR {
        SecoutR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - The Calculated Overall Parity used in Double Error Detection"]
    #[inline(always)]
    pub fn dedout(&self) -> DedoutR {
        DedoutR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Flash ECC Fault Parity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltpar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfltparSpec;
impl crate::RegisterSpec for FfltparSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffltpar::R`](R) reader structure"]
impl crate::Readable for FfltparSpec {}
#[doc = "`reset()` method sets FFLTPAR to value 0"]
impl crate::Resettable for FfltparSpec {}
