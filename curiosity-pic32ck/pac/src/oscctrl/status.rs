#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `XOSCRDY` reader - XOSC Ready"]
pub type XoscrdyR = crate::BitReader;
#[doc = "Field `XOSCFAIL` reader - XOSC Startup Failure"]
pub type XoscfailR = crate::BitReader;
#[doc = "Field `CLKFAIL` reader - XOSC Clock Failure"]
pub type ClkfailR = crate::BitReader;
#[doc = "Field `XOSCCKSW` reader - XOSC Clock Switch"]
pub type XoscckswR = crate::BitReader;
#[doc = "Field `DFLLRDY` reader - DFLL Ready"]
pub type DfllrdyR = crate::BitReader;
#[doc = "Field `DFLLLOCK` reader - DFLL Lock"]
pub type DflllockR = crate::BitReader;
#[doc = "Field `DFLLOVF` reader - DFLL Tuner Overflow"]
pub type DfllovfR = crate::BitReader;
#[doc = "Field `DFLLUNF` reader - DFLL Tuner Underflow"]
pub type DfllunfR = crate::BitReader;
#[doc = "Field `DFLLRCS` reader - DFLL Reference Clock Stopped"]
pub type DfllrcsR = crate::BitReader;
#[doc = "Field `DFLLFAIL` reader - DFLL Startup Failure"]
pub type DfllfailR = crate::BitReader;
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
    #[doc = "Bit 3 - XOSC Clock Switch"]
    #[inline(always)]
    pub fn xosccksw(&self) -> XoscckswR {
        XoscckswR::new(((self.bits >> 3) & 1) != 0)
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
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
