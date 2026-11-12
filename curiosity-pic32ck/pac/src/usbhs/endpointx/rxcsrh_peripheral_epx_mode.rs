#[doc = "Register `RXCSRH_PERIPHERAL_EPX_MODE` reader"]
pub type R = crate::R<RxcsrhPeripheralEpxModeSpec>;
#[doc = "Register `RXCSRH_PERIPHERAL_EPX_MODE` writer"]
pub type W = crate::W<RxcsrhPeripheralEpxModeSpec>;
#[doc = "Field `INCOMPRX` reader - This Bit Is Set In A High-Bandwidth Isochronous/Interrupt Transfer If The Packet In The Rx FIFO Is Incomplete Because Parts Of The Data Were Not Received."]
pub type IncomprxR = crate::BitReader;
#[doc = "Field `INCOMPRX` writer - This Bit Is Set In A High-Bandwidth Isochronous/Interrupt Transfer If The Packet In The Rx FIFO Is Incomplete Because Parts Of The Data Were Not Received."]
pub type IncomprxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREQMODE` reader - The CPU Sets This Bit To Select DMA Request Mode"]
pub type DmareqmodeR = crate::BitReader;
#[doc = "Field `DMAREQMODE` writer - The CPU Sets This Bit To Select DMA Request Mode"]
pub type DmareqmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISNYET` reader - The CPU Sets This Bit To Disable The Sending Of NYET Handshakes"]
pub type DisnyetR = crate::BitReader;
#[doc = "Field `DISNYET` writer - The CPU Sets This Bit To Disable The Sending Of NYET Handshakes"]
pub type DisnyetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREQENAB` reader - The CPU Sets This Bit To Enable The DMA Request For The RX Endpoint"]
pub type DmareqenabR = crate::BitReader;
#[doc = "Field `DMAREQENAB` writer - The CPU Sets This Bit To Enable The DMA Request For The RX Endpoint"]
pub type DmareqenabW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISO` reader - The CPU Sets This Bit To Enable the RX Endpoint For Isochronous Transfers"]
pub type IsoR = crate::BitReader;
#[doc = "Field `ISO` writer - The CPU Sets This Bit To Enable the RX Endpoint For Isochronous Transfers"]
pub type IsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCLEAR` reader - If The CPU Sets This Bit Then The RxPktRdy Bit Will Be Automatically Cleared Whean A Packet Of RxMaxP Bytes Has Benn Unloaded Froim The Rx FIFO"]
pub type AutoclearR = crate::BitReader;
#[doc = "Field `AUTOCLEAR` writer - If The CPU Sets This Bit Then The RxPktRdy Bit Will Be Automatically Cleared Whean A Packet Of RxMaxP Bytes Has Benn Unloaded Froim The Rx FIFO"]
pub type AutoclearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This Bit Is Set In A High-Bandwidth Isochronous/Interrupt Transfer If The Packet In The Rx FIFO Is Incomplete Because Parts Of The Data Were Not Received."]
    #[inline(always)]
    pub fn incomprx(&self) -> IncomprxR {
        IncomprxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - The CPU Sets This Bit To Select DMA Request Mode"]
    #[inline(always)]
    pub fn dmareqmode(&self) -> DmareqmodeR {
        DmareqmodeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The CPU Sets This Bit To Disable The Sending Of NYET Handshakes"]
    #[inline(always)]
    pub fn disnyet(&self) -> DisnyetR {
        DisnyetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The CPU Sets This Bit To Enable The DMA Request For The RX Endpoint"]
    #[inline(always)]
    pub fn dmareqenab(&self) -> DmareqenabR {
        DmareqenabR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The CPU Sets This Bit To Enable the RX Endpoint For Isochronous Transfers"]
    #[inline(always)]
    pub fn iso(&self) -> IsoR {
        IsoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If The CPU Sets This Bit Then The RxPktRdy Bit Will Be Automatically Cleared Whean A Packet Of RxMaxP Bytes Has Benn Unloaded Froim The Rx FIFO"]
    #[inline(always)]
    pub fn autoclear(&self) -> AutoclearR {
        AutoclearR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This Bit Is Set In A High-Bandwidth Isochronous/Interrupt Transfer If The Packet In The Rx FIFO Is Incomplete Because Parts Of The Data Were Not Received."]
    #[inline(always)]
    pub fn incomprx(&mut self) -> IncomprxW<'_, RxcsrhPeripheralEpxModeSpec> {
        IncomprxW::new(self, 0)
    }
    #[doc = "Bit 3 - The CPU Sets This Bit To Select DMA Request Mode"]
    #[inline(always)]
    pub fn dmareqmode(&mut self) -> DmareqmodeW<'_, RxcsrhPeripheralEpxModeSpec> {
        DmareqmodeW::new(self, 3)
    }
    #[doc = "Bit 4 - The CPU Sets This Bit To Disable The Sending Of NYET Handshakes"]
    #[inline(always)]
    pub fn disnyet(&mut self) -> DisnyetW<'_, RxcsrhPeripheralEpxModeSpec> {
        DisnyetW::new(self, 4)
    }
    #[doc = "Bit 5 - The CPU Sets This Bit To Enable The DMA Request For The RX Endpoint"]
    #[inline(always)]
    pub fn dmareqenab(&mut self) -> DmareqenabW<'_, RxcsrhPeripheralEpxModeSpec> {
        DmareqenabW::new(self, 5)
    }
    #[doc = "Bit 6 - The CPU Sets This Bit To Enable the RX Endpoint For Isochronous Transfers"]
    #[inline(always)]
    pub fn iso(&mut self) -> IsoW<'_, RxcsrhPeripheralEpxModeSpec> {
        IsoW::new(self, 6)
    }
    #[doc = "Bit 7 - If The CPU Sets This Bit Then The RxPktRdy Bit Will Be Automatically Cleared Whean A Packet Of RxMaxP Bytes Has Benn Unloaded Froim The Rx FIFO"]
    #[inline(always)]
    pub fn autoclear(&mut self) -> AutoclearW<'_, RxcsrhPeripheralEpxModeSpec> {
        AutoclearW::new(self, 7)
    }
}
#[doc = "ENDPOINTX Additional Control And Status Bits for RX Endpoint 0 Or Endpoint 1-15\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcsrh_peripheral_epx_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxcsrh_peripheral_epx_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxcsrhPeripheralEpxModeSpec;
impl crate::RegisterSpec for RxcsrhPeripheralEpxModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxcsrh_peripheral_epx_mode::R`](R) reader structure"]
impl crate::Readable for RxcsrhPeripheralEpxModeSpec {}
#[doc = "`write(|w| ..)` method takes [`rxcsrh_peripheral_epx_mode::W`](W) writer structure"]
impl crate::Writable for RxcsrhPeripheralEpxModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXCSRH_PERIPHERAL_EPX_MODE to value 0"]
impl crate::Resettable for RxcsrhPeripheralEpxModeSpec {}
