#[doc = "Register `NONSEC` reader"]
pub type R = crate::R<NonsecSpec>;
#[doc = "Register `NONSEC` writer"]
pub type W = crate::W<NonsecSpec>;
#[doc = "Field `NONSEC` reader - Port Security Attribution"]
pub type NonsecR = crate::FieldReader<u32>;
#[doc = "Field `NONSEC` writer - Port Security Attribution"]
pub type NonsecW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Security Attribution"]
    #[inline(always)]
    pub fn nonsec(&self) -> NonsecR {
        NonsecR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Security Attribution"]
    #[inline(always)]
    pub fn nonsec(&mut self) -> NonsecW<'_, NonsecSpec> {
        NonsecW::new(self, 0)
    }
}
#[doc = "Security Attribution\n\nYou can [`read`](crate::Reg::read) this register and get [`nonsec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonsec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NonsecSpec;
impl crate::RegisterSpec for NonsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nonsec::R`](R) reader structure"]
impl crate::Readable for NonsecSpec {}
#[doc = "`write(|w| ..)` method takes [`nonsec::W`](W) writer structure"]
impl crate::Writable for NonsecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NONSEC to value 0"]
impl crate::Resettable for NonsecSpec {}
