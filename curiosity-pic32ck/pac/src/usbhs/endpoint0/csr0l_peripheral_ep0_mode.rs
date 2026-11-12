#[doc = "Register `CSR0L_PERIPHERAL_EP0_MODE` reader"]
pub type R = crate::R<Csr0lPeripheralEp0ModeSpec>;
#[doc = "Register `CSR0L_PERIPHERAL_EP0_MODE` writer"]
pub type W = crate::W<Csr0lPeripheralEp0ModeSpec>;
#[doc = "Field `RXPKTRDY` reader - This Bit Is Set When A Data Packet Has Been Received"]
pub type RxpktrdyR = crate::BitReader;
#[doc = "Field `RXPKTRDY` writer - This Bit Is Set When A Data Packet Has Been Received"]
pub type RxpktrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPKTRDY` reader - The CPU Sets This Bit After Loading A Data Packet Into The FIFO."]
pub type TxpktrdyR = crate::BitReader;
#[doc = "Field `TXPKTRDY` writer - The CPU Sets This Bit After Loading A Data Packet Into The FIFO."]
pub type TxpktrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENTSTALL` reader - This Bit Is Set When A Stall Handshake Is Transmitted."]
pub type SentstallR = crate::BitReader;
#[doc = "Field `SENTSTALL` writer - This Bit Is Set When A Stall Handshake Is Transmitted."]
pub type SentstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAEND` reader - The CPU Sets This Bit 1. When Setting TxPktRdy For The Last Data Packet 2. When Clearing RxPktRdy After Unloading The Last Data Packet.3. When Setting TxPktRdy For A Zero Length Data Packet."]
pub type DataendR = crate::BitReader;
#[doc = "Field `DATAEND` writer - The CPU Sets This Bit 1. When Setting TxPktRdy For The Last Data Packet 2. When Clearing RxPktRdy After Unloading The Last Data Packet.3. When Setting TxPktRdy For A Zero Length Data Packet."]
pub type DataendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUPEND` reader - This Bit Will Be Set When A Control Transaction Ends Before The DataEnd Bit Has Been Set"]
pub type SetupendR = crate::BitReader;
#[doc = "Field `SETUPEND` writer - This Bit Will Be Set When A Control Transaction Ends Before The DataEnd Bit Has Been Set"]
pub type SetupendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENDSTALL` reader - The CPU Writes A 1 To This Bit To Terminate The Current Transaction"]
pub type SendstallR = crate::BitReader;
#[doc = "Field `SENDSTALL` writer - The CPU Writes A 1 To This Bit To Terminate The Current Transaction"]
pub type SendstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERVICEDRXPKTRDY` reader - The CPU Writes A 1 To This Bit To Clear The RxPktRdy Bit. It Is Cleared Automatically"]
pub type ServicedrxpktrdyR = crate::BitReader;
#[doc = "Field `SERVICEDRXPKTRDY` writer - The CPU Writes A 1 To This Bit To Clear The RxPktRdy Bit. It Is Cleared Automatically"]
pub type ServicedrxpktrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERVICEDSETUPEND` reader - The CPU Writes A 1 To This Bit To Clear The SetupEnd Bit. It Is Cleared Automatically."]
pub type ServicedsetupendR = crate::BitReader;
#[doc = "Field `SERVICEDSETUPEND` writer - The CPU Writes A 1 To This Bit To Clear The SetupEnd Bit. It Is Cleared Automatically."]
pub type ServicedsetupendW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - This Bit Is Set When A Stall Handshake Is Transmitted."]
    #[inline(always)]
    pub fn sentstall(&self) -> SentstallR {
        SentstallR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The CPU Sets This Bit 1. When Setting TxPktRdy For The Last Data Packet 2. When Clearing RxPktRdy After Unloading The Last Data Packet.3. When Setting TxPktRdy For A Zero Length Data Packet."]
    #[inline(always)]
    pub fn dataend(&self) -> DataendR {
        DataendR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This Bit Will Be Set When A Control Transaction Ends Before The DataEnd Bit Has Been Set"]
    #[inline(always)]
    pub fn setupend(&self) -> SetupendR {
        SetupendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The CPU Writes A 1 To This Bit To Terminate The Current Transaction"]
    #[inline(always)]
    pub fn sendstall(&self) -> SendstallR {
        SendstallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The CPU Writes A 1 To This Bit To Clear The RxPktRdy Bit. It Is Cleared Automatically"]
    #[inline(always)]
    pub fn servicedrxpktrdy(&self) -> ServicedrxpktrdyR {
        ServicedrxpktrdyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The CPU Writes A 1 To This Bit To Clear The SetupEnd Bit. It Is Cleared Automatically."]
    #[inline(always)]
    pub fn servicedsetupend(&self) -> ServicedsetupendR {
        ServicedsetupendR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This Bit Is Set When A Data Packet Has Been Received"]
    #[inline(always)]
    pub fn rxpktrdy(&mut self) -> RxpktrdyW<'_, Csr0lPeripheralEp0ModeSpec> {
        RxpktrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - The CPU Sets This Bit After Loading A Data Packet Into The FIFO."]
    #[inline(always)]
    pub fn txpktrdy(&mut self) -> TxpktrdyW<'_, Csr0lPeripheralEp0ModeSpec> {
        TxpktrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - This Bit Is Set When A Stall Handshake Is Transmitted."]
    #[inline(always)]
    pub fn sentstall(&mut self) -> SentstallW<'_, Csr0lPeripheralEp0ModeSpec> {
        SentstallW::new(self, 2)
    }
    #[doc = "Bit 3 - The CPU Sets This Bit 1. When Setting TxPktRdy For The Last Data Packet 2. When Clearing RxPktRdy After Unloading The Last Data Packet.3. When Setting TxPktRdy For A Zero Length Data Packet."]
    #[inline(always)]
    pub fn dataend(&mut self) -> DataendW<'_, Csr0lPeripheralEp0ModeSpec> {
        DataendW::new(self, 3)
    }
    #[doc = "Bit 4 - This Bit Will Be Set When A Control Transaction Ends Before The DataEnd Bit Has Been Set"]
    #[inline(always)]
    pub fn setupend(&mut self) -> SetupendW<'_, Csr0lPeripheralEp0ModeSpec> {
        SetupendW::new(self, 4)
    }
    #[doc = "Bit 5 - The CPU Writes A 1 To This Bit To Terminate The Current Transaction"]
    #[inline(always)]
    pub fn sendstall(&mut self) -> SendstallW<'_, Csr0lPeripheralEp0ModeSpec> {
        SendstallW::new(self, 5)
    }
    #[doc = "Bit 6 - The CPU Writes A 1 To This Bit To Clear The RxPktRdy Bit. It Is Cleared Automatically"]
    #[inline(always)]
    pub fn servicedrxpktrdy(&mut self) -> ServicedrxpktrdyW<'_, Csr0lPeripheralEp0ModeSpec> {
        ServicedrxpktrdyW::new(self, 6)
    }
    #[doc = "Bit 7 - The CPU Writes A 1 To This Bit To Clear The SetupEnd Bit. It Is Cleared Automatically."]
    #[inline(always)]
    pub fn servicedsetupend(&mut self) -> ServicedsetupendW<'_, Csr0lPeripheralEp0ModeSpec> {
        ServicedsetupendW::new(self, 7)
    }
}
#[doc = "ENDPOINT0 Control And Status Bits for TX Endpoint 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csr0l_peripheral_ep0_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0l_peripheral_ep0_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr0lPeripheralEp0ModeSpec;
impl crate::RegisterSpec for Csr0lPeripheralEp0ModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csr0l_peripheral_ep0_mode::R`](R) reader structure"]
impl crate::Readable for Csr0lPeripheralEp0ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`csr0l_peripheral_ep0_mode::W`](W) writer structure"]
impl crate::Writable for Csr0lPeripheralEp0ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR0L_PERIPHERAL_EP0_MODE to value 0"]
impl crate::Resettable for Csr0lPeripheralEp0ModeSpec {}
