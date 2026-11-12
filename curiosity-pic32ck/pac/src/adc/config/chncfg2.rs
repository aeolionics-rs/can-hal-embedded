#[doc = "Register `CHNCFG2` reader"]
pub type R = crate::R<Chncfg2Spec>;
#[doc = "Register `CHNCFG2` writer"]
pub type W = crate::W<Chncfg2Spec>;
#[doc = "Field `CSS` reader - Channel SCAN Select"]
pub type CssR = crate::FieldReader<u16>;
#[doc = "Field `CSS` writer - Channel SCAN Select"]
pub type CssW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `FRACT` reader - Channel Fractional"]
pub type FractR = crate::FieldReader<u16>;
#[doc = "Field `FRACT` writer - Channel Fractional"]
pub type FractW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Channel SCAN Select"]
    #[inline(always)]
    pub fn css(&self) -> CssR {
        CssR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Channel Fractional"]
    #[inline(always)]
    pub fn fract(&self) -> FractR {
        FractR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Channel SCAN Select"]
    #[inline(always)]
    pub fn css(&mut self) -> CssW<'_, Chncfg2Spec> {
        CssW::new(self, 0)
    }
    #[doc = "Bits 16:29 - Channel Fractional"]
    #[inline(always)]
    pub fn fract(&mut self) -> FractW<'_, Chncfg2Spec> {
        FractW::new(self, 16)
    }
}
#[doc = "Channel Configuration 2(FRACT/CSS)\n\nYou can [`read`](crate::Reg::read) this register and get [`chncfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chncfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chncfg2Spec;
impl crate::RegisterSpec for Chncfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chncfg2::R`](R) reader structure"]
impl crate::Readable for Chncfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`chncfg2::W`](W) writer structure"]
impl crate::Writable for Chncfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNCFG2 to value 0"]
impl crate::Resettable for Chncfg2Spec {}
