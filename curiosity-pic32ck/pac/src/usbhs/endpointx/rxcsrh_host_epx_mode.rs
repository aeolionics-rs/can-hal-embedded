#[doc = "Register `RXCSRH_HOST_EPX_MODE` reader"]
pub type R = crate::R<RxcsrhHostEpxModeSpec>;
#[doc = "Register `RXCSRH_HOST_EPX_MODE` writer"]
pub type W = crate::W<RxcsrhHostEpxModeSpec>;
#[doc = "Field `INCOMPRX` reader - This Bit Is Set In A High-Bandwidth Isochronous/Interrupt Transfer If The Packet In The Rx FIFO Is Incomplete"]
pub type IncomprxR = crate::BitReader;
#[doc = "Field `INCOMPRX` writer - This Bit Is Set In A High-Bandwidth Isochronous/Interrupt Transfer If The Packet In The Rx FIFO Is Incomplete"]
pub type IncomprxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATOGGLE` reader - When Read, This Bit Indicates The Current State Of The Endpoint"]
pub type DatatoggleR = crate::BitReader;
#[doc = "Field `DATATOGGLE` writer - When Read, This Bit Indicates The Current State Of The Endpoint"]
pub type DatatoggleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATOGGLEWRTENABLE` reader - The CPU Writes A 1 To This Bit To Enable The Current State Of The Endpoint 0 Data Toggle To Be Written"]
pub type DatatogglewrtenableR = crate::BitReader;
#[doc = "Field `DATATOGGLEWRTENABLE` writer - The CPU Writes A 1 To This Bit To Enable The Current State Of The Endpoint 0 Data Toggle To Be Written"]
pub type DatatogglewrtenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREQMODE` reader - The CPU Sets This Bit To Select DMA Request Mode"]
pub type DmareqmodeR = crate::BitReader;
#[doc = "Field `DMAREQMODE` writer - The CPU Sets This Bit To Select DMA Request Mode"]
pub type DmareqmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIDERROR` reader - Indicates A PID Error In The Received Packet"]
pub type PiderrorR = crate::BitReader;
#[doc = "Field `PIDERROR` writer - Indicates A PID Error In The Received Packet"]
pub type PiderrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREQENAB` reader - The CPU Sets This Bit To Enable The DMA Request For The RX Endpoint"]
pub type DmareqenabR = crate::BitReader;
#[doc = "Field `DMAREQENAB` writer - The CPU Sets This Bit To Enable The DMA Request For The RX Endpoint"]
pub type DmareqenabW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOREQ` reader - If The CPU Sets This Bit, The ReqPkt Bit Will Be Automatically Set When The RxPktRdy Bit Is Cleared."]
pub type AutoreqR = crate::BitReader;
#[doc = "Field `AUTOREQ` writer - If The CPU Sets This Bit, The ReqPkt Bit Will Be Automatically Set When The RxPktRdy Bit Is Cleared."]
pub type AutoreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCLEAR` reader - If The CPU Sets This Bit, TxPktRdy Will Be Automatically Cleared When A Packet Of RxMaxP Bytes Has Been Unloaded From The RX FIFO"]
pub type AutoclearR = crate::BitReader;
#[doc = "Field `AUTOCLEAR` writer - If The CPU Sets This Bit, TxPktRdy Will Be Automatically Cleared When A Packet Of RxMaxP Bytes Has Been Unloaded From The RX FIFO"]
pub type AutoclearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This Bit Is Set In A High-Bandwidth Isochronous/Interrupt Transfer If The Packet In The Rx FIFO Is Incomplete"]
    #[inline(always)]
    pub fn incomprx(&self) -> IncomprxR {
        IncomprxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When Read, This Bit Indicates The Current State Of The Endpoint"]
    #[inline(always)]
    pub fn datatoggle(&self) -> DatatoggleR {
        DatatoggleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The CPU Writes A 1 To This Bit To Enable The Current State Of The Endpoint 0 Data Toggle To Be Written"]
    #[inline(always)]
    pub fn datatogglewrtenable(&self) -> DatatogglewrtenableR {
        DatatogglewrtenableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The CPU Sets This Bit To Select DMA Request Mode"]
    #[inline(always)]
    pub fn dmareqmode(&self) -> DmareqmodeR {
        DmareqmodeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates A PID Error In The Received Packet"]
    #[inline(always)]
    pub fn piderror(&self) -> PiderrorR {
        PiderrorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The CPU Sets This Bit To Enable The DMA Request For The RX Endpoint"]
    #[inline(always)]
    pub fn dmareqenab(&self) -> DmareqenabR {
        DmareqenabR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If The CPU Sets This Bit, The ReqPkt Bit Will Be Automatically Set When The RxPktRdy Bit Is Cleared."]
    #[inline(always)]
    pub fn autoreq(&self) -> AutoreqR {
        AutoreqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If The CPU Sets This Bit, TxPktRdy Will Be Automatically Cleared When A Packet Of RxMaxP Bytes Has Been Unloaded From The RX FIFO"]
    #[inline(always)]
    pub fn autoclear(&self) -> AutoclearR {
        AutoclearR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This Bit Is Set In A High-Bandwidth Isochronous/Interrupt Transfer If The Packet In The Rx FIFO Is Incomplete"]
    #[inline(always)]
    pub fn incomprx(&mut self) -> IncomprxW<'_, RxcsrhHostEpxModeSpec> {
        IncomprxW::new(self, 0)
    }
    #[doc = "Bit 1 - When Read, This Bit Indicates The Current State Of The Endpoint"]
    #[inline(always)]
    pub fn datatoggle(&mut self) -> DatatoggleW<'_, RxcsrhHostEpxModeSpec> {
        DatatoggleW::new(self, 1)
    }
    #[doc = "Bit 2 - The CPU Writes A 1 To This Bit To Enable The Current State Of The Endpoint 0 Data Toggle To Be Written"]
    #[inline(always)]
    pub fn datatogglewrtenable(&mut self) -> DatatogglewrtenableW<'_, RxcsrhHostEpxModeSpec> {
        DatatogglewrtenableW::new(self, 2)
    }
    #[doc = "Bit 3 - The CPU Sets This Bit To Select DMA Request Mode"]
    #[inline(always)]
    pub fn dmareqmode(&mut self) -> DmareqmodeW<'_, RxcsrhHostEpxModeSpec> {
        DmareqmodeW::new(self, 3)
    }
    #[doc = "Bit 4 - Indicates A PID Error In The Received Packet"]
    #[inline(always)]
    pub fn piderror(&mut self) -> PiderrorW<'_, RxcsrhHostEpxModeSpec> {
        PiderrorW::new(self, 4)
    }
    #[doc = "Bit 5 - The CPU Sets This Bit To Enable The DMA Request For The RX Endpoint"]
    #[inline(always)]
    pub fn dmareqenab(&mut self) -> DmareqenabW<'_, RxcsrhHostEpxModeSpec> {
        DmareqenabW::new(self, 5)
    }
    #[doc = "Bit 6 - If The CPU Sets This Bit, The ReqPkt Bit Will Be Automatically Set When The RxPktRdy Bit Is Cleared."]
    #[inline(always)]
    pub fn autoreq(&mut self) -> AutoreqW<'_, RxcsrhHostEpxModeSpec> {
        AutoreqW::new(self, 6)
    }
    #[doc = "Bit 7 - If The CPU Sets This Bit, TxPktRdy Will Be Automatically Cleared When A Packet Of RxMaxP Bytes Has Been Unloaded From The RX FIFO"]
    #[inline(always)]
    pub fn autoclear(&mut self) -> AutoclearW<'_, RxcsrhHostEpxModeSpec> {
        AutoclearW::new(self, 7)
    }
}
#[doc = "ENDPOINTX Additional Control And Status Bits for RX Endpoint 0 Or Endpoint 1-15\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcsrh_host_epx_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxcsrh_host_epx_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxcsrhHostEpxModeSpec;
impl crate::RegisterSpec for RxcsrhHostEpxModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxcsrh_host_epx_mode::R`](R) reader structure"]
impl crate::Readable for RxcsrhHostEpxModeSpec {}
#[doc = "`write(|w| ..)` method takes [`rxcsrh_host_epx_mode::W`](W) writer structure"]
impl crate::Writable for RxcsrhHostEpxModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXCSRH_HOST_EPX_MODE to value 0"]
impl crate::Resettable for RxcsrhHostEpxModeSpec {}
