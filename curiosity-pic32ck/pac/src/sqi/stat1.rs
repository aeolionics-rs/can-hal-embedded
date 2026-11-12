#[doc = "Register `STAT1` reader"]
pub type R = crate::R<Stat1Spec>;
#[doc = "Field `RXBUFCNT` reader - Number of Words of Read Data in the FIFO."]
pub type RxbufcntR = crate::FieldReader<u16>;
#[doc = "Field `TXBUFFREE` reader - Number of Words of Space available in the TX FIFO. Max value equals value of MEM_SIZE_BYTES parameter. Default value is Max"]
pub type TxbuffreeR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Number of Words of Read Data in the FIFO."]
    #[inline(always)]
    pub fn rxbufcnt(&self) -> RxbufcntR {
        RxbufcntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Number of Words of Space available in the TX FIFO. Max value equals value of MEM_SIZE_BYTES parameter. Default value is Max"]
    #[inline(always)]
    pub fn txbuffree(&self) -> TxbuffreeR {
        TxbuffreeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SPI Status1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stat1Spec;
impl crate::RegisterSpec for Stat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat1::R`](R) reader structure"]
impl crate::Readable for Stat1Spec {}
#[doc = "`reset()` method sets STAT1 to value 0x0100_0000"]
impl crate::Resettable for Stat1Spec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
