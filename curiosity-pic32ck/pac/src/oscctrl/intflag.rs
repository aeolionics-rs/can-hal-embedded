#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `XOSCRDY` reader - XOSC Ready"]
pub type XoscrdyR = crate::BitReader;
#[doc = "Field `XOSCRDY` writer - XOSC Ready"]
pub type XoscrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `XOSCFAIL` reader - XOSC Startup Failure"]
pub type XoscfailR = crate::BitReader;
#[doc = "Field `XOSCFAIL` writer - XOSC Startup Failure"]
pub type XoscfailW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLKFAIL` reader - XOSC Clock Failure"]
pub type ClkfailR = crate::BitReader;
#[doc = "Field `CLKFAIL` writer - XOSC Clock Failure"]
pub type ClkfailW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DFLLRDY` reader - DFLL Ready"]
pub type DfllrdyR = crate::BitReader;
#[doc = "Field `DFLLRDY` writer - DFLL Ready"]
pub type DfllrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DFLLLOCK` reader - DFLL Lock"]
pub type DflllockR = crate::BitReader;
#[doc = "Field `DFLLLOCK` writer - DFLL Lock"]
pub type DflllockW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DFLLOVF` reader - DFLL Tuner Overflow"]
pub type DfllovfR = crate::BitReader;
#[doc = "Field `DFLLOVF` writer - DFLL Tuner Overflow"]
pub type DfllovfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DFLLUNF` reader - DFLL Tuner Underflow"]
pub type DfllunfR = crate::BitReader;
#[doc = "Field `DFLLUNF` writer - DFLL Tuner Underflow"]
pub type DfllunfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped"]
pub type DfllrcsR = crate::BitReader;
#[doc = "Field `DFLLRCS` writer - DFLL Reference Clock Stopped"]
pub type DfllrcsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DFLLFAIL` reader - DFLL Startup Failure"]
pub type DfllfailR = crate::BitReader;
#[doc = "Field `DFLLFAIL` writer - DFLL Startup Failure"]
pub type DfllfailW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PLL0LOCKR` reader - PLL 0 Lock Rise"]
pub type Pll0lockrR = crate::BitReader;
#[doc = "Field `PLL0LOCKR` writer - PLL 0 Lock Rise"]
pub type Pll0lockrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XoscrdyR {
        XoscrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XOSC Startup Failure"]
    #[inline(always)]
    pub fn xoscfail(&self) -> XoscfailR {
        XoscfailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC Clock Failure"]
    #[inline(always)]
    pub fn clkfail(&self) -> ClkfailR {
        ClkfailR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - DFLL Ready"]
    #[inline(always)]
    pub fn dfllrdy(&self) -> DfllrdyR {
        DfllrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DFLL Lock"]
    #[inline(always)]
    pub fn dflllock(&self) -> DflllockR {
        DflllockR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DFLL Tuner Overflow"]
    #[inline(always)]
    pub fn dfllovf(&self) -> DfllovfR {
        DfllovfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DFLL Tuner Underflow"]
    #[inline(always)]
    pub fn dfllunf(&self) -> DfllunfR {
        DfllunfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfllrcs(&self) -> DfllrcsR {
        DfllrcsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DFLL Startup Failure"]
    #[inline(always)]
    pub fn dfllfail(&self) -> DfllfailR {
        DfllfailR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL 0 Lock Rise"]
    #[inline(always)]
    pub fn pll0lockr(&self) -> Pll0lockrR {
        Pll0lockrR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    pub fn xoscrdy(&mut self) -> XoscrdyW<'_, IntflagSpec> {
        XoscrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - XOSC Startup Failure"]
    #[inline(always)]
    pub fn xoscfail(&mut self) -> XoscfailW<'_, IntflagSpec> {
        XoscfailW::new(self, 1)
    }
    #[doc = "Bit 2 - XOSC Clock Failure"]
    #[inline(always)]
    pub fn clkfail(&mut self) -> ClkfailW<'_, IntflagSpec> {
        ClkfailW::new(self, 2)
    }
    #[doc = "Bit 8 - DFLL Ready"]
    #[inline(always)]
    pub fn dfllrdy(&mut self) -> DfllrdyW<'_, IntflagSpec> {
        DfllrdyW::new(self, 8)
    }
    #[doc = "Bit 9 - DFLL Lock"]
    #[inline(always)]
    pub fn dflllock(&mut self) -> DflllockW<'_, IntflagSpec> {
        DflllockW::new(self, 9)
    }
    #[doc = "Bit 10 - DFLL Tuner Overflow"]
    #[inline(always)]
    pub fn dfllovf(&mut self) -> DfllovfW<'_, IntflagSpec> {
        DfllovfW::new(self, 10)
    }
    #[doc = "Bit 11 - DFLL Tuner Underflow"]
    #[inline(always)]
    pub fn dfllunf(&mut self) -> DfllunfW<'_, IntflagSpec> {
        DfllunfW::new(self, 11)
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfllrcs(&mut self) -> DfllrcsW<'_, IntflagSpec> {
        DfllrcsW::new(self, 12)
    }
    #[doc = "Bit 13 - DFLL Startup Failure"]
    #[inline(always)]
    pub fn dfllfail(&mut self) -> DfllfailW<'_, IntflagSpec> {
        DfllfailW::new(self, 13)
    }
    #[doc = "Bit 24 - PLL 0 Lock Rise"]
    #[inline(always)]
    pub fn pll0lockr(&mut self) -> Pll0lockrW<'_, IntflagSpec> {
        Pll0lockrW::new(self, 24)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0100_3f07;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
