#[doc = "Register `TXCSRH_PERIPHERAL_EPX_MODE` reader"]
pub type R = crate::R<TxcsrhPeripheralEpxModeSpec>;
#[doc = "Register `TXCSRH_PERIPHERAL_EPX_MODE` writer"]
pub type W = crate::W<TxcsrhPeripheralEpxModeSpec>;
#[doc = "Field `DMAREQMODE` reader - The CPU Sets This Bit To Select DMA Request Mode 1 And Clears It To Select DMA Request Mode 0."]
pub type DmareqmodeR = crate::BitReader;
#[doc = "Field `DMAREQMODE` writer - The CPU Sets This Bit To Select DMA Request Mode 1 And Clears It To Select DMA Request Mode 0."]
pub type DmareqmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRCDATATOG` reader - The CPU Sets This Bit To Force The Endpoint Data Toggle To Switch And The Data Packet To Be Cleared From The FIFO."]
pub type FrcdatatogR = crate::BitReader;
#[doc = "Field `FRCDATATOG` writer - The CPU Sets This Bit To Force The Endpoint Data Toggle To Switch And The Data Packet To Be Cleared From The FIFO."]
pub type FrcdatatogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREQENAB` reader - The CPU Sets This Bit To Enable The DMA Request For The TX Endpoint"]
pub type DmareqenabR = crate::BitReader;
#[doc = "Field `DMAREQENAB` writer - The CPU Sets This Bit To Enable The DMA Request For The TX Endpoint"]
pub type DmareqenabW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - The CPU Sets This Bit To Enable The Endpoint Direction As TX"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - The CPU Sets This Bit To Enable The Endpoint Direction As TX"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISO` reader - The CPU Sets This Bit To Enable The TX Endpoint For Isochronous Transfers"]
pub type IsoR = crate::BitReader;
#[doc = "Field `ISO` writer - The CPU Sets This Bit To Enable The TX Endpoint For Isochronous Transfers"]
pub type IsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOSET` reader - If The CPU Sets This Bit, TxPktRdy Will Be Automatically Set When Data Of The Maximum Packet Size Is Loaded Into The TX FIFO."]
pub type AutosetR = crate::BitReader;
#[doc = "Field `AUTOSET` writer - If The CPU Sets This Bit, TxPktRdy Will Be Automatically Set When Data Of The Maximum Packet Size Is Loaded Into The TX FIFO."]
pub type AutosetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - The CPU Sets This Bit To Select DMA Request Mode 1 And Clears It To Select DMA Request Mode 0."]
    #[inline(always)]
    pub fn dmareqmode(&self) -> DmareqmodeR {
        DmareqmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The CPU Sets This Bit To Force The Endpoint Data Toggle To Switch And The Data Packet To Be Cleared From The FIFO."]
    #[inline(always)]
    pub fn frcdatatog(&self) -> FrcdatatogR {
        FrcdatatogR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The CPU Sets This Bit To Enable The DMA Request For The TX Endpoint"]
    #[inline(always)]
    pub fn dmareqenab(&self) -> DmareqenabR {
        DmareqenabR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The CPU Sets This Bit To Enable The Endpoint Direction As TX"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The CPU Sets This Bit To Enable The TX Endpoint For Isochronous Transfers"]
    #[inline(always)]
    pub fn iso(&self) -> IsoR {
        IsoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If The CPU Sets This Bit, TxPktRdy Will Be Automatically Set When Data Of The Maximum Packet Size Is Loaded Into The TX FIFO."]
    #[inline(always)]
    pub fn autoset(&self) -> AutosetR {
        AutosetR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - The CPU Sets This Bit To Select DMA Request Mode 1 And Clears It To Select DMA Request Mode 0."]
    #[inline(always)]
    pub fn dmareqmode(&mut self) -> DmareqmodeW<'_, TxcsrhPeripheralEpxModeSpec> {
        DmareqmodeW::new(self, 2)
    }
    #[doc = "Bit 3 - The CPU Sets This Bit To Force The Endpoint Data Toggle To Switch And The Data Packet To Be Cleared From The FIFO."]
    #[inline(always)]
    pub fn frcdatatog(&mut self) -> FrcdatatogW<'_, TxcsrhPeripheralEpxModeSpec> {
        FrcdatatogW::new(self, 3)
    }
    #[doc = "Bit 4 - The CPU Sets This Bit To Enable The DMA Request For The TX Endpoint"]
    #[inline(always)]
    pub fn dmareqenab(&mut self) -> DmareqenabW<'_, TxcsrhPeripheralEpxModeSpec> {
        DmareqenabW::new(self, 4)
    }
    #[doc = "Bit 5 - The CPU Sets This Bit To Enable The Endpoint Direction As TX"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, TxcsrhPeripheralEpxModeSpec> {
        ModeW::new(self, 5)
    }
    #[doc = "Bit 6 - The CPU Sets This Bit To Enable The TX Endpoint For Isochronous Transfers"]
    #[inline(always)]
    pub fn iso(&mut self) -> IsoW<'_, TxcsrhPeripheralEpxModeSpec> {
        IsoW::new(self, 6)
    }
    #[doc = "Bit 7 - If The CPU Sets This Bit, TxPktRdy Will Be Automatically Set When Data Of The Maximum Packet Size Is Loaded Into The TX FIFO."]
    #[inline(always)]
    pub fn autoset(&mut self) -> AutosetW<'_, TxcsrhPeripheralEpxModeSpec> {
        AutosetW::new(self, 7)
    }
}
#[doc = "ENDPOINTX Additional Control And Status Bits For TX Endpoints 1-15\n\nYou can [`read`](crate::Reg::read) this register and get [`txcsrh_peripheral_epx_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txcsrh_peripheral_epx_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxcsrhPeripheralEpxModeSpec;
impl crate::RegisterSpec for TxcsrhPeripheralEpxModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txcsrh_peripheral_epx_mode::R`](R) reader structure"]
impl crate::Readable for TxcsrhPeripheralEpxModeSpec {}
#[doc = "`write(|w| ..)` method takes [`txcsrh_peripheral_epx_mode::W`](W) writer structure"]
impl crate::Writable for TxcsrhPeripheralEpxModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXCSRH_PERIPHERAL_EPX_MODE to value 0"]
impl crate::Resettable for TxcsrhPeripheralEpxModeSpec {}
