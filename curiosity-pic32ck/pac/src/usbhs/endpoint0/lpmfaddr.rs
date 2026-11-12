#[doc = "Register `LPMFADDR` reader"]
pub type R = crate::R<LpmfaddrSpec>;
#[doc = "Register `LPMFADDR` writer"]
pub type W = crate::W<LpmfaddrSpec>;
#[doc = "Field `FUNCADDR` reader - function address that will be placed in the LPM payload"]
pub type FuncaddrR = crate::FieldReader;
#[doc = "Field `FUNCADDR` writer - function address that will be placed in the LPM payload"]
pub type FuncaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - function address that will be placed in the LPM payload"]
    #[inline(always)]
    pub fn funcaddr(&self) -> FuncaddrR {
        FuncaddrR::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - function address that will be placed in the LPM payload"]
    #[inline(always)]
    pub fn funcaddr(&mut self) -> FuncaddrW<'_, LpmfaddrSpec> {
        FuncaddrW::new(self, 0)
    }
}
#[doc = "Function Address In LPM Payload\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmfaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmfaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmfaddrSpec;
impl crate::RegisterSpec for LpmfaddrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lpmfaddr::R`](R) reader structure"]
impl crate::Readable for LpmfaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`lpmfaddr::W`](W) writer structure"]
impl crate::Writable for LpmfaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPMFADDR to value 0"]
impl crate::Resettable for LpmfaddrSpec {}
