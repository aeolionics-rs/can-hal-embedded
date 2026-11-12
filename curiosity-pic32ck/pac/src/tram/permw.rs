#[doc = "Register `PERMW` writer"]
pub type W = crate::W<PermwSpec>;
#[doc = "Field `DATA` writer - Permutation Scrambler Data Input"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl W {
    #[doc = "Bits 0:2 - Permutation Scrambler Data Input"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, PermwSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Permutation Write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`permw::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PermwSpec;
impl crate::RegisterSpec for PermwSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`permw::W`](W) writer structure"]
impl crate::Writable for PermwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERMW to value 0"]
impl crate::Resettable for PermwSpec {}
