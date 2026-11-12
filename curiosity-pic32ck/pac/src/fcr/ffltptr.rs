#[doc = "Register `FFLTPTR` reader"]
pub type R = crate::R<FfltptrSpec>;
#[doc = "Register `FFLTPTR` writer"]
pub type W = crate::W<FfltptrSpec>;
#[doc = "Field `FLT1PTR` reader - Fault 1 Injection Pointer"]
pub type Flt1ptrR = crate::FieldReader<u16>;
#[doc = "Field `FLT1PTR` writer - Fault 1 Injection Pointer"]
pub type Flt1ptrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `FLT2PTR` reader - Fault 2 Injection Pointer"]
pub type Flt2ptrR = crate::FieldReader<u16>;
#[doc = "Field `FLT2PTR` writer - Fault 2 Injection Pointer"]
pub type Flt2ptrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Fault 1 Injection Pointer"]
    #[inline(always)]
    pub fn flt1ptr(&self) -> Flt1ptrR {
        Flt1ptrR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Fault 2 Injection Pointer"]
    #[inline(always)]
    pub fn flt2ptr(&self) -> Flt2ptrR {
        Flt2ptrR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Fault 1 Injection Pointer"]
    #[inline(always)]
    pub fn flt1ptr(&mut self) -> Flt1ptrW<'_, FfltptrSpec> {
        Flt1ptrW::new(self, 0)
    }
    #[doc = "Bits 16:24 - Fault 2 Injection Pointer"]
    #[inline(always)]
    pub fn flt2ptr(&mut self) -> Flt2ptrW<'_, FfltptrSpec> {
        Flt2ptrW::new(self, 16)
    }
}
#[doc = "Flash ECC Fault Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffltptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffltptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfltptrSpec;
impl crate::RegisterSpec for FfltptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffltptr::R`](R) reader structure"]
impl crate::Readable for FfltptrSpec {}
#[doc = "`write(|w| ..)` method takes [`ffltptr::W`](W) writer structure"]
impl crate::Writable for FfltptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FFLTPTR to value 0"]
impl crate::Resettable for FfltptrSpec {}
