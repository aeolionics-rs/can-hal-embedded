#[doc = "Register `CHCRCDAT` reader"]
pub type R = crate::R<ChcrcdatSpec>;
#[doc = "Register `CHCRCDAT` writer"]
pub type W = crate::W<ChcrcdatSpec>;
#[doc = "Field `CRCDAT` reader - CRC Data"]
pub type CrcdatR = crate::FieldReader<u32>;
#[doc = "Field `CRCDAT` writer - CRC Data"]
pub type CrcdatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Data"]
    #[inline(always)]
    pub fn crcdat(&self) -> CrcdatR {
        CrcdatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Data"]
    #[inline(always)]
    pub fn crcdat(&mut self) -> CrcdatW<'_, ChcrcdatSpec> {
        CrcdatW::new(self, 0)
    }
}
#[doc = "CHANNEL CRC/CHECKSUM DATA REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chcrcdat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chcrcdat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChcrcdatSpec;
impl crate::RegisterSpec for ChcrcdatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chcrcdat::R`](R) reader structure"]
impl crate::Readable for ChcrcdatSpec {}
#[doc = "`write(|w| ..)` method takes [`chcrcdat::W`](W) writer structure"]
impl crate::Writable for ChcrcdatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCRCDAT to value 0"]
impl crate::Resettable for ChcrcdatSpec {}
