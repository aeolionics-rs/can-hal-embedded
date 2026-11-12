#[doc = "Register `TXCSRH_HOST_EPX_MODE` reader"]
pub type R = crate::R<TxcsrhHostEpxModeSpec>;
#[doc = "Register `TXCSRH_HOST_EPX_MODE` writer"]
pub type W = crate::W<TxcsrhHostEpxModeSpec>;
#[doc = "Field `DATATOGGLE` reader - When Read, This Bit Indicates The Current State Of The Endpoint"]
pub type DatatoggleR = crate::BitReader;
#[doc = "Field `DATATOGGLE` writer - When Read, This Bit Indicates The Current State Of The Endpoint"]
pub type DatatoggleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATOGGLEWRENABLE` reader - The CPU Writes A 1 To This Bit To Enable The Current State Of The Endpoint 0 Data Toggle To Be Written"]
pub type DatatogglewrenableR = crate::BitReader;
#[doc = "Field `DATATOGGLEWRENABLE` writer - The CPU Writes A 1 To This Bit To Enable The Current State Of The Endpoint 0 Data Toggle To Be Written"]
pub type DatatogglewrenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREQMODE` reader - The CPU Sets This Bit To Select DMA Request Mode"]
pub type DmareqmodeR = crate::BitReader;
#[doc = "Field `DMAREQMODE` writer - The CPU Sets This Bit To Select DMA Request Mode"]
pub type DmareqmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRCDATATOG` reader - The CPU Sets This Bit To Force The Endpoint Data Toggle To Switch And The Data Packe"]
pub type FrcdatatogR = crate::BitReader;
#[doc = "Field `FRCDATATOG` writer - The CPU Sets This Bit To Force The Endpoint Data Toggle To Switch And The Data Packe"]
pub type FrcdatatogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREQENAB` reader - The CPU Sets This Bit To Enable The DMA Request For The RX Endpoint"]
pub type DmareqenabR = crate::BitReader;
#[doc = "Field `DMAREQENAB` writer - The CPU Sets This Bit To Enable The DMA Request For The RX Endpoint"]
pub type DmareqenabW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - The CPU Sets This Bit To Enable The Endpoint Direction As TX"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - The CPU Sets This Bit To Enable The Endpoint Direction As TX"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOSET` reader - If The CPU Sets This Bit, TxPktRdy Will Be Automatically Set When Data Of The Maximum P"]
pub type AutosetR = crate::BitReader;
#[doc = "Field `AUTOSET` writer - If The CPU Sets This Bit, TxPktRdy Will Be Automatically Set When Data Of The Maximum P"]
pub type AutosetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When Read, This Bit Indicates The Current State Of The Endpoint"]
    #[inline(always)]
    pub fn datatoggle(&self) -> DatatoggleR {
        DatatoggleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The CPU Writes A 1 To This Bit To Enable The Current State Of The Endpoint 0 Data Toggle To Be Written"]
    #[inline(always)]
    pub fn datatogglewrenable(&self) -> DatatogglewrenableR {
        DatatogglewrenableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The CPU Sets This Bit To Select DMA Request Mode"]
    #[inline(always)]
    pub fn dmareqmode(&self) -> DmareqmodeR {
        DmareqmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The CPU Sets This Bit To Force The Endpoint Data Toggle To Switch And The Data Packe"]
    #[inline(always)]
    pub fn frcdatatog(&self) -> FrcdatatogR {
        FrcdatatogR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The CPU Sets This Bit To Enable The DMA Request For The RX Endpoint"]
    #[inline(always)]
    pub fn dmareqenab(&self) -> DmareqenabR {
        DmareqenabR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The CPU Sets This Bit To Enable The Endpoint Direction As TX"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - If The CPU Sets This Bit, TxPktRdy Will Be Automatically Set When Data Of The Maximum P"]
    #[inline(always)]
    pub fn autoset(&self) -> AutosetR {
        AutosetR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When Read, This Bit Indicates The Current State Of The Endpoint"]
    #[inline(always)]
    pub fn datatoggle(&mut self) -> DatatoggleW<'_, TxcsrhHostEpxModeSpec> {
        DatatoggleW::new(self, 0)
    }
    #[doc = "Bit 1 - The CPU Writes A 1 To This Bit To Enable The Current State Of The Endpoint 0 Data Toggle To Be Written"]
    #[inline(always)]
    pub fn datatogglewrenable(&mut self) -> DatatogglewrenableW<'_, TxcsrhHostEpxModeSpec> {
        DatatogglewrenableW::new(self, 1)
    }
    #[doc = "Bit 2 - The CPU Sets This Bit To Select DMA Request Mode"]
    #[inline(always)]
    pub fn dmareqmode(&mut self) -> DmareqmodeW<'_, TxcsrhHostEpxModeSpec> {
        DmareqmodeW::new(self, 2)
    }
    #[doc = "Bit 3 - The CPU Sets This Bit To Force The Endpoint Data Toggle To Switch And The Data Packe"]
    #[inline(always)]
    pub fn frcdatatog(&mut self) -> FrcdatatogW<'_, TxcsrhHostEpxModeSpec> {
        FrcdatatogW::new(self, 3)
    }
    #[doc = "Bit 4 - The CPU Sets This Bit To Enable The DMA Request For The RX Endpoint"]
    #[inline(always)]
    pub fn dmareqenab(&mut self) -> DmareqenabW<'_, TxcsrhHostEpxModeSpec> {
        DmareqenabW::new(self, 4)
    }
    #[doc = "Bit 5 - The CPU Sets This Bit To Enable The Endpoint Direction As TX"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, TxcsrhHostEpxModeSpec> {
        ModeW::new(self, 5)
    }
    #[doc = "Bit 7 - If The CPU Sets This Bit, TxPktRdy Will Be Automatically Set When Data Of The Maximum P"]
    #[inline(always)]
    pub fn autoset(&mut self) -> AutosetW<'_, TxcsrhHostEpxModeSpec> {
        AutosetW::new(self, 7)
    }
}
#[doc = "ENDPOINTX Additional Control And Status Bits For TX Endpoints 1-15\n\nYou can [`read`](crate::Reg::read) this register and get [`txcsrh_host_epx_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txcsrh_host_epx_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxcsrhHostEpxModeSpec;
impl crate::RegisterSpec for TxcsrhHostEpxModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txcsrh_host_epx_mode::R`](R) reader structure"]
impl crate::Readable for TxcsrhHostEpxModeSpec {}
#[doc = "`write(|w| ..)` method takes [`txcsrh_host_epx_mode::W`](W) writer structure"]
impl crate::Writable for TxcsrhHostEpxModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXCSRH_HOST_EPX_MODE to value 0"]
impl crate::Resettable for TxcsrhHostEpxModeSpec {}
