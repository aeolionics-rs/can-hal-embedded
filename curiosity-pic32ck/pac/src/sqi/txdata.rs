#[doc = "Register `TXDATA` reader"]
pub type R = crate::R<TxdataSpec>;
#[doc = "Register `TXDATA` writer"]
pub type W = crate::W<TxdataSpec>;
#[doc = "Field `TXDATA` reader - Data is loaded into this register before being transmitted. Just prior to the beginning of a data transfer, the data in TX_DATA is loaded into the shift register. This register is used as a window to write data into Transmit buffer."]
pub type TxdataR = crate::FieldReader<u32>;
#[doc = "Field `TXDATA` writer - Data is loaded into this register before being transmitted. Just prior to the beginning of a data transfer, the data in TX_DATA is loaded into the shift register. This register is used as a window to write data into Transmit buffer."]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data is loaded into this register before being transmitted. Just prior to the beginning of a data transfer, the data in TX_DATA is loaded into the shift register. This register is used as a window to write data into Transmit buffer."]
    #[inline(always)]
    pub fn txdata(&self) -> TxdataR {
        TxdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data is loaded into this register before being transmitted. Just prior to the beginning of a data transfer, the data in TX_DATA is loaded into the shift register. This register is used as a window to write data into Transmit buffer."]
    #[inline(always)]
    pub fn txdata(&mut self) -> TxdataW<'_, TxdataSpec> {
        TxdataW::new(self, 0)
    }
}
#[doc = "SPI Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdataSpec;
impl crate::RegisterSpec for TxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata::R`](R) reader structure"]
impl crate::Readable for TxdataSpec {}
#[doc = "`write(|w| ..)` method takes [`txdata::W`](W) writer structure"]
impl crate::Writable for TxdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDATA to value 0"]
impl crate::Resettable for TxdataSpec {}
