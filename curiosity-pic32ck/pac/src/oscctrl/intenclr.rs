#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `XOSCRDY` reader - XOSC Ready Interrupt Enable"]
pub type XoscrdyR = crate::BitReader;
#[doc = "Field `XOSCRDY` writer - XOSC Ready Interrupt Enable"]
pub type XoscrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `XOSCFAIL` reader - XOSC Startup Failure Interrupt Enable"]
pub type XoscfailR = crate::BitReader;
#[doc = "Field `XOSCFAIL` writer - XOSC Startup Failure Interrupt Enable"]
pub type XoscfailW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLKFAIL` reader - XOSC Clock Failure Interrupt Enable"]
pub type ClkfailR = crate::BitReader;
#[doc = "Field `CLKFAIL` writer - XOSC Clock Failure Interrupt Enable"]
pub type ClkfailW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DFLLRDY` reader - DFLL Ready Interrupt Enable"]
pub type DfllrdyR = crate::BitReader;
#[doc = "Field `DFLLRDY` writer - DFLL Ready Interrupt Enable"]
pub type DfllrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DFLLLOCK` reader - DFLL Lock Interrupt Enable"]
pub type DflllockR = crate::BitReader;
#[doc = "Field `DFLLLOCK` writer - DFLL Lock Interrupt Enable"]
pub type DflllockW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DFLLOVF` reader - DFLL Tuner Overflow Interrupt Enable"]
pub type DfllovfR = crate::BitReader;
#[doc = "Field `DFLLOVF` writer - DFLL Tuner Overflow Interrupt Enable"]
pub type DfllovfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DFLLUNF` reader - DFLL Tuner Underflow Interrupt Enable"]
pub type DfllunfR = crate::BitReader;
#[doc = "Field `DFLLUNF` writer - DFLL Tuner Underflow Interrupt Enable"]
pub type DfllunfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped Interrupt Enable"]
pub type DfllrcsR = crate::BitReader;
#[doc = "Field `DFLLRCS` writer - DFLL Reference Clock Stopped Interrupt Enable"]
pub type DfllrcsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DFLLFAIL` reader - DFLL Startup Failure Interrupt Enable"]
pub type DfllfailR = crate::BitReader;
#[doc = "Field `DFLLFAIL` writer - DFLL Startup Failure Interrupt Enable"]
pub type DfllfailW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PLL0LOCKR` reader - PLL 0 Lock Rise Interrupt Enable"]
pub type Pll0lockrR = crate::BitReader;
#[doc = "Field `PLL0LOCKR` writer - PLL 0 Lock Rise Interrupt Enable"]
pub type Pll0lockrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XoscrdyR {
        XoscrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XOSC Startup Failure Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail(&self) -> XoscfailR {
        XoscfailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC Clock Failure Interrupt Enable"]
    #[inline(always)]
    pub fn clkfail(&self) -> ClkfailR {
        ClkfailR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DfllrdyR {
        DfllrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DFLL Lock Interrupt Enable"]
    #[inline(always)]
    pub fn dflllock(&self) -> DflllockR {
        DflllockR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DFLL Tuner Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn dfllovf(&self) -> DfllovfR {
        DfllovfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DFLL Tuner Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn dfllunf(&self) -> DfllunfR {
        DfllunfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DfllrcsR {
        DfllrcsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DFLL Startup Failure Interrupt Enable"]
    #[inline(always)]
    pub fn dfllfail(&self) -> DfllfailR {
        DfllfailR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL 0 Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn pll0lockr(&self) -> Pll0lockrR {
        Pll0lockrR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy(&mut self) -> XoscrdyW<'_, IntenclrSpec> {
        XoscrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - XOSC Startup Failure Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail(&mut self) -> XoscfailW<'_, IntenclrSpec> {
        XoscfailW::new(self, 1)
    }
    #[doc = "Bit 2 - XOSC Clock Failure Interrupt Enable"]
    #[inline(always)]
    pub fn clkfail(&mut self) -> ClkfailW<'_, IntenclrSpec> {
        ClkfailW::new(self, 2)
    }
    #[doc = "Bit 8 - DFLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrdy(&mut self) -> DfllrdyW<'_, IntenclrSpec> {
        DfllrdyW::new(self, 8)
    }
    #[doc = "Bit 9 - DFLL Lock Interrupt Enable"]
    #[inline(always)]
    pub fn dflllock(&mut self) -> DflllockW<'_, IntenclrSpec> {
        DflllockW::new(self, 9)
    }
    #[doc = "Bit 10 - DFLL Tuner Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn dfllovf(&mut self) -> DfllovfW<'_, IntenclrSpec> {
        DfllovfW::new(self, 10)
    }
    #[doc = "Bit 11 - DFLL Tuner Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn dfllunf(&mut self) -> DfllunfW<'_, IntenclrSpec> {
        DfllunfW::new(self, 11)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped Interrupt Enable"]
    #[inline(always)]
    pub fn dfllrcs(&mut self) -> DfllrcsW<'_, IntenclrSpec> {
        DfllrcsW::new(self, 12)
    }
    #[doc = "Bit 13 - DFLL Startup Failure Interrupt Enable"]
    #[inline(always)]
    pub fn dfllfail(&mut self) -> DfllfailW<'_, IntenclrSpec> {
        DfllfailW::new(self, 13)
    }
    #[doc = "Bit 24 - PLL 0 Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn pll0lockr(&mut self) -> Pll0lockrW<'_, IntenclrSpec> {
        Pll0lockrW::new(self, 24)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0100_3f07;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
