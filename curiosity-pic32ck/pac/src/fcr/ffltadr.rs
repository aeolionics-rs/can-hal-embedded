#[doc = "Register `FFLTADR` reader"]
pub type R = crate::R<FfltadrSpec>;
#[doc = "Register `FFLTADR` writer"]
pub type W = crate::W<FfltadrSpec>;
#[doc = "Field `FLTADR` reader - Fault Inject Mode - Physical Address"]
pub type FltadrR = crate::FieldReader<u32>;
#[doc = "Field `FLTADR` writer - Fault Inject Mode - Physical Address"]
pub type FltadrW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Fault Inject Mode - Physical Address"]
    #[inline(always)]
    pub fn fltadr(&self) -> FltadrR {
        FltadrR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Fault Inject Mode - Physical Address"]
    #[inline(always)]
    pub fn fltadr(&mut self) -> FltadrW<'_, FfltadrSpec> {
        FltadrW::new(self, 0)
    }
}
#[doc = "Flash Fault Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffltadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfltadrSpec;
impl crate::RegisterSpec for FfltadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffltadr::R`](R) reader structure"]
impl crate::Readable for FfltadrSpec {}
#[doc = "`write(|w| ..)` method takes [`ffltadr::W`](W) writer structure"]
impl crate::Writable for FfltadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FFLTADR to value 0"]
impl crate::Resettable for FfltadrSpec {}
