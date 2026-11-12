#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `TXEMPTYIE` reader - TX FIFO Empty Interrupt Enable."]
pub type TxemptyieR = crate::BitReader;
#[doc = "Field `TXEMPTYIE` writer - TX FIFO Empty Interrupt Enable."]
pub type TxemptyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFULLIE` reader - TX FIFO Full Interrupt Enable."]
pub type TxfullieR = crate::BitReader;
#[doc = "Field `TXFULLIE` writer - TX FIFO Full Interrupt Enable."]
pub type TxfullieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTHRIE` reader - Enable Interrupt to be generated when the TX FIFO has space equal to or more than TX_INT_THRES words."]
pub type TxthrieR = crate::BitReader;
#[doc = "Field `TXTHRIE` writer - Enable Interrupt to be generated when the TX FIFO has space equal to or more than TX_INT_THRES words."]
pub type TxthrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEMPTYIE` reader - RX FIFO Empty Interrupt Enable."]
pub type RxemptyieR = crate::BitReader;
#[doc = "Field `RXEMPTYIE` writer - RX FIFO Empty Interrupt Enable."]
pub type RxemptyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULLIE` reader - RX FIFO Full Interrupt Enable."]
pub type RxfullieR = crate::BitReader;
#[doc = "Field `RXFULLIE` writer - RX FIFO Full Interrupt Enable."]
pub type RxfullieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHRIE` reader - Enable Interrupt to be generated when the RX FIFO has data more than or equal to RX_INT_THRES words. In case of Boot/XIP mode, the power on reset value of RX buffer threshold is zero. So this bit will be set to 1, immediately after power on reset, till a read request on the AHB Bus."]
pub type RxthrieR = crate::BitReader;
#[doc = "Field `RXTHRIE` writer - Enable Interrupt to be generated when the RX FIFO has data more than or equal to RX_INT_THRES words. In case of Boot/XIP mode, the power on reset value of RX buffer threshold is zero. So this bit will be set to 1, immediately after power on reset, till a read request on the AHB Bus."]
pub type RxthrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONFULLIE` reader - Control Buffer Full Interrupt Enable"]
pub type ConfullieR = crate::BitReader;
#[doc = "Field `CONFULLIE` writer - Control Buffer Full Interrupt Enable"]
pub type ConfullieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONEMPTYIE` reader - Control Buffer Empty Interrupt Enable"]
pub type ConemptyieR = crate::BitReader;
#[doc = "Field `CONEMPTYIE` writer - Control Buffer Empty Interrupt Enable"]
pub type ConemptyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTHRIE` reader - Control Buffer Threshold Interrupt Enable"]
pub type ConthrieR = crate::BitReader;
#[doc = "Field `CONTHRIE` writer - Control Buffer Threshold Interrupt Enable"]
pub type ConthrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDDONEIE` reader - Current Buffer Descriptor Interrupt Enable"]
pub type BddoneieR = crate::BitReader;
#[doc = "Field `BDDONEIE` writer - Current Buffer Descriptor Interrupt Enable"]
pub type BddoneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTCOMPIE` reader - Packet completion Interrupt"]
pub type PktcompieR = crate::BitReader;
#[doc = "Field `PKTCOMPIE` writer - Packet completion Interrupt"]
pub type PktcompieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEIE` reader - Master Error Interrupt Enable"]
pub type DmaeieR = crate::BitReader;
#[doc = "Field `DMAEIE` writer - Master Error Interrupt Enable"]
pub type DmaeieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX FIFO Empty Interrupt Enable."]
    #[inline(always)]
    pub fn txemptyie(&self) -> TxemptyieR {
        TxemptyieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Full Interrupt Enable."]
    #[inline(always)]
    pub fn txfullie(&self) -> TxfullieR {
        TxfullieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Interrupt to be generated when the TX FIFO has space equal to or more than TX_INT_THRES words."]
    #[inline(always)]
    pub fn txthrie(&self) -> TxthrieR {
        TxthrieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Empty Interrupt Enable."]
    #[inline(always)]
    pub fn rxemptyie(&self) -> RxemptyieR {
        RxemptyieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Full Interrupt Enable."]
    #[inline(always)]
    pub fn rxfullie(&self) -> RxfullieR {
        RxfullieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Interrupt to be generated when the RX FIFO has data more than or equal to RX_INT_THRES words. In case of Boot/XIP mode, the power on reset value of RX buffer threshold is zero. So this bit will be set to 1, immediately after power on reset, till a read request on the AHB Bus."]
    #[inline(always)]
    pub fn rxthrie(&self) -> RxthrieR {
        RxthrieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Control Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn confullie(&self) -> ConfullieR {
        ConfullieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Control Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn conemptyie(&self) -> ConemptyieR {
        ConemptyieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Control Buffer Threshold Interrupt Enable"]
    #[inline(always)]
    pub fn conthrie(&self) -> ConthrieR {
        ConthrieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Current Buffer Descriptor Interrupt Enable"]
    #[inline(always)]
    pub fn bddoneie(&self) -> BddoneieR {
        BddoneieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Packet completion Interrupt"]
    #[inline(always)]
    pub fn pktcompie(&self) -> PktcompieR {
        PktcompieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Error Interrupt Enable"]
    #[inline(always)]
    pub fn dmaeie(&self) -> DmaeieR {
        DmaeieR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Empty Interrupt Enable."]
    #[inline(always)]
    pub fn txemptyie(&mut self) -> TxemptyieW<'_, IntenSpec> {
        TxemptyieW::new(self, 0)
    }
    #[doc = "Bit 1 - TX FIFO Full Interrupt Enable."]
    #[inline(always)]
    pub fn txfullie(&mut self) -> TxfullieW<'_, IntenSpec> {
        TxfullieW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Interrupt to be generated when the TX FIFO has space equal to or more than TX_INT_THRES words."]
    #[inline(always)]
    pub fn txthrie(&mut self) -> TxthrieW<'_, IntenSpec> {
        TxthrieW::new(self, 2)
    }
    #[doc = "Bit 3 - RX FIFO Empty Interrupt Enable."]
    #[inline(always)]
    pub fn rxemptyie(&mut self) -> RxemptyieW<'_, IntenSpec> {
        RxemptyieW::new(self, 3)
    }
    #[doc = "Bit 4 - RX FIFO Full Interrupt Enable."]
    #[inline(always)]
    pub fn rxfullie(&mut self) -> RxfullieW<'_, IntenSpec> {
        RxfullieW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Interrupt to be generated when the RX FIFO has data more than or equal to RX_INT_THRES words. In case of Boot/XIP mode, the power on reset value of RX buffer threshold is zero. So this bit will be set to 1, immediately after power on reset, till a read request on the AHB Bus."]
    #[inline(always)]
    pub fn rxthrie(&mut self) -> RxthrieW<'_, IntenSpec> {
        RxthrieW::new(self, 5)
    }
    #[doc = "Bit 6 - Control Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn confullie(&mut self) -> ConfullieW<'_, IntenSpec> {
        ConfullieW::new(self, 6)
    }
    #[doc = "Bit 7 - Control Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn conemptyie(&mut self) -> ConemptyieW<'_, IntenSpec> {
        ConemptyieW::new(self, 7)
    }
    #[doc = "Bit 8 - Control Buffer Threshold Interrupt Enable"]
    #[inline(always)]
    pub fn conthrie(&mut self) -> ConthrieW<'_, IntenSpec> {
        ConthrieW::new(self, 8)
    }
    #[doc = "Bit 9 - Current Buffer Descriptor Interrupt Enable"]
    #[inline(always)]
    pub fn bddoneie(&mut self) -> BddoneieW<'_, IntenSpec> {
        BddoneieW::new(self, 9)
    }
    #[doc = "Bit 10 - Packet completion Interrupt"]
    #[inline(always)]
    pub fn pktcompie(&mut self) -> PktcompieW<'_, IntenSpec> {
        PktcompieW::new(self, 10)
    }
    #[doc = "Bit 11 - Master Error Interrupt Enable"]
    #[inline(always)]
    pub fn dmaeie(&mut self) -> DmaeieW<'_, IntenSpec> {
        DmaeieW::new(self, 11)
    }
}
#[doc = "SPI Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0xad"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0xad;
}
