#[doc = "Register `RXFUNCADDREP` reader"]
pub type R = crate::R<RxfuncaddrepSpec>;
#[doc = "Register `RXFUNCADDREP` writer"]
pub type W = crate::W<RxfuncaddrepSpec>;
#[doc = "Field `RXFUNCADDR` reader - address of the target function"]
pub type RxfuncaddrR = crate::FieldReader;
#[doc = "Field `RXFUNCADDR` writer - address of the target function"]
pub type RxfuncaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - address of the target function"]
    #[inline(always)]
    pub fn rxfuncaddr(&self) -> RxfuncaddrR {
        RxfuncaddrR::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - address of the target function"]
    #[inline(always)]
    pub fn rxfuncaddr(&mut self) -> RxfuncaddrW<'_, RxfuncaddrepSpec> {
        RxfuncaddrW::new(self, 0)
    }
}
#[doc = "Relevant In Host Mode Only. Address Of The Target Function.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfuncaddrep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfuncaddrep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxfuncaddrepSpec;
impl crate::RegisterSpec for RxfuncaddrepSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxfuncaddrep::R`](R) reader structure"]
impl crate::Readable for RxfuncaddrepSpec {}
#[doc = "`write(|w| ..)` method takes [`rxfuncaddrep::W`](W) writer structure"]
impl crate::Writable for RxfuncaddrepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXFUNCADDREP to value 0"]
impl crate::Resettable for RxfuncaddrepSpec {}
