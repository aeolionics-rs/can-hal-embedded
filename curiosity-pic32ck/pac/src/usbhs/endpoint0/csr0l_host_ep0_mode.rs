#[doc = "Register `CSR0L_HOST_EP0_MODE` reader"]
pub type R = crate::R<Csr0lHostEp0ModeSpec>;
#[doc = "Register `CSR0L_HOST_EP0_MODE` writer"]
pub type W = crate::W<Csr0lHostEp0ModeSpec>;
#[doc = "Field `RXPKTRDY` reader - This Bit Is Set When A Data Packet Has Been Received"]
pub type RxpktrdyR = crate::BitReader;
#[doc = "Field `RXPKTRDY` writer - This Bit Is Set When A Data Packet Has Been Received"]
pub type RxpktrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPKTRDY` reader - The CPU Sets This Bit After Loading A Data Packet Into The FIFO."]
pub type TxpktrdyR = crate::BitReader;
#[doc = "Field `TXPKTRDY` writer - The CPU Sets This Bit After Loading A Data Packet Into The FIFO."]
pub type TxpktrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALL` reader - This Bit Is Set When A Stall Handshake Is Received."]
pub type RxstallR = crate::BitReader;
#[doc = "Field `RXSTALL` writer - This Bit Is Set When A Stall Handshake Is Received."]
pub type RxstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUPPKT` reader - The CPU Sets This Bit, At The Same Time As The TxPktRdy Bit Is Set, To Send A SETUP Token Instead Of An OUT Token For The Transaction"]
pub type SetuppktR = crate::BitReader;
#[doc = "Field `SETUPPKT` writer - The CPU Sets This Bit, At The Same Time As The TxPktRdy Bit Is Set, To Send A SETUP Token Instead Of An OUT Token For The Transaction"]
pub type SetuppktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - This Bit Will Be Set When Three Attempts Have Been Made To Perform A Transaction With No Response From The Peripheral"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - This Bit Will Be Set When Three Attempts Have Been Made To Perform A Transaction With No Response From The Peripheral"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQPKT` reader - The CPU Sets This Bit To Request An IN Transaction"]
pub type ReqpktR = crate::BitReader;
#[doc = "Field `REQPKT` writer - The CPU Sets This Bit To Request An IN Transaction"]
pub type ReqpktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUSPKT` reader - The CPU Sets This Bit At The Same Time As The TxPktRdy Or ReqPkt Bit Is Set, To Perform A Status Stage Transaction"]
pub type StatuspktR = crate::BitReader;
#[doc = "Field `STATUSPKT` writer - The CPU Sets This Bit At The Same Time As The TxPktRdy Or ReqPkt Bit Is Set, To Perform A Status Stage Transaction"]
pub type StatuspktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKTIMEOUT` reader - This Bit Will Be Set When Endpoint 0 Is Halted Following The Receipt Of NAK Responses For Longer Than The Time Set By The NAKLimit0 register."]
pub type NaktimeoutR = crate::BitReader;
#[doc = "Field `NAKTIMEOUT` writer - This Bit Will Be Set When Endpoint 0 Is Halted Following The Receipt Of NAK Responses For Longer Than The Time Set By The NAKLimit0 register."]
pub type NaktimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This Bit Is Set When A Data Packet Has Been Received"]
    #[inline(always)]
    pub fn rxpktrdy(&self) -> RxpktrdyR {
        RxpktrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The CPU Sets This Bit After Loading A Data Packet Into The FIFO."]
    #[inline(always)]
    pub fn txpktrdy(&self) -> TxpktrdyR {
        TxpktrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This Bit Is Set When A Stall Handshake Is Received."]
    #[inline(always)]
    pub fn rxstall(&self) -> RxstallR {
        RxstallR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The CPU Sets This Bit, At The Same Time As The TxPktRdy Bit Is Set, To Send A SETUP Token Instead Of An OUT Token For The Transaction"]
    #[inline(always)]
    pub fn setuppkt(&self) -> SetuppktR {
        SetuppktR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This Bit Will Be Set When Three Attempts Have Been Made To Perform A Transaction With No Response From The Peripheral"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The CPU Sets This Bit To Request An IN Transaction"]
    #[inline(always)]
    pub fn reqpkt(&self) -> ReqpktR {
        ReqpktR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The CPU Sets This Bit At The Same Time As The TxPktRdy Or ReqPkt Bit Is Set, To Perform A Status Stage Transaction"]
    #[inline(always)]
    pub fn statuspkt(&self) -> StatuspktR {
        StatuspktR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This Bit Will Be Set When Endpoint 0 Is Halted Following The Receipt Of NAK Responses For Longer Than The Time Set By The NAKLimit0 register."]
    #[inline(always)]
    pub fn naktimeout(&self) -> NaktimeoutR {
        NaktimeoutR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This Bit Is Set When A Data Packet Has Been Received"]
    #[inline(always)]
    pub fn rxpktrdy(&mut self) -> RxpktrdyW<'_, Csr0lHostEp0ModeSpec> {
        RxpktrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - The CPU Sets This Bit After Loading A Data Packet Into The FIFO."]
    #[inline(always)]
    pub fn txpktrdy(&mut self) -> TxpktrdyW<'_, Csr0lHostEp0ModeSpec> {
        TxpktrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - This Bit Is Set When A Stall Handshake Is Received."]
    #[inline(always)]
    pub fn rxstall(&mut self) -> RxstallW<'_, Csr0lHostEp0ModeSpec> {
        RxstallW::new(self, 2)
    }
    #[doc = "Bit 3 - The CPU Sets This Bit, At The Same Time As The TxPktRdy Bit Is Set, To Send A SETUP Token Instead Of An OUT Token For The Transaction"]
    #[inline(always)]
    pub fn setuppkt(&mut self) -> SetuppktW<'_, Csr0lHostEp0ModeSpec> {
        SetuppktW::new(self, 3)
    }
    #[doc = "Bit 4 - This Bit Will Be Set When Three Attempts Have Been Made To Perform A Transaction With No Response From The Peripheral"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, Csr0lHostEp0ModeSpec> {
        ErrorW::new(self, 4)
    }
    #[doc = "Bit 5 - The CPU Sets This Bit To Request An IN Transaction"]
    #[inline(always)]
    pub fn reqpkt(&mut self) -> ReqpktW<'_, Csr0lHostEp0ModeSpec> {
        ReqpktW::new(self, 5)
    }
    #[doc = "Bit 6 - The CPU Sets This Bit At The Same Time As The TxPktRdy Or ReqPkt Bit Is Set, To Perform A Status Stage Transaction"]
    #[inline(always)]
    pub fn statuspkt(&mut self) -> StatuspktW<'_, Csr0lHostEp0ModeSpec> {
        StatuspktW::new(self, 6)
    }
    #[doc = "Bit 7 - This Bit Will Be Set When Endpoint 0 Is Halted Following The Receipt Of NAK Responses For Longer Than The Time Set By The NAKLimit0 register."]
    #[inline(always)]
    pub fn naktimeout(&mut self) -> NaktimeoutW<'_, Csr0lHostEp0ModeSpec> {
        NaktimeoutW::new(self, 7)
    }
}
#[doc = "ENDPOINT0 Control And Status Bits for TX Endpoint 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csr0l_host_ep0_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0l_host_ep0_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr0lHostEp0ModeSpec;
impl crate::RegisterSpec for Csr0lHostEp0ModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csr0l_host_ep0_mode::R`](R) reader structure"]
impl crate::Readable for Csr0lHostEp0ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`csr0l_host_ep0_mode::W`](W) writer structure"]
impl crate::Writable for Csr0lHostEp0ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR0L_HOST_EP0_MODE to value 0"]
impl crate::Resettable for Csr0lHostEp0ModeSpec {}
