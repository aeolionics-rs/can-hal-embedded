#[doc = "Register `INTSIGEN` reader"]
pub type R = crate::R<IntsigenSpec>;
#[doc = "Register `INTSIGEN` writer"]
pub type W = crate::W<IntsigenSpec>;
#[doc = "Field `TXEMPTYISE` reader - TX FIFO Empty Interrupt Signal Enable."]
pub type TxemptyiseR = crate::BitReader;
#[doc = "Field `TXEMPTYISE` writer - TX FIFO Empty Interrupt Signal Enable."]
pub type TxemptyiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFULLISE` reader - TX FIFO Full Interrupt Signal Enable."]
pub type TxfulliseR = crate::BitReader;
#[doc = "Field `TXFULLISE` writer - TX FIFO Full Interrupt Signal Enable."]
pub type TxfulliseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTHRISE` reader - Signal Enable Interrupt to be generated when the TX FIFO has space equal to or more than TX_INT_THRES words."]
pub type TxthriseR = crate::BitReader;
#[doc = "Field `TXTHRISE` writer - Signal Enable Interrupt to be generated when the TX FIFO has space equal to or more than TX_INT_THRES words."]
pub type TxthriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEMPTYISE` reader - RX FIFO Empty Interrupt Signal Enable."]
pub type RxemptyiseR = crate::BitReader;
#[doc = "Field `RXEMPTYISE` writer - RX FIFO Empty Interrupt Signal Enable."]
pub type RxemptyiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULLISE` reader - RX FIFO Full Interrupt Signal Enable."]
pub type RxfulliseR = crate::BitReader;
#[doc = "Field `RXFULLISE` writer - RX FIFO Full Interrupt Signal Enable."]
pub type RxfulliseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHRISE` reader - Signal Enable Interrupt to be generated when the RX FIFO has data more than or equal to RX_INT_THRES words. In case of Boot/XIP mode, the power on reset value of RX buffer threshold is zero. So this bit will be set to 1, immediately after power on reset, till a read request on the AHB Bus."]
pub type RxthriseR = crate::BitReader;
#[doc = "Field `RXTHRISE` writer - Signal Enable Interrupt to be generated when the RX FIFO has data more than or equal to RX_INT_THRES words. In case of Boot/XIP mode, the power on reset value of RX buffer threshold is zero. So this bit will be set to 1, immediately after power on reset, till a read request on the AHB Bus."]
pub type RxthriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONFULLISE` reader - Control Buffer Full Interrupt Signal Enable"]
pub type ConfulliseR = crate::BitReader;
#[doc = "Field `CONFULLISE` writer - Control Buffer Full Interrupt Signal Enable"]
pub type ConfulliseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONEMPTYISE` reader - Control Buffer Empty Interrupt Signal Enable"]
pub type ConemptyiseR = crate::BitReader;
#[doc = "Field `CONEMPTYISE` writer - Control Buffer Empty Interrupt Signal Enable"]
pub type ConemptyiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTHRISE` reader - Control Buffer Threshold Interrupt Signal Enable"]
pub type ConthriseR = crate::BitReader;
#[doc = "Field `CONTHRISE` writer - Control Buffer Threshold Interrupt Signal Enable"]
pub type ConthriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDDONEISE` reader - Current Buffer Descriptor Interrupt Signal Enable"]
pub type BddoneiseR = crate::BitReader;
#[doc = "Field `BDDONEISE` writer - Current Buffer Descriptor Interrupt Signal Enable"]
pub type BddoneiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTCOMPISE` reader - Packet completion Interrupt Signal Enable"]
pub type PktcompiseR = crate::BitReader;
#[doc = "Field `PKTCOMPISE` writer - Packet completion Interrupt Signal Enable"]
pub type PktcompiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEISE` reader - Master Error Interrupt Signal Enable"]
pub type DmaeiseR = crate::BitReader;
#[doc = "Field `DMAEISE` writer - Master Error Interrupt Signal Enable"]
pub type DmaeiseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX FIFO Empty Interrupt Signal Enable."]
    #[inline(always)]
    pub fn txemptyise(&self) -> TxemptyiseR {
        TxemptyiseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Full Interrupt Signal Enable."]
    #[inline(always)]
    pub fn txfullise(&self) -> TxfulliseR {
        TxfulliseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Signal Enable Interrupt to be generated when the TX FIFO has space equal to or more than TX_INT_THRES words."]
    #[inline(always)]
    pub fn txthrise(&self) -> TxthriseR {
        TxthriseR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Empty Interrupt Signal Enable."]
    #[inline(always)]
    pub fn rxemptyise(&self) -> RxemptyiseR {
        RxemptyiseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Full Interrupt Signal Enable."]
    #[inline(always)]
    pub fn rxfullise(&self) -> RxfulliseR {
        RxfulliseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Signal Enable Interrupt to be generated when the RX FIFO has data more than or equal to RX_INT_THRES words. In case of Boot/XIP mode, the power on reset value of RX buffer threshold is zero. So this bit will be set to 1, immediately after power on reset, till a read request on the AHB Bus."]
    #[inline(always)]
    pub fn rxthrise(&self) -> RxthriseR {
        RxthriseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Control Buffer Full Interrupt Signal Enable"]
    #[inline(always)]
    pub fn confullise(&self) -> ConfulliseR {
        ConfulliseR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Control Buffer Empty Interrupt Signal Enable"]
    #[inline(always)]
    pub fn conemptyise(&self) -> ConemptyiseR {
        ConemptyiseR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Control Buffer Threshold Interrupt Signal Enable"]
    #[inline(always)]
    pub fn conthrise(&self) -> ConthriseR {
        ConthriseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Current Buffer Descriptor Interrupt Signal Enable"]
    #[inline(always)]
    pub fn bddoneise(&self) -> BddoneiseR {
        BddoneiseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Packet completion Interrupt Signal Enable"]
    #[inline(always)]
    pub fn pktcompise(&self) -> PktcompiseR {
        PktcompiseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Error Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmaeise(&self) -> DmaeiseR {
        DmaeiseR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Empty Interrupt Signal Enable."]
    #[inline(always)]
    pub fn txemptyise(&mut self) -> TxemptyiseW<'_, IntsigenSpec> {
        TxemptyiseW::new(self, 0)
    }
    #[doc = "Bit 1 - TX FIFO Full Interrupt Signal Enable."]
    #[inline(always)]
    pub fn txfullise(&mut self) -> TxfulliseW<'_, IntsigenSpec> {
        TxfulliseW::new(self, 1)
    }
    #[doc = "Bit 2 - Signal Enable Interrupt to be generated when the TX FIFO has space equal to or more than TX_INT_THRES words."]
    #[inline(always)]
    pub fn txthrise(&mut self) -> TxthriseW<'_, IntsigenSpec> {
        TxthriseW::new(self, 2)
    }
    #[doc = "Bit 3 - RX FIFO Empty Interrupt Signal Enable."]
    #[inline(always)]
    pub fn rxemptyise(&mut self) -> RxemptyiseW<'_, IntsigenSpec> {
        RxemptyiseW::new(self, 3)
    }
    #[doc = "Bit 4 - RX FIFO Full Interrupt Signal Enable."]
    #[inline(always)]
    pub fn rxfullise(&mut self) -> RxfulliseW<'_, IntsigenSpec> {
        RxfulliseW::new(self, 4)
    }
    #[doc = "Bit 5 - Signal Enable Interrupt to be generated when the RX FIFO has data more than or equal to RX_INT_THRES words. In case of Boot/XIP mode, the power on reset value of RX buffer threshold is zero. So this bit will be set to 1, immediately after power on reset, till a read request on the AHB Bus."]
    #[inline(always)]
    pub fn rxthrise(&mut self) -> RxthriseW<'_, IntsigenSpec> {
        RxthriseW::new(self, 5)
    }
    #[doc = "Bit 6 - Control Buffer Full Interrupt Signal Enable"]
    #[inline(always)]
    pub fn confullise(&mut self) -> ConfulliseW<'_, IntsigenSpec> {
        ConfulliseW::new(self, 6)
    }
    #[doc = "Bit 7 - Control Buffer Empty Interrupt Signal Enable"]
    #[inline(always)]
    pub fn conemptyise(&mut self) -> ConemptyiseW<'_, IntsigenSpec> {
        ConemptyiseW::new(self, 7)
    }
    #[doc = "Bit 8 - Control Buffer Threshold Interrupt Signal Enable"]
    #[inline(always)]
    pub fn conthrise(&mut self) -> ConthriseW<'_, IntsigenSpec> {
        ConthriseW::new(self, 8)
    }
    #[doc = "Bit 9 - Current Buffer Descriptor Interrupt Signal Enable"]
    #[inline(always)]
    pub fn bddoneise(&mut self) -> BddoneiseW<'_, IntsigenSpec> {
        BddoneiseW::new(self, 9)
    }
    #[doc = "Bit 10 - Packet completion Interrupt Signal Enable"]
    #[inline(always)]
    pub fn pktcompise(&mut self) -> PktcompiseW<'_, IntsigenSpec> {
        PktcompiseW::new(self, 10)
    }
    #[doc = "Bit 11 - Master Error Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmaeise(&mut self) -> DmaeiseW<'_, IntsigenSpec> {
        DmaeiseW::new(self, 11)
    }
}
#[doc = "SPI Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intsigen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsigen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntsigenSpec;
impl crate::RegisterSpec for IntsigenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsigen::R`](R) reader structure"]
impl crate::Readable for IntsigenSpec {}
#[doc = "`write(|w| ..)` method takes [`intsigen::W`](W) writer structure"]
impl crate::Writable for IntsigenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSIGEN to value 0x08ad"]
impl crate::Resettable for IntsigenSpec {
    const RESET_VALUE: u32 = 0x08ad;
}
