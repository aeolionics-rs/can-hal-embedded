#[doc = "Register `RXCSRL_PERIPHERAL_EPX_MODE` reader"]
pub type R = crate::R<RxcsrlPeripheralEpxModeSpec>;
#[doc = "Register `RXCSRL_PERIPHERAL_EPX_MODE` writer"]
pub type W = crate::W<RxcsrlPeripheralEpxModeSpec>;
#[doc = "Field `RXPKTRDY` reader - This Bit Is Set When A Data Packet Has Been Received."]
pub type RxpktrdyR = crate::BitReader;
#[doc = "Field `RXPKTRDY` writer - This Bit Is Set When A Data Packet Has Been Received."]
pub type RxpktrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOFULL` reader - This Bit Is Set When No More Packets Can Be Loaded Into The Rx FIFO."]
pub type FifofullR = crate::BitReader;
#[doc = "Field `FIFOFULL` writer - This Bit Is Set When No More Packets Can Be Loaded Into The Rx FIFO."]
pub type FifofullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERRUN` reader - This Bit Is Set If An OUT Packet Cannot Be Loaded Into The Rx FIFO."]
pub type OverrunR = crate::BitReader;
#[doc = "Field `OVERRUN` writer - This Bit Is Set If An OUT Packet Cannot Be Loaded Into The Rx FIFO."]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAERROR` reader - This Bit Is Set When RxPktRdy Is Set If The Data Packet Has A CRC Or Bit-stuff Error."]
pub type DataerrorR = crate::BitReader;
#[doc = "Field `DATAERROR` writer - This Bit Is Set When RxPktRdy Is Set If The Data Packet Has A CRC Or Bit-stuff Error."]
pub type DataerrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSHFIFO` reader - The CPU Writes A 1 To This Bit To Flush The Next Packet To Be Read From The Endpoint Rx FIFO"]
pub type FlushfifoR = crate::BitReader;
#[doc = "Field `FLUSHFIFO` writer - The CPU Writes A 1 To This Bit To Flush The Next Packet To Be Read From The Endpoint Rx FIFO"]
pub type FlushfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENDSTALL` reader - The CPU Writes A 1 To This Bit To Issue A STALL Handshake."]
pub type SendstallR = crate::BitReader;
#[doc = "Field `SENDSTALL` writer - The CPU Writes A 1 To This Bit To Issue A STALL Handshake."]
pub type SendstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENTSTALL` reader - This Bit Is Set When A STALL Handshake Is Transmitted."]
pub type SentstallR = crate::BitReader;
#[doc = "Field `SENTSTALL` writer - This Bit Is Set When A STALL Handshake Is Transmitted."]
pub type SentstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRDATATOG` reader - The CPU Writes A 1 To This Bit To Reset The Endpoint Data Toggle To 0."]
pub type ClrdatatogR = crate::BitReader;
#[doc = "Field `CLRDATATOG` writer - The CPU Writes A 1 To This Bit To Reset The Endpoint Data Toggle To 0."]
pub type ClrdatatogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This Bit Is Set When A Data Packet Has Been Received."]
    #[inline(always)]
    pub fn rxpktrdy(&self) -> RxpktrdyR {
        RxpktrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This Bit Is Set When No More Packets Can Be Loaded Into The Rx FIFO."]
    #[inline(always)]
    pub fn fifofull(&self) -> FifofullR {
        FifofullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This Bit Is Set If An OUT Packet Cannot Be Loaded Into The Rx FIFO."]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This Bit Is Set When RxPktRdy Is Set If The Data Packet Has A CRC Or Bit-stuff Error."]
    #[inline(always)]
    pub fn dataerror(&self) -> DataerrorR {
        DataerrorR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The CPU Writes A 1 To This Bit To Flush The Next Packet To Be Read From The Endpoint Rx FIFO"]
    #[inline(always)]
    pub fn flushfifo(&self) -> FlushfifoR {
        FlushfifoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The CPU Writes A 1 To This Bit To Issue A STALL Handshake."]
    #[inline(always)]
    pub fn sendstall(&self) -> SendstallR {
        SendstallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This Bit Is Set When A STALL Handshake Is Transmitted."]
    #[inline(always)]
    pub fn sentstall(&self) -> SentstallR {
        SentstallR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The CPU Writes A 1 To This Bit To Reset The Endpoint Data Toggle To 0."]
    #[inline(always)]
    pub fn clrdatatog(&self) -> ClrdatatogR {
        ClrdatatogR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This Bit Is Set When A Data Packet Has Been Received."]
    #[inline(always)]
    pub fn rxpktrdy(&mut self) -> RxpktrdyW<'_, RxcsrlPeripheralEpxModeSpec> {
        RxpktrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - This Bit Is Set When No More Packets Can Be Loaded Into The Rx FIFO."]
    #[inline(always)]
    pub fn fifofull(&mut self) -> FifofullW<'_, RxcsrlPeripheralEpxModeSpec> {
        FifofullW::new(self, 1)
    }
    #[doc = "Bit 2 - This Bit Is Set If An OUT Packet Cannot Be Loaded Into The Rx FIFO."]
    #[inline(always)]
    pub fn overrun(&mut self) -> OverrunW<'_, RxcsrlPeripheralEpxModeSpec> {
        OverrunW::new(self, 2)
    }
    #[doc = "Bit 3 - This Bit Is Set When RxPktRdy Is Set If The Data Packet Has A CRC Or Bit-stuff Error."]
    #[inline(always)]
    pub fn dataerror(&mut self) -> DataerrorW<'_, RxcsrlPeripheralEpxModeSpec> {
        DataerrorW::new(self, 3)
    }
    #[doc = "Bit 4 - The CPU Writes A 1 To This Bit To Flush The Next Packet To Be Read From The Endpoint Rx FIFO"]
    #[inline(always)]
    pub fn flushfifo(&mut self) -> FlushfifoW<'_, RxcsrlPeripheralEpxModeSpec> {
        FlushfifoW::new(self, 4)
    }
    #[doc = "Bit 5 - The CPU Writes A 1 To This Bit To Issue A STALL Handshake."]
    #[inline(always)]
    pub fn sendstall(&mut self) -> SendstallW<'_, RxcsrlPeripheralEpxModeSpec> {
        SendstallW::new(self, 5)
    }
    #[doc = "Bit 6 - This Bit Is Set When A STALL Handshake Is Transmitted."]
    #[inline(always)]
    pub fn sentstall(&mut self) -> SentstallW<'_, RxcsrlPeripheralEpxModeSpec> {
        SentstallW::new(self, 6)
    }
    #[doc = "Bit 7 - The CPU Writes A 1 To This Bit To Reset The Endpoint Data Toggle To 0."]
    #[inline(always)]
    pub fn clrdatatog(&mut self) -> ClrdatatogW<'_, RxcsrlPeripheralEpxModeSpec> {
        ClrdatatogW::new(self, 7)
    }
}
#[doc = "ENDPOINTX Control And Status Bits for RX Endpoint 0 Or Endpoint 1-15\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcsrl_peripheral_epx_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxcsrl_peripheral_epx_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxcsrlPeripheralEpxModeSpec;
impl crate::RegisterSpec for RxcsrlPeripheralEpxModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxcsrl_peripheral_epx_mode::R`](R) reader structure"]
impl crate::Readable for RxcsrlPeripheralEpxModeSpec {}
#[doc = "`write(|w| ..)` method takes [`rxcsrl_peripheral_epx_mode::W`](W) writer structure"]
impl crate::Writable for RxcsrlPeripheralEpxModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXCSRL_PERIPHERAL_EPX_MODE to value 0"]
impl crate::Resettable for RxcsrlPeripheralEpxModeSpec {}
