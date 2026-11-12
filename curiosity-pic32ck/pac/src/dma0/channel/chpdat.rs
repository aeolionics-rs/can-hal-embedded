#[doc = "Register `CHPDAT` reader"]
pub type R = crate::R<ChpdatSpec>;
#[doc = "Register `CHPDAT` writer"]
pub type W = crate::W<ChpdatSpec>;
#[doc = "Field `PDAT` reader - Channel Pattern Match Data"]
pub type PdatR = crate::FieldReader<u16>;
#[doc = "Field `PDAT` writer - Channel Pattern Match Data"]
pub type PdatW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PIGN` reader - Channel Pattern Ignore Value"]
pub type PignR = crate::FieldReader;
#[doc = "Field `PIGN` writer - Channel Pattern Ignore Value"]
pub type PignW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Channel Pattern Match Data"]
    #[inline(always)]
    pub fn pdat(&self) -> PdatR {
        PdatR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - Channel Pattern Ignore Value"]
    #[inline(always)]
    pub fn pign(&self) -> PignR {
        PignR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel Pattern Match Data"]
    #[inline(always)]
    pub fn pdat(&mut self) -> PdatW<'_, ChpdatSpec> {
        PdatW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Channel Pattern Ignore Value"]
    #[inline(always)]
    pub fn pign(&mut self) -> PignW<'_, ChpdatSpec> {
        PignW::new(self, 24)
    }
}
#[doc = "CHANNEL PATTERN MATCH DATA\n\nYou can [`read`](crate::Reg::read) this register and get [`chpdat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chpdat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChpdatSpec;
impl crate::RegisterSpec for ChpdatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chpdat::R`](R) reader structure"]
impl crate::Readable for ChpdatSpec {}
#[doc = "`write(|w| ..)` method takes [`chpdat::W`](W) writer structure"]
impl crate::Writable for ChpdatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHPDAT to value 0"]
impl crate::Resettable for ChpdatSpec {}
