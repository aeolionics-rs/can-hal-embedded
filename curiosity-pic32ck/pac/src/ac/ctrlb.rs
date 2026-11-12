#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `START0` reader - Comparator 0 Start Comparison"]
pub type Start0R = crate::BitReader;
#[doc = "Field `START0` writer - Comparator 0 Start Comparison"]
pub type Start0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START1` reader - Comparator 1 Start Comparison"]
pub type Start1R = crate::BitReader;
#[doc = "Field `START1` writer - Comparator 1 Start Comparison"]
pub type Start1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Start Comparison"]
    #[inline(always)]
    pub fn start0(&self) -> Start0R {
        Start0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Start Comparison"]
    #[inline(always)]
    pub fn start1(&self) -> Start1R {
        Start1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Start Comparison"]
    #[inline(always)]
    pub fn start0(&mut self) -> Start0W<'_, CtrlbSpec> {
        Start0W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 1 Start Comparison"]
    #[inline(always)]
    pub fn start1(&mut self) -> Start1W<'_, CtrlbSpec> {
        Start1W::new(self, 1)
    }
}
#[doc = "Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}
