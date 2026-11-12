#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - SOFTWARE RESET BIT"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - SOFTWARE RESET BIT"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - ENABLE BIT"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - ENABLE BIT"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRMWS` reader - ROM ACCESS TIME WAIT STATE"]
pub type PrmwsR = crate::FieldReader;
#[doc = "Field `PRMWS` writer - ROM ACCESS TIME WAIT STATE"]
pub type PrmwsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - SOFTWARE RESET BIT"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ENABLE BIT"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:10 - ROM ACCESS TIME WAIT STATE"]
    #[inline(always)]
    pub fn prmws(&self) -> PrmwsR {
        PrmwsR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SOFTWARE RESET BIT"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - ENABLE BIT"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 8:10 - ROM ACCESS TIME WAIT STATE"]
    #[inline(always)]
    pub fn prmws(&mut self) -> PrmwsW<'_, CtrlaSpec> {
        PrmwsW::new(self, 8)
    }
}
#[doc = "CONTROL A REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRLA to value 0x02"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u32 = 0x02;
}
