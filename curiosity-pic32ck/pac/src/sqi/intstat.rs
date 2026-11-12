#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Register `INTSTAT` writer"]
pub type W = crate::W<IntstatSpec>;
#[doc = "Field `TXEMPTYIF` reader - TX FIFO Empty Interrupt."]
pub type TxemptyifR = crate::BitReader;
#[doc = "Field `TXEMPTYIF` writer - TX FIFO Empty Interrupt."]
pub type TxemptyifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFULLIF` reader - TX FIFO Full Interrupt."]
pub type TxfullifR = crate::BitReader;
#[doc = "Field `TXFULLIF` writer - TX FIFO Full Interrupt."]
pub type TxfullifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTHRIF` reader - Interrupt to be generated when the TX FIFO has space equal to or more than TX_INT_THRES words."]
pub type TxthrifR = crate::BitReader;
#[doc = "Field `TXTHRIF` writer - Interrupt to be generated when the TX FIFO has space equal to or more than TX_INT_THRES words."]
pub type TxthrifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEMPTYIF` reader - RX FIFO Empty Interrupt."]
pub type RxemptyifR = crate::BitReader;
#[doc = "Field `RXEMPTYIF` writer - RX FIFO Empty Interrupt."]
pub type RxemptyifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULLIF` reader - RX FIFO Full Interrupt."]
pub type RxfullifR = crate::BitReader;
#[doc = "Field `RXFULLIF` writer - RX FIFO Full Interrupt."]
pub type RxfullifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHRIF` reader - Interrupt to be generated when the RX FIFO has data more than or equal to RX_INT_THRES words. In case of Boot/XIP mode, the power on reset value of RX buffer threshold is zero. So this bit will be set to 1, immediately after power on reset, till a read request on the AHB Bus."]
pub type RxthrifR = crate::BitReader;
#[doc = "Field `RXTHRIF` writer - Interrupt to be generated when the RX FIFO has data more than or equal to RX_INT_THRES words. In case of Boot/XIP mode, the power on reset value of RX buffer threshold is zero. So this bit will be set to 1, immediately after power on reset, till a read request on the AHB Bus."]
pub type RxthrifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONFULLIF` reader - Control Buffer Full Interrupt"]
pub type ConfullifR = crate::BitReader;
#[doc = "Field `CONFULLIF` writer - Control Buffer Full Interrupt"]
pub type ConfullifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONEMPTYIF` reader - Control Buffer Empty Interrupt"]
pub type ConemptyifR = crate::BitReader;
#[doc = "Field `CONEMPTYIF` writer - Control Buffer Empty Interrupt"]
pub type ConemptyifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTHRIF` reader - Control Buffer Threshold Interrupt"]
pub type ConthrifR = crate::BitReader;
#[doc = "Field `CONTHRIF` writer - Control Buffer Threshold Interrupt"]
pub type ConthrifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDDONEIF` reader - BDP sets this bit to '1' after current buffer descriptor is processed"]
pub type BddoneifR = crate::BitReader;
#[doc = "Field `BDDONEIF` writer - BDP sets this bit to '1' after current buffer descriptor is processed"]
pub type BddoneifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTCOMPIF` reader - Packet completion Interrupt"]
pub type PktcompifR = crate::BitReader;
#[doc = "Field `PKTCOMPIF` writer - Packet completion Interrupt"]
pub type PktcompifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEIF` reader - Master Error Interrupt"]
pub type DmaeifR = crate::BitReader;
#[doc = "Field `DMAEIF` writer - Master Error Interrupt"]
pub type DmaeifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX FIFO Empty Interrupt."]
    #[inline(always)]
    pub fn txemptyif(&self) -> TxemptyifR {
        TxemptyifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Full Interrupt."]
    #[inline(always)]
    pub fn txfullif(&self) -> TxfullifR {
        TxfullifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt to be generated when the TX FIFO has space equal to or more than TX_INT_THRES words."]
    #[inline(always)]
    pub fn txthrif(&self) -> TxthrifR {
        TxthrifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Empty Interrupt."]
    #[inline(always)]
    pub fn rxemptyif(&self) -> RxemptyifR {
        RxemptyifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Full Interrupt."]
    #[inline(always)]
    pub fn rxfullif(&self) -> RxfullifR {
        RxfullifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt to be generated when the RX FIFO has data more than or equal to RX_INT_THRES words. In case of Boot/XIP mode, the power on reset value of RX buffer threshold is zero. So this bit will be set to 1, immediately after power on reset, till a read request on the AHB Bus."]
    #[inline(always)]
    pub fn rxthrif(&self) -> RxthrifR {
        RxthrifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Control Buffer Full Interrupt"]
    #[inline(always)]
    pub fn confullif(&self) -> ConfullifR {
        ConfullifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Control Buffer Empty Interrupt"]
    #[inline(always)]
    pub fn conemptyif(&self) -> ConemptyifR {
        ConemptyifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Control Buffer Threshold Interrupt"]
    #[inline(always)]
    pub fn conthrif(&self) -> ConthrifR {
        ConthrifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BDP sets this bit to '1' after current buffer descriptor is processed"]
    #[inline(always)]
    pub fn bddoneif(&self) -> BddoneifR {
        BddoneifR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Packet completion Interrupt"]
    #[inline(always)]
    pub fn pktcompif(&self) -> PktcompifR {
        PktcompifR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Error Interrupt"]
    #[inline(always)]
    pub fn dmaeif(&self) -> DmaeifR {
        DmaeifR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Empty Interrupt."]
    #[inline(always)]
    pub fn txemptyif(&mut self) -> TxemptyifW<'_, IntstatSpec> {
        TxemptyifW::new(self, 0)
    }
    #[doc = "Bit 1 - TX FIFO Full Interrupt."]
    #[inline(always)]
    pub fn txfullif(&mut self) -> TxfullifW<'_, IntstatSpec> {
        TxfullifW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt to be generated when the TX FIFO has space equal to or more than TX_INT_THRES words."]
    #[inline(always)]
    pub fn txthrif(&mut self) -> TxthrifW<'_, IntstatSpec> {
        TxthrifW::new(self, 2)
    }
    #[doc = "Bit 3 - RX FIFO Empty Interrupt."]
    #[inline(always)]
    pub fn rxemptyif(&mut self) -> RxemptyifW<'_, IntstatSpec> {
        RxemptyifW::new(self, 3)
    }
    #[doc = "Bit 4 - RX FIFO Full Interrupt."]
    #[inline(always)]
    pub fn rxfullif(&mut self) -> RxfullifW<'_, IntstatSpec> {
        RxfullifW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt to be generated when the RX FIFO has data more than or equal to RX_INT_THRES words. In case of Boot/XIP mode, the power on reset value of RX buffer threshold is zero. So this bit will be set to 1, immediately after power on reset, till a read request on the AHB Bus."]
    #[inline(always)]
    pub fn rxthrif(&mut self) -> RxthrifW<'_, IntstatSpec> {
        RxthrifW::new(self, 5)
    }
    #[doc = "Bit 6 - Control Buffer Full Interrupt"]
    #[inline(always)]
    pub fn confullif(&mut self) -> ConfullifW<'_, IntstatSpec> {
        ConfullifW::new(self, 6)
    }
    #[doc = "Bit 7 - Control Buffer Empty Interrupt"]
    #[inline(always)]
    pub fn conemptyif(&mut self) -> ConemptyifW<'_, IntstatSpec> {
        ConemptyifW::new(self, 7)
    }
    #[doc = "Bit 8 - Control Buffer Threshold Interrupt"]
    #[inline(always)]
    pub fn conthrif(&mut self) -> ConthrifW<'_, IntstatSpec> {
        ConthrifW::new(self, 8)
    }
    #[doc = "Bit 9 - BDP sets this bit to '1' after current buffer descriptor is processed"]
    #[inline(always)]
    pub fn bddoneif(&mut self) -> BddoneifW<'_, IntstatSpec> {
        BddoneifW::new(self, 9)
    }
    #[doc = "Bit 10 - Packet completion Interrupt"]
    #[inline(always)]
    pub fn pktcompif(&mut self) -> PktcompifW<'_, IntstatSpec> {
        PktcompifW::new(self, 10)
    }
    #[doc = "Bit 11 - Master Error Interrupt"]
    #[inline(always)]
    pub fn dmaeif(&mut self) -> DmaeifW<'_, IntstatSpec> {
        DmaeifW::new(self, 11)
    }
}
#[doc = "SPI Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intstat::W`](W) writer structure"]
impl crate::Writable for IntstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSTAT to value 0xad"]
impl crate::Resettable for IntstatSpec {
    const RESET_VALUE: u32 = 0xad;
}
