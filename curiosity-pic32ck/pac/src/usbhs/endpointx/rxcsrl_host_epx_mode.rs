#[doc = "Register `RXCSRL_HOST_EPX_MODE` reader"]
pub type R = crate::R<RxcsrlHostEpxModeSpec>;
#[doc = "Register `RXCSRL_HOST_EPX_MODE` writer"]
pub type W = crate::W<RxcsrlHostEpxModeSpec>;
#[doc = "Field `RXPKTRDY` reader - This Bit Is Set When A Data Packet Has Been Received."]
pub type RxpktrdyR = crate::BitReader;
#[doc = "Field `RXPKTRDY` writer - This Bit Is Set When A Data Packet Has Been Received."]
pub type RxpktrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOFULL` reader - This Bit Is Set When No More Packets Can Be Loaded"]
pub type FifofullR = crate::BitReader;
#[doc = "Field `FIFOFULL` writer - This Bit Is Set When No More Packets Can Be Loaded"]
pub type FifofullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - The USB Sets This Bit When 3 Attempts Have Been Made To Receive A Packet And No Data Packet Has Been Received."]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - The USB Sets This Bit When 3 Attempts Have Been Made To Receive A Packet And No Data Packet Has Been Received."]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKTIMEOUT` reader - This Bit Is Set When RxPktRdy Is Set If The Data Packet Has A CRC Or Bit-stuff Error."]
pub type NaktimeoutR = crate::BitReader;
#[doc = "Field `NAKTIMEOUT` writer - This Bit Is Set When RxPktRdy Is Set If The Data Packet Has A CRC Or Bit-stuff Error."]
pub type NaktimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSHFIFO` reader - The CPU Writes A 1 To This Bit To Flush The Next Packet To Be Read From The Endpoint Rx FIFO"]
pub type FlushfifoR = crate::BitReader;
#[doc = "Field `FLUSHFIFO` writer - The CPU Writes A 1 To This Bit To Flush The Next Packet To Be Read From The Endpoint Rx FIFO"]
pub type FlushfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQPKT` reader - The CPU Writes A 1 To This Bit To Request An IN Transaction."]
pub type ReqpktR = crate::BitReader;
#[doc = "Field `REQPKT` writer - The CPU Writes A 1 To This Bit To Request An IN Transaction."]
pub type ReqpktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALL` reader - When A STALL Handshake Is Received, This Bit Is Set And An Interrupt Is Generated."]
pub type RxstallR = crate::BitReader;
#[doc = "Field `RXSTALL` writer - When A STALL Handshake Is Received, This Bit Is Set And An Interrupt Is Generated."]
pub type RxstallW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 1 - This Bit Is Set When No More Packets Can Be Loaded"]
    #[inline(always)]
    pub fn fifofull(&self) -> FifofullR {
        FifofullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The USB Sets This Bit When 3 Attempts Have Been Made To Receive A Packet And No Data Packet Has Been Received."]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This Bit Is Set When RxPktRdy Is Set If The Data Packet Has A CRC Or Bit-stuff Error."]
    #[inline(always)]
    pub fn naktimeout(&self) -> NaktimeoutR {
        NaktimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The CPU Writes A 1 To This Bit To Flush The Next Packet To Be Read From The Endpoint Rx FIFO"]
    #[inline(always)]
    pub fn flushfifo(&self) -> FlushfifoR {
        FlushfifoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The CPU Writes A 1 To This Bit To Request An IN Transaction."]
    #[inline(always)]
    pub fn reqpkt(&self) -> ReqpktR {
        ReqpktR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When A STALL Handshake Is Received, This Bit Is Set And An Interrupt Is Generated."]
    #[inline(always)]
    pub fn rxstall(&self) -> RxstallR {
        RxstallR::new(((self.bits >> 6) & 1) != 0)
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
    pub fn rxpktrdy(&mut self) -> RxpktrdyW<'_, RxcsrlHostEpxModeSpec> {
        RxpktrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - This Bit Is Set When No More Packets Can Be Loaded"]
    #[inline(always)]
    pub fn fifofull(&mut self) -> FifofullW<'_, RxcsrlHostEpxModeSpec> {
        FifofullW::new(self, 1)
    }
    #[doc = "Bit 2 - The USB Sets This Bit When 3 Attempts Have Been Made To Receive A Packet And No Data Packet Has Been Received."]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, RxcsrlHostEpxModeSpec> {
        ErrorW::new(self, 2)
    }
    #[doc = "Bit 3 - This Bit Is Set When RxPktRdy Is Set If The Data Packet Has A CRC Or Bit-stuff Error."]
    #[inline(always)]
    pub fn naktimeout(&mut self) -> NaktimeoutW<'_, RxcsrlHostEpxModeSpec> {
        NaktimeoutW::new(self, 3)
    }
    #[doc = "Bit 4 - The CPU Writes A 1 To This Bit To Flush The Next Packet To Be Read From The Endpoint Rx FIFO"]
    #[inline(always)]
    pub fn flushfifo(&mut self) -> FlushfifoW<'_, RxcsrlHostEpxModeSpec> {
        FlushfifoW::new(self, 4)
    }
    #[doc = "Bit 5 - The CPU Writes A 1 To This Bit To Request An IN Transaction."]
    #[inline(always)]
    pub fn reqpkt(&mut self) -> ReqpktW<'_, RxcsrlHostEpxModeSpec> {
        ReqpktW::new(self, 5)
    }
    #[doc = "Bit 6 - When A STALL Handshake Is Received, This Bit Is Set And An Interrupt Is Generated."]
    #[inline(always)]
    pub fn rxstall(&mut self) -> RxstallW<'_, RxcsrlHostEpxModeSpec> {
        RxstallW::new(self, 6)
    }
    #[doc = "Bit 7 - The CPU Writes A 1 To This Bit To Reset The Endpoint Data Toggle To 0."]
    #[inline(always)]
    pub fn clrdatatog(&mut self) -> ClrdatatogW<'_, RxcsrlHostEpxModeSpec> {
        ClrdatatogW::new(self, 7)
    }
}
#[doc = "ENDPOINTX Control And Status Bits for RX Endpoint 0 Or Endpoint 1-15\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcsrl_host_epx_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxcsrl_host_epx_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxcsrlHostEpxModeSpec;
impl crate::RegisterSpec for RxcsrlHostEpxModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxcsrl_host_epx_mode::R`](R) reader structure"]
impl crate::Readable for RxcsrlHostEpxModeSpec {}
#[doc = "`write(|w| ..)` method takes [`rxcsrl_host_epx_mode::W`](W) writer structure"]
impl crate::Writable for RxcsrlHostEpxModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXCSRL_HOST_EPX_MODE to value 0"]
impl crate::Resettable for RxcsrlHostEpxModeSpec {}
