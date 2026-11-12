#[doc = "Register `CSR0H_HOST_EP0_MODE` reader"]
pub type R = crate::R<Csr0hHostEp0ModeSpec>;
#[doc = "Register `CSR0H_HOST_EP0_MODE` writer"]
pub type W = crate::W<Csr0hHostEp0ModeSpec>;
#[doc = "Field `FLUSHFIFO` reader - Flush The Next Packet To Be Transmitted/Read"]
pub type FlushfifoR = crate::BitReader;
#[doc = "Field `FLUSHFIFO` writer - Flush The Next Packet To Be Transmitted/Read"]
pub type FlushfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATOGGLE` reader - When Read, This Bit Indicates The Current State Of The Endpoint 0 Data Toggle"]
pub type DatatoggleR = crate::BitReader;
#[doc = "Field `DATATOGGLE` writer - When Read, This Bit Indicates The Current State Of The Endpoint 0 Data Toggle"]
pub type DatatoggleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATOGGLEWRTENABLE` reader - The CPU Writes A 1 To This Bit To Enable The Current State Of The Endpoint 0 Data Toggle To Be Written"]
pub type DatatogglewrtenableR = crate::BitReader;
#[doc = "Field `DATATOGGLEWRTENABLE` writer - The CPU Writes A 1 To This Bit To Enable The Current State Of The Endpoint 0 Data Toggle To Be Written"]
pub type DatatogglewrtenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISPING` reader - The CPU Writes A 1 To This Bit To Instruct The Core Not To Issue PING Tokens In Data And Status Phases Of A High-Speed Control Transfer"]
pub type DispingR = crate::BitReader;
#[doc = "Field `DISPING` writer - The CPU Writes A 1 To This Bit To Instruct The Core Not To Issue PING Tokens In Data And Status Phases Of A High-Speed Control Transfer"]
pub type DispingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flush The Next Packet To Be Transmitted/Read"]
    #[inline(always)]
    pub fn flushfifo(&self) -> FlushfifoR {
        FlushfifoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When Read, This Bit Indicates The Current State Of The Endpoint 0 Data Toggle"]
    #[inline(always)]
    pub fn datatoggle(&self) -> DatatoggleR {
        DatatoggleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The CPU Writes A 1 To This Bit To Enable The Current State Of The Endpoint 0 Data Toggle To Be Written"]
    #[inline(always)]
    pub fn datatogglewrtenable(&self) -> DatatogglewrtenableR {
        DatatogglewrtenableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The CPU Writes A 1 To This Bit To Instruct The Core Not To Issue PING Tokens In Data And Status Phases Of A High-Speed Control Transfer"]
    #[inline(always)]
    pub fn disping(&self) -> DispingR {
        DispingR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flush The Next Packet To Be Transmitted/Read"]
    #[inline(always)]
    pub fn flushfifo(&mut self) -> FlushfifoW<'_, Csr0hHostEp0ModeSpec> {
        FlushfifoW::new(self, 0)
    }
    #[doc = "Bit 1 - When Read, This Bit Indicates The Current State Of The Endpoint 0 Data Toggle"]
    #[inline(always)]
    pub fn datatoggle(&mut self) -> DatatoggleW<'_, Csr0hHostEp0ModeSpec> {
        DatatoggleW::new(self, 1)
    }
    #[doc = "Bit 2 - The CPU Writes A 1 To This Bit To Enable The Current State Of The Endpoint 0 Data Toggle To Be Written"]
    #[inline(always)]
    pub fn datatogglewrtenable(&mut self) -> DatatogglewrtenableW<'_, Csr0hHostEp0ModeSpec> {
        DatatogglewrtenableW::new(self, 2)
    }
    #[doc = "Bit 3 - The CPU Writes A 1 To This Bit To Instruct The Core Not To Issue PING Tokens In Data And Status Phases Of A High-Speed Control Transfer"]
    #[inline(always)]
    pub fn disping(&mut self) -> DispingW<'_, Csr0hHostEp0ModeSpec> {
        DispingW::new(self, 3)
    }
}
#[doc = "ENDPOINT0 Additional Control And Status Bits For TX Endpoint 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csr0h_host_ep0_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0h_host_ep0_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr0hHostEp0ModeSpec;
impl crate::RegisterSpec for Csr0hHostEp0ModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csr0h_host_ep0_mode::R`](R) reader structure"]
impl crate::Readable for Csr0hHostEp0ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`csr0h_host_ep0_mode::W`](W) writer structure"]
impl crate::Writable for Csr0hHostEp0ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR0H_HOST_EP0_MODE to value 0"]
impl crate::Resettable for Csr0hHostEp0ModeSpec {}
