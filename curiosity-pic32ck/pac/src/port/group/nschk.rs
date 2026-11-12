#[doc = "Register `NSCHK` reader"]
pub type R = crate::R<NschkSpec>;
#[doc = "Register `NSCHK` writer"]
pub type W = crate::W<NschkSpec>;
#[doc = "Field `NSCHK` reader - Port Security Attribution Check"]
pub type NschkR = crate::FieldReader<u32>;
#[doc = "Field `NSCHK` writer - Port Security Attribution Check"]
pub type NschkW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Security Attribution Check"]
    #[inline(always)]
    pub fn nschk(&self) -> NschkR {
        NschkR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Security Attribution Check"]
    #[inline(always)]
    pub fn nschk(&mut self) -> NschkW<'_, NschkSpec> {
        NschkW::new(self, 0)
    }
}
#[doc = "Security Attribution Check\n\nYou can [`read`](crate::Reg::read) this register and get [`nschk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nschk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NschkSpec;
impl crate::RegisterSpec for NschkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nschk::R`](R) reader structure"]
impl crate::Readable for NschkSpec {}
#[doc = "`write(|w| ..)` method takes [`nschk::W`](W) writer structure"]
impl crate::Writable for NschkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NSCHK to value 0"]
impl crate::Resettable for NschkSpec {}
