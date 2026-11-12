#[doc = "Register `CHSSA` reader"]
pub type R = crate::R<ChssaSpec>;
#[doc = "Register `CHSSA` writer"]
pub type W = crate::W<ChssaSpec>;
#[doc = "Field `SSA` reader - Channel Source Start Address"]
pub type SsaR = crate::FieldReader<u32>;
#[doc = "Field `SSA` writer - Channel Source Start Address"]
pub type SsaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Source Start Address"]
    #[inline(always)]
    pub fn ssa(&self) -> SsaR {
        SsaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Source Start Address"]
    #[inline(always)]
    pub fn ssa(&mut self) -> SsaW<'_, ChssaSpec> {
        SsaW::new(self, 0)
    }
}
#[doc = "CHANNEL SOURCE START ADDRESS\n\nYou can [`read`](crate::Reg::read) this register and get [`chssa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chssa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChssaSpec;
impl crate::RegisterSpec for ChssaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chssa::R`](R) reader structure"]
impl crate::Readable for ChssaSpec {}
#[doc = "`write(|w| ..)` method takes [`chssa::W`](W) writer structure"]
impl crate::Writable for ChssaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSSA to value 0"]
impl crate::Resettable for ChssaSpec {}
