#[doc = "Register `CHNXT` reader"]
pub type R = crate::R<ChnxtSpec>;
#[doc = "Register `CHNXT` writer"]
pub type W = crate::W<ChnxtSpec>;
#[doc = "Field `NXT` reader - Channel Address Pointer to Next Descriptor"]
pub type NxtR = crate::FieldReader<u32>;
#[doc = "Field `NXT` writer - Channel Address Pointer to Next Descriptor"]
pub type NxtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Address Pointer to Next Descriptor"]
    #[inline(always)]
    pub fn nxt(&self) -> NxtR {
        NxtR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Address Pointer to Next Descriptor"]
    #[inline(always)]
    pub fn nxt(&mut self) -> NxtW<'_, ChnxtSpec> {
        NxtW::new(self, 0)
    }
}
#[doc = "CHANNEL NEXT DESCRIPTOR ADDRESS POINTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chnxt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnxt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnxtSpec;
impl crate::RegisterSpec for ChnxtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnxt::R`](R) reader structure"]
impl crate::Readable for ChnxtSpec {}
#[doc = "`write(|w| ..)` method takes [`chnxt::W`](W) writer structure"]
impl crate::Writable for ChnxtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNXT to value 0"]
impl crate::Resettable for ChnxtSpec {}
