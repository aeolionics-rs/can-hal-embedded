#[doc = "Register `CHNCFG1` reader"]
pub type R = crate::R<Chncfg1Spec>;
#[doc = "Register `CHNCFG1` writer"]
pub type W = crate::W<Chncfg1Spec>;
#[doc = "Field `CHNCMPEN` reader - Channel Comparator Enable"]
pub type ChncmpenR = crate::FieldReader<u16>;
#[doc = "Field `CHNCMPEN` writer - Channel Comparator Enable"]
pub type ChncmpenW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LVL` reader - Channel Level"]
pub type LvlR = crate::FieldReader<u16>;
#[doc = "Field `LVL` writer - Channel Level"]
pub type LvlW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Channel Comparator Enable"]
    #[inline(always)]
    pub fn chncmpen(&self) -> ChncmpenR {
        ChncmpenR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Channel Level"]
    #[inline(always)]
    pub fn lvl(&self) -> LvlR {
        LvlR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Channel Comparator Enable"]
    #[inline(always)]
    pub fn chncmpen(&mut self) -> ChncmpenW<'_, Chncfg1Spec> {
        ChncmpenW::new(self, 0)
    }
    #[doc = "Bits 16:29 - Channel Level"]
    #[inline(always)]
    pub fn lvl(&mut self) -> LvlW<'_, Chncfg1Spec> {
        LvlW::new(self, 16)
    }
}
#[doc = "Channel Configuration 1 (LVL/CMPEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`chncfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chncfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chncfg1Spec;
impl crate::RegisterSpec for Chncfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chncfg1::R`](R) reader structure"]
impl crate::Readable for Chncfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`chncfg1::W`](W) writer structure"]
impl crate::Writable for Chncfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNCFG1 to value 0"]
impl crate::Resettable for Chncfg1Spec {}
