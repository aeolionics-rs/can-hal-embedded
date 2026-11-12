#[doc = "Register `INTTHR` reader"]
pub type R = crate::R<IntthrSpec>;
#[doc = "Register `INTTHR` writer"]
pub type W = crate::W<IntthrSpec>;
#[doc = "Field `RXINTTHR` reader - Receive Interrupt is set when Receive FIFO CNT is larger than or equal to RX_INT_THRES Value."]
pub type RxintthrR = crate::FieldReader;
#[doc = "Field `RXINTTHR` writer - Receive Interrupt is set when Receive FIFO CNT is larger than or equal to RX_INT_THRES Value."]
pub type RxintthrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXINTTHR` reader - Transmit Interrupt is set when Transmit FIFO has equal or more space than TX_INT_THRES bytes."]
pub type TxintthrR = crate::FieldReader;
#[doc = "Field `TXINTTHR` writer - Transmit Interrupt is set when Transmit FIFO has equal or more space than TX_INT_THRES bytes."]
pub type TxintthrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive Interrupt is set when Receive FIFO CNT is larger than or equal to RX_INT_THRES Value."]
    #[inline(always)]
    pub fn rxintthr(&self) -> RxintthrR {
        RxintthrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit Interrupt is set when Transmit FIFO has equal or more space than TX_INT_THRES bytes."]
    #[inline(always)]
    pub fn txintthr(&self) -> TxintthrR {
        TxintthrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Interrupt is set when Receive FIFO CNT is larger than or equal to RX_INT_THRES Value."]
    #[inline(always)]
    pub fn rxintthr(&mut self) -> RxintthrW<'_, IntthrSpec> {
        RxintthrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Transmit Interrupt is set when Transmit FIFO has equal or more space than TX_INT_THRES bytes."]
    #[inline(always)]
    pub fn txintthr(&mut self) -> TxintthrW<'_, IntthrSpec> {
        TxintthrW::new(self, 8)
    }
}
#[doc = "SPI Interrupt Threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`intthr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intthr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntthrSpec;
impl crate::RegisterSpec for IntthrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intthr::R`](R) reader structure"]
impl crate::Readable for IntthrSpec {}
#[doc = "`write(|w| ..)` method takes [`intthr::W`](W) writer structure"]
impl crate::Writable for IntthrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTTHR to value 0"]
impl crate::Resettable for IntthrSpec {}
