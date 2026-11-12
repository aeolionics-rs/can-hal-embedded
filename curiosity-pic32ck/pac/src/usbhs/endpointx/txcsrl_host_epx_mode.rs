#[doc = "Register `TXCSRL_HOST_EPX_MODE` reader"]
pub type R = crate::R<TxcsrlHostEpxModeSpec>;
#[doc = "Register `TXCSRL_HOST_EPX_MODE` writer"]
pub type W = crate::W<TxcsrlHostEpxModeSpec>;
#[doc = "Field `TXPKTRDY` reader - The CPU Sets This Bit After Loading A Data Packet Into The FIFO."]
pub type TxpktrdyR = crate::BitReader;
#[doc = "Field `TXPKTRDY` writer - The CPU Sets This Bit After Loading A Data Packet Into The FIFO."]
pub type TxpktrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFONOTEMPTY` reader - The USB Sets This Bit When There Is At Least 1 Packet In The TX FIFO."]
pub type FifonotemptyR = crate::BitReader;
#[doc = "Field `FIFONOTEMPTY` writer - The USB Sets This Bit When There Is At Least 1 Packet In The TX FIFO."]
pub type FifonotemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - This Bit Will Be Set When Three Attempts Have Been Made To Perform A Transaction With No Response From The Peripheral"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - This Bit Will Be Set When Three Attempts Have Been Made To Perform A Transaction With No Response From The Peripheral"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSHFIFO` reader - The CPU Writes A 1 To This Bit To Flush The Latest Packet From The Endpoint TX FIFO"]
pub type FlushfifoR = crate::BitReader;
#[doc = "Field `FLUSHFIFO` writer - The CPU Writes A 1 To This Bit To Flush The Latest Packet From The Endpoint TX FIFO"]
pub type FlushfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUPPKT` reader - The CPU Sets This Bit, At The Same Time As The TxPktRdy Bit Is Set, To Send A SETUP Token Instead Of An OUT Token For The Transaction"]
pub type SetuppktR = crate::BitReader;
#[doc = "Field `SETUPPKT` writer - The CPU Sets This Bit, At The Same Time As The TxPktRdy Bit Is Set, To Send A SETUP Token Instead Of An OUT Token For The Transaction"]
pub type SetuppktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALL` reader - This Bit Is Set When A Stall Handshake Is Received."]
pub type RxstallR = crate::BitReader;
#[doc = "Field `RXSTALL` writer - This Bit Is Set When A Stall Handshake Is Received."]
pub type RxstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRDATATOG` reader - The CPU Writes A 1 To This Bit To Reset The Endpoint Data Toggle To 0."]
pub type ClrdatatogR = crate::BitReader;
#[doc = "Field `CLRDATATOG` writer - The CPU Writes A 1 To This Bit To Reset The Endpoint Data Toggle To 0."]
pub type ClrdatatogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKTIMEOUT` reader - This Bit Will Be Set When Endpoint 0 Is Halted Following The Receipt Of NAK Responses For Longer Than The Time Set By The NAKLimit0 register."]
pub type NaktimeoutR = crate::BitReader;
#[doc = "Field `NAKTIMEOUT` writer - This Bit Will Be Set When Endpoint 0 Is Halted Following The Receipt Of NAK Responses For Longer Than The Time Set By The NAKLimit0 register."]
pub type NaktimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The CPU Sets This Bit After Loading A Data Packet Into The FIFO."]
    #[inline(always)]
    pub fn txpktrdy(&self) -> TxpktrdyR {
        TxpktrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The USB Sets This Bit When There Is At Least 1 Packet In The TX FIFO."]
    #[inline(always)]
    pub fn fifonotempty(&self) -> FifonotemptyR {
        FifonotemptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This Bit Will Be Set When Three Attempts Have Been Made To Perform A Transaction With No Response From The Peripheral"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The CPU Writes A 1 To This Bit To Flush The Latest Packet From The Endpoint TX FIFO"]
    #[inline(always)]
    pub fn flushfifo(&self) -> FlushfifoR {
        FlushfifoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The CPU Sets This Bit, At The Same Time As The TxPktRdy Bit Is Set, To Send A SETUP Token Instead Of An OUT Token For The Transaction"]
    #[inline(always)]
    pub fn setuppkt(&self) -> SetuppktR {
        SetuppktR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This Bit Is Set When A Stall Handshake Is Received."]
    #[inline(always)]
    pub fn rxstall(&self) -> RxstallR {
        RxstallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The CPU Writes A 1 To This Bit To Reset The Endpoint Data Toggle To 0."]
    #[inline(always)]
    pub fn clrdatatog(&self) -> ClrdatatogR {
        ClrdatatogR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This Bit Will Be Set When Endpoint 0 Is Halted Following The Receipt Of NAK Responses For Longer Than The Time Set By The NAKLimit0 register."]
    #[inline(always)]
    pub fn naktimeout(&self) -> NaktimeoutR {
        NaktimeoutR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The CPU Sets This Bit After Loading A Data Packet Into The FIFO."]
    #[inline(always)]
    pub fn txpktrdy(&mut self) -> TxpktrdyW<'_, TxcsrlHostEpxModeSpec> {
        TxpktrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - The USB Sets This Bit When There Is At Least 1 Packet In The TX FIFO."]
    #[inline(always)]
    pub fn fifonotempty(&mut self) -> FifonotemptyW<'_, TxcsrlHostEpxModeSpec> {
        FifonotemptyW::new(self, 1)
    }
    #[doc = "Bit 2 - This Bit Will Be Set When Three Attempts Have Been Made To Perform A Transaction With No Response From The Peripheral"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, TxcsrlHostEpxModeSpec> {
        ErrorW::new(self, 2)
    }
    #[doc = "Bit 3 - The CPU Writes A 1 To This Bit To Flush The Latest Packet From The Endpoint TX FIFO"]
    #[inline(always)]
    pub fn flushfifo(&mut self) -> FlushfifoW<'_, TxcsrlHostEpxModeSpec> {
        FlushfifoW::new(self, 3)
    }
    #[doc = "Bit 4 - The CPU Sets This Bit, At The Same Time As The TxPktRdy Bit Is Set, To Send A SETUP Token Instead Of An OUT Token For The Transaction"]
    #[inline(always)]
    pub fn setuppkt(&mut self) -> SetuppktW<'_, TxcsrlHostEpxModeSpec> {
        SetuppktW::new(self, 4)
    }
    #[doc = "Bit 5 - This Bit Is Set When A Stall Handshake Is Received."]
    #[inline(always)]
    pub fn rxstall(&mut self) -> RxstallW<'_, TxcsrlHostEpxModeSpec> {
        RxstallW::new(self, 5)
    }
    #[doc = "Bit 6 - The CPU Writes A 1 To This Bit To Reset The Endpoint Data Toggle To 0."]
    #[inline(always)]
    pub fn clrdatatog(&mut self) -> ClrdatatogW<'_, TxcsrlHostEpxModeSpec> {
        ClrdatatogW::new(self, 6)
    }
    #[doc = "Bit 7 - This Bit Will Be Set When Endpoint 0 Is Halted Following The Receipt Of NAK Responses For Longer Than The Time Set By The NAKLimit0 register."]
    #[inline(always)]
    pub fn naktimeout(&mut self) -> NaktimeoutW<'_, TxcsrlHostEpxModeSpec> {
        NaktimeoutW::new(self, 7)
    }
}
#[doc = "ENDPOINTX Control And Status Bits for TX Endpoints 1-15\n\nYou can [`read`](crate::Reg::read) this register and get [`txcsrl_host_epx_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txcsrl_host_epx_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxcsrlHostEpxModeSpec;
impl crate::RegisterSpec for TxcsrlHostEpxModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txcsrl_host_epx_mode::R`](R) reader structure"]
impl crate::Readable for TxcsrlHostEpxModeSpec {}
#[doc = "`write(|w| ..)` method takes [`txcsrl_host_epx_mode::W`](W) writer structure"]
impl crate::Writable for TxcsrlHostEpxModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXCSRL_HOST_EPX_MODE to value 0"]
impl crate::Resettable for TxcsrlHostEpxModeSpec {}
