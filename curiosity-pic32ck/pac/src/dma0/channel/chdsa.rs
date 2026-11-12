#[doc = "Register `CHDSA` reader"]
pub type R = crate::R<ChdsaSpec>;
#[doc = "Register `CHDSA` writer"]
pub type W = crate::W<ChdsaSpec>;
#[doc = "Field `DSA` reader - Channel Destination Start Address"]
pub type DsaR = crate::FieldReader<u32>;
#[doc = "Field `DSA` writer - Channel Destination Start Address"]
pub type DsaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Destination Start Address"]
    #[inline(always)]
    pub fn dsa(&self) -> DsaR {
        DsaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Destination Start Address"]
    #[inline(always)]
    pub fn dsa(&mut self) -> DsaW<'_, ChdsaSpec> {
        DsaW::new(self, 0)
    }
}
#[doc = "CHANNEL DESTINATION START ADDRESS\n\nYou can [`read`](crate::Reg::read) this register and get [`chdsa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdsa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChdsaSpec;
impl crate::RegisterSpec for ChdsaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chdsa::R`](R) reader structure"]
impl crate::Readable for ChdsaSpec {}
#[doc = "`write(|w| ..)` method takes [`chdsa::W`](W) writer structure"]
impl crate::Writable for ChdsaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHDSA to value 0"]
impl crate::Resettable for ChdsaSpec {}
