#[doc = "Register `CHRDYDAT` reader"]
pub type R = crate::R<ChrdydatSpec>;
#[doc = "Register `CHRDYDAT` writer"]
pub type W = crate::W<ChrdydatSpec>;
#[doc = "Field `CHRDYDAT` reader - Channel Output Data"]
pub type ChrdydatR = crate::FieldReader<u16>;
#[doc = "Field `CHRDYDAT` writer - Channel Output Data"]
pub type ChrdydatW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LVL` reader - Level Setting"]
pub type LvlR = crate::BitReader;
#[doc = "Field `LVL` writer - Level Setting"]
pub type LvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF` reader - Differential Setting"]
pub type DiffR = crate::BitReader;
#[doc = "Field `DIFF` writer - Differential Setting"]
pub type DiffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIGN` reader - Sign Setting"]
pub type SignR = crate::BitReader;
#[doc = "Field `SIGN` writer - Sign Setting"]
pub type SignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRACT` reader - Fractional Setting"]
pub type FractR = crate::BitReader;
#[doc = "Field `FRACT` writer - Fractional Setting"]
pub type FractW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Channel Output Data"]
    #[inline(always)]
    pub fn chrdydat(&self) -> ChrdydatR {
        ChrdydatR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Level Setting"]
    #[inline(always)]
    pub fn lvl(&self) -> LvlR {
        LvlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Differential Setting"]
    #[inline(always)]
    pub fn diff(&self) -> DiffR {
        DiffR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Sign Setting"]
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Fractional Setting"]
    #[inline(always)]
    pub fn fract(&self) -> FractR {
        FractR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel Output Data"]
    #[inline(always)]
    pub fn chrdydat(&mut self) -> ChrdydatW<'_, ChrdydatSpec> {
        ChrdydatW::new(self, 0)
    }
    #[doc = "Bit 24 - Level Setting"]
    #[inline(always)]
    pub fn lvl(&mut self) -> LvlW<'_, ChrdydatSpec> {
        LvlW::new(self, 24)
    }
    #[doc = "Bit 25 - Differential Setting"]
    #[inline(always)]
    pub fn diff(&mut self) -> DiffW<'_, ChrdydatSpec> {
        DiffW::new(self, 25)
    }
    #[doc = "Bit 26 - Sign Setting"]
    #[inline(always)]
    pub fn sign(&mut self) -> SignW<'_, ChrdydatSpec> {
        SignW::new(self, 26)
    }
    #[doc = "Bit 27 - Fractional Setting"]
    #[inline(always)]
    pub fn fract(&mut self) -> FractW<'_, ChrdydatSpec> {
        FractW::new(self, 27)
    }
}
#[doc = "Channel Ready Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chrdydat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chrdydat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChrdydatSpec;
impl crate::RegisterSpec for ChrdydatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chrdydat::R`](R) reader structure"]
impl crate::Readable for ChrdydatSpec {}
#[doc = "`write(|w| ..)` method takes [`chrdydat::W`](W) writer structure"]
impl crate::Writable for ChrdydatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHRDYDAT to value 0"]
impl crate::Resettable for ChrdydatSpec {}
