#[doc = "Register `MBFI1` reader"]
pub type R = crate::R<Mbfi1Spec>;
#[doc = "Register `MBFI1` writer"]
pub type W = crate::W<Mbfi1Spec>;
#[doc = "Field `ADDR` reader - Word Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Word Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Word Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Word Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, Mbfi1Spec> {
        AddrW::new(self, 2)
    }
}
#[doc = "Memory Bist Fault Injection 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mbfi1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbfi1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mbfi1Spec;
impl crate::RegisterSpec for Mbfi1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbfi1::R`](R) reader structure"]
impl crate::Readable for Mbfi1Spec {}
#[doc = "`write(|w| ..)` method takes [`mbfi1::W`](W) writer structure"]
impl crate::Writable for Mbfi1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBFI1 to value 0"]
impl crate::Resettable for Mbfi1Spec {}
