#[doc = "Register `TESTMODE` reader"]
pub type R = crate::R<TestmodeSpec>;
#[doc = "Register `TESTMODE` writer"]
pub type W = crate::W<TestmodeSpec>;
#[doc = "Field `TESTSE0NAK` reader - Set To Enter The Test_SE0_NAK mode"]
pub type Testse0nakR = crate::BitReader;
#[doc = "Field `TESTSE0NAK` writer - Set To Enter The Test_SE0_NAK mode"]
pub type Testse0nakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TESTJ` reader - Set To Enter The Test_J mode"]
pub type TestjR = crate::BitReader;
#[doc = "Field `TESTJ` writer - Set To Enter The Test_J mode"]
pub type TestjW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TESTK` reader - Set To Enter The Test_K mode"]
pub type TestkR = crate::BitReader;
#[doc = "Field `TESTK` writer - Set To Enter The Test_K mode"]
pub type TestkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TESTPACKET` reader - Set To Enter The Test_Packet mode"]
pub type TestpacketR = crate::BitReader;
#[doc = "Field `TESTPACKET` writer - Set To Enter The Test_Packet mode"]
pub type TestpacketW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEHS` reader - Depending On Bit 7 Force Controller Into High Speed Mode"]
pub type ForcehsR = crate::BitReader;
#[doc = "Field `FORCEHS` writer - Depending On Bit 7 Force Controller Into High Speed Mode"]
pub type ForcehsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEFS` reader - Depending On Bit 7 Force Controller Into FS Speed Mode"]
pub type ForcefsR = crate::BitReader;
#[doc = "Field `FORCEFS` writer - Depending On Bit 7 Force Controller Into FS Speed Mode"]
pub type ForcefsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOACCESS` reader - The CPU Sets This Bit To Transfer The Packet In The Endpoint 0 TX FIFO To The Endpoint 0 Rx FIFO. It Is Cleared Automatically."]
pub type FifoaccessR = crate::BitReader;
#[doc = "Field `FIFOACCESS` writer - The CPU Sets This Bit To Transfer The Packet In The Endpoint 0 TX FIFO To The Endpoint 0 Rx FIFO. It Is Cleared Automatically."]
pub type FifoaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEHOST` reader - The CPU Sets This Bit To Instruct The Core To Enter Host Mode When The Session Bit Is Set"]
pub type ForcehostR = crate::BitReader;
#[doc = "Field `FORCEHOST` writer - The CPU Sets This Bit To Instruct The Core To Enter Host Mode When The Session Bit Is Set"]
pub type ForcehostW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set To Enter The Test_SE0_NAK mode"]
    #[inline(always)]
    pub fn testse0nak(&self) -> Testse0nakR {
        Testse0nakR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set To Enter The Test_J mode"]
    #[inline(always)]
    pub fn testj(&self) -> TestjR {
        TestjR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set To Enter The Test_K mode"]
    #[inline(always)]
    pub fn testk(&self) -> TestkR {
        TestkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set To Enter The Test_Packet mode"]
    #[inline(always)]
    pub fn testpacket(&self) -> TestpacketR {
        TestpacketR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Depending On Bit 7 Force Controller Into High Speed Mode"]
    #[inline(always)]
    pub fn forcehs(&self) -> ForcehsR {
        ForcehsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Depending On Bit 7 Force Controller Into FS Speed Mode"]
    #[inline(always)]
    pub fn forcefs(&self) -> ForcefsR {
        ForcefsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The CPU Sets This Bit To Transfer The Packet In The Endpoint 0 TX FIFO To The Endpoint 0 Rx FIFO. It Is Cleared Automatically."]
    #[inline(always)]
    pub fn fifoaccess(&self) -> FifoaccessR {
        FifoaccessR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The CPU Sets This Bit To Instruct The Core To Enter Host Mode When The Session Bit Is Set"]
    #[inline(always)]
    pub fn forcehost(&self) -> ForcehostR {
        ForcehostR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set To Enter The Test_SE0_NAK mode"]
    #[inline(always)]
    pub fn testse0nak(&mut self) -> Testse0nakW<'_, TestmodeSpec> {
        Testse0nakW::new(self, 0)
    }
    #[doc = "Bit 1 - Set To Enter The Test_J mode"]
    #[inline(always)]
    pub fn testj(&mut self) -> TestjW<'_, TestmodeSpec> {
        TestjW::new(self, 1)
    }
    #[doc = "Bit 2 - Set To Enter The Test_K mode"]
    #[inline(always)]
    pub fn testk(&mut self) -> TestkW<'_, TestmodeSpec> {
        TestkW::new(self, 2)
    }
    #[doc = "Bit 3 - Set To Enter The Test_Packet mode"]
    #[inline(always)]
    pub fn testpacket(&mut self) -> TestpacketW<'_, TestmodeSpec> {
        TestpacketW::new(self, 3)
    }
    #[doc = "Bit 4 - Depending On Bit 7 Force Controller Into High Speed Mode"]
    #[inline(always)]
    pub fn forcehs(&mut self) -> ForcehsW<'_, TestmodeSpec> {
        ForcehsW::new(self, 4)
    }
    #[doc = "Bit 5 - Depending On Bit 7 Force Controller Into FS Speed Mode"]
    #[inline(always)]
    pub fn forcefs(&mut self) -> ForcefsW<'_, TestmodeSpec> {
        ForcefsW::new(self, 5)
    }
    #[doc = "Bit 6 - The CPU Sets This Bit To Transfer The Packet In The Endpoint 0 TX FIFO To The Endpoint 0 Rx FIFO. It Is Cleared Automatically."]
    #[inline(always)]
    pub fn fifoaccess(&mut self) -> FifoaccessW<'_, TestmodeSpec> {
        FifoaccessW::new(self, 6)
    }
    #[doc = "Bit 7 - The CPU Sets This Bit To Instruct The Core To Enter Host Mode When The Session Bit Is Set"]
    #[inline(always)]
    pub fn forcehost(&mut self) -> ForcehostW<'_, TestmodeSpec> {
        ForcehostW::new(self, 7)
    }
}
#[doc = "Not Used In Normal Operation. Configuration To Put the USBCORE Into One Of The Four Test Modes For HighSpeed Operation\n\nYou can [`read`](crate::Reg::read) this register and get [`testmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestmodeSpec;
impl crate::RegisterSpec for TestmodeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`testmode::R`](R) reader structure"]
impl crate::Readable for TestmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`testmode::W`](W) writer structure"]
impl crate::Writable for TestmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TESTMODE to value 0"]
impl crate::Resettable for TestmodeSpec {}
