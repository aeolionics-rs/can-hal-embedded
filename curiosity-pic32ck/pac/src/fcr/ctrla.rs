#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `ARB` reader - AHB Arbitration scheme"]
pub type ArbR = crate::BitReader;
#[doc = "Field `ARB` writer - AHB Arbitration scheme"]
pub type ArbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWS` reader - Flash Access Time Defined in terms of AHB Clock Wait States"]
pub type FwsR = crate::FieldReader;
#[doc = "Field `FWS` writer - Flash Access Time Defined in terms of AHB Clock Wait States"]
pub type FwsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADRWS` reader - Address Wait State Enable"]
pub type AdrwsR = crate::BitReader;
#[doc = "Field `ADRWS` writer - Address Wait State Enable"]
pub type AdrwsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOWS` reader - Automatic Wait State Enable."]
pub type AutowsR = crate::BitReader;
#[doc = "Field `AUTOWS` writer - Automatic Wait State Enable."]
pub type AutowsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDBUFWS` reader - Data returned from the Read Buffer match the Flash Wait States"]
pub type RdbufwsR = crate::FieldReader;
#[doc = "Field `RDBUFWS` writer - Data returned from the Read Buffer match the Flash Wait States"]
pub type RdbufwsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 2 - AHB Arbitration scheme"]
    #[inline(always)]
    pub fn arb(&self) -> ArbR {
        ArbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Flash Access Time Defined in terms of AHB Clock Wait States"]
    #[inline(always)]
    pub fn fws(&self) -> FwsR {
        FwsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Address Wait State Enable"]
    #[inline(always)]
    pub fn adrws(&self) -> AdrwsR {
        AdrwsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Automatic Wait State Enable."]
    #[inline(always)]
    pub fn autows(&self) -> AutowsR {
        AutowsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Data returned from the Read Buffer match the Flash Wait States"]
    #[inline(always)]
    pub fn rdbufws(&self) -> RdbufwsR {
        RdbufwsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - AHB Arbitration scheme"]
    #[inline(always)]
    pub fn arb(&mut self) -> ArbW<'_, CtrlaSpec> {
        ArbW::new(self, 2)
    }
    #[doc = "Bits 8:11 - Flash Access Time Defined in terms of AHB Clock Wait States"]
    #[inline(always)]
    pub fn fws(&mut self) -> FwsW<'_, CtrlaSpec> {
        FwsW::new(self, 8)
    }
    #[doc = "Bit 14 - Address Wait State Enable"]
    #[inline(always)]
    pub fn adrws(&mut self) -> AdrwsW<'_, CtrlaSpec> {
        AdrwsW::new(self, 14)
    }
    #[doc = "Bit 15 - Automatic Wait State Enable."]
    #[inline(always)]
    pub fn autows(&mut self) -> AutowsW<'_, CtrlaSpec> {
        AutowsW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Data returned from the Read Buffer match the Flash Wait States"]
    #[inline(always)]
    pub fn rdbufws(&mut self) -> RdbufwsW<'_, CtrlaSpec> {
        RdbufwsW::new(self, 16)
    }
}
#[doc = "Control A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLA to value 0x8000"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u32 = 0x8000;
}
