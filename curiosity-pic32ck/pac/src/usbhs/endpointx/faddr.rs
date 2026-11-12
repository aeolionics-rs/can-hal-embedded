#[doc = "Register `FADDR` reader"]
pub type R = crate::R<FaddrSpec>;
#[doc = "Register `FADDR` writer"]
pub type W = crate::W<FaddrSpec>;
#[doc = "Field `FUNCADDR` reader - Function address"]
pub type FuncaddrR = crate::FieldReader;
#[doc = "Field `FUNCADDR` writer - Function address"]
pub type FuncaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Function address"]
    #[inline(always)]
    pub fn funcaddr(&self) -> FuncaddrR {
        FuncaddrR::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Function address"]
    #[inline(always)]
    pub fn funcaddr(&mut self) -> FuncaddrW<'_, FaddrSpec> {
        FuncaddrW::new(self, 0)
    }
}
#[doc = "Function Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`faddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaddrSpec;
impl crate::RegisterSpec for FaddrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`faddr::R`](R) reader structure"]
impl crate::Readable for FaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`faddr::W`](W) writer structure"]
impl crate::Writable for FaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FADDR to value 0"]
impl crate::Resettable for FaddrSpec {}
