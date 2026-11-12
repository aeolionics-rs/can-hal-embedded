#[doc = "Register `TIDM[%s]` reader"]
pub type R = crate::R<TidmSpec>;
#[doc = "Register `TIDM[%s]` writer"]
pub type W = crate::W<TidmSpec>;
#[doc = "Field `TID` reader - Type ID Match 1"]
pub type TidR = crate::FieldReader<u16>;
#[doc = "Field `TID` writer - Type ID Match 1"]
pub type TidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Type ID Match 1"]
    #[inline(always)]
    pub fn tid(&self) -> TidR {
        TidR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID Match 1"]
    #[inline(always)]
    pub fn tid(&mut self) -> TidW<'_, TidmSpec> {
        TidW::new(self, 0)
    }
}
#[doc = "Type ID Match Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tidm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tidm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TidmSpec;
impl crate::RegisterSpec for TidmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tidm::R`](R) reader structure"]
impl crate::Readable for TidmSpec {}
#[doc = "`write(|w| ..)` method takes [`tidm::W`](W) writer structure"]
impl crate::Writable for TidmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIDM[%s] to value 0"]
impl crate::Resettable for TidmSpec {}
