#[doc = "Register `HSMINTFLAG` reader"]
pub type R = crate::R<HsmintflagSpec>;
#[doc = "Register `HSMINTFLAG` writer"]
pub type W = crate::W<HsmintflagSpec>;
#[doc = "Field `DONE` reader - NVM Operation Done Flag Bit"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - NVM Operation Done Flag Bit"]
pub type DoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `KEYERR` reader - Key Error Flag Bit"]
pub type KeyerrR = crate::BitReader;
#[doc = "Field `KEYERR` writer - Key Error Flag Bit"]
pub type KeyerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CFGERR` reader - Configuration Error Flag Bit"]
pub type CfgerrR = crate::BitReader;
#[doc = "Field `CFGERR` writer - Configuration Error Flag Bit"]
pub type CfgerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FIFOERR` reader - FIFO Underrun during Row Write Flag Bit"]
pub type FifoerrR = crate::BitReader;
#[doc = "Field `FIFOERR` writer - FIFO Underrun during Row Write Flag Bit"]
pub type FifoerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BUSERR` reader - AHB Bus Error during Row Write Flag Bit"]
pub type BuserrR = crate::BitReader;
#[doc = "Field `BUSERR` writer - AHB Bus Error during Row Write Flag Bit"]
pub type BuserrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WPERR` reader - Write Protection Error Flag Bit"]
pub type WperrR = crate::BitReader;
#[doc = "Field `WPERR` writer - Write Protection Error Flag Bit"]
pub type WperrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OPERR` reader - NVMOP Error Flag Bit"]
pub type OperrR = crate::BitReader;
#[doc = "Field `OPERR` writer - NVMOP Error Flag Bit"]
pub type OperrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SECERR` reader - Security Violation Error Bit"]
pub type SecerrR = crate::BitReader;
#[doc = "Field `SECERR` writer - Security Violation Error Bit"]
pub type SecerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HTDPGM` reader - High Temperature Detect Error Flag Bit"]
pub type HtdpgmR = crate::BitReader;
#[doc = "Field `HTDPGM` writer - High Temperature Detect Error Flag Bit"]
pub type HtdpgmW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BORERR` reader - Brown Out Detect Error Flag Bit"]
pub type BorerrR = crate::BitReader;
#[doc = "Field `BORERR` writer - Brown Out Detect Error Flag Bit"]
pub type BorerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WRERR` reader - Write Error Flag Bit"]
pub type WrerrR = crate::BitReader;
#[doc = "Field `WRERR` writer - Write Error Flag Bit"]
pub type WrerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - NVM Operation Done Flag Bit"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key Error Flag Bit"]
    #[inline(always)]
    pub fn keyerr(&self) -> KeyerrR {
        KeyerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configuration Error Flag Bit"]
    #[inline(always)]
    pub fn cfgerr(&self) -> CfgerrR {
        CfgerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO Underrun during Row Write Flag Bit"]
    #[inline(always)]
    pub fn fifoerr(&self) -> FifoerrR {
        FifoerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB Bus Error during Row Write Flag Bit"]
    #[inline(always)]
    pub fn buserr(&self) -> BuserrR {
        BuserrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Protection Error Flag Bit"]
    #[inline(always)]
    pub fn wperr(&self) -> WperrR {
        WperrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NVMOP Error Flag Bit"]
    #[inline(always)]
    pub fn operr(&self) -> OperrR {
        OperrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Security Violation Error Bit"]
    #[inline(always)]
    pub fn secerr(&self) -> SecerrR {
        SecerrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Temperature Detect Error Flag Bit"]
    #[inline(always)]
    pub fn htdpgm(&self) -> HtdpgmR {
        HtdpgmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Brown Out Detect Error Flag Bit"]
    #[inline(always)]
    pub fn borerr(&self) -> BorerrR {
        BorerrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write Error Flag Bit"]
    #[inline(always)]
    pub fn wrerr(&self) -> WrerrR {
        WrerrR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NVM Operation Done Flag Bit"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, HsmintflagSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Key Error Flag Bit"]
    #[inline(always)]
    pub fn keyerr(&mut self) -> KeyerrW<'_, HsmintflagSpec> {
        KeyerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Configuration Error Flag Bit"]
    #[inline(always)]
    pub fn cfgerr(&mut self) -> CfgerrW<'_, HsmintflagSpec> {
        CfgerrW::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO Underrun during Row Write Flag Bit"]
    #[inline(always)]
    pub fn fifoerr(&mut self) -> FifoerrW<'_, HsmintflagSpec> {
        FifoerrW::new(self, 3)
    }
    #[doc = "Bit 4 - AHB Bus Error during Row Write Flag Bit"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BuserrW<'_, HsmintflagSpec> {
        BuserrW::new(self, 4)
    }
    #[doc = "Bit 5 - Write Protection Error Flag Bit"]
    #[inline(always)]
    pub fn wperr(&mut self) -> WperrW<'_, HsmintflagSpec> {
        WperrW::new(self, 5)
    }
    #[doc = "Bit 6 - NVMOP Error Flag Bit"]
    #[inline(always)]
    pub fn operr(&mut self) -> OperrW<'_, HsmintflagSpec> {
        OperrW::new(self, 6)
    }
    #[doc = "Bit 7 - Security Violation Error Bit"]
    #[inline(always)]
    pub fn secerr(&mut self) -> SecerrW<'_, HsmintflagSpec> {
        SecerrW::new(self, 7)
    }
    #[doc = "Bit 8 - High Temperature Detect Error Flag Bit"]
    #[inline(always)]
    pub fn htdpgm(&mut self) -> HtdpgmW<'_, HsmintflagSpec> {
        HtdpgmW::new(self, 8)
    }
    #[doc = "Bit 12 - Brown Out Detect Error Flag Bit"]
    #[inline(always)]
    pub fn borerr(&mut self) -> BorerrW<'_, HsmintflagSpec> {
        BorerrW::new(self, 12)
    }
    #[doc = "Bit 13 - Write Error Flag Bit"]
    #[inline(always)]
    pub fn wrerr(&mut self) -> WrerrW<'_, HsmintflagSpec> {
        WrerrW::new(self, 13)
    }
}
#[doc = "HSM NVM Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsmintflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsmintflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsmintflagSpec;
impl crate::RegisterSpec for HsmintflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsmintflag::R`](R) reader structure"]
impl crate::Readable for HsmintflagSpec {}
#[doc = "`write(|w| ..)` method takes [`hsmintflag::W`](W) writer structure"]
impl crate::Writable for HsmintflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x31ff;
}
#[doc = "`reset()` method sets HSMINTFLAG to value 0"]
impl crate::Resettable for HsmintflagSpec {}
