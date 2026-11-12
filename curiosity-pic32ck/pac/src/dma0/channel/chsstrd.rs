#[doc = "Register `CHSSTRD` reader"]
pub type R = crate::R<ChsstrdSpec>;
#[doc = "Register `CHSSTRD` writer"]
pub type W = crate::W<ChsstrdSpec>;
#[doc = "Field `SSTRD` reader - Source Cell Stride Size"]
pub type SstrdR = crate::FieldReader<u16>;
#[doc = "Field `SSTRD` writer - Source Cell Stride Size"]
pub type SstrdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Source Cell Stride Size"]
    #[inline(always)]
    pub fn sstrd(&self) -> SstrdR {
        SstrdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Source Cell Stride Size"]
    #[inline(always)]
    pub fn sstrd(&mut self) -> SstrdW<'_, ChsstrdSpec> {
        SstrdW::new(self, 0)
    }
}
#[doc = "CHANNEL SOURCE CELL STRIDE SIZE REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chsstrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsstrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChsstrdSpec;
impl crate::RegisterSpec for ChsstrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsstrd::R`](R) reader structure"]
impl crate::Readable for ChsstrdSpec {}
#[doc = "`write(|w| ..)` method takes [`chsstrd::W`](W) writer structure"]
impl crate::Writable for ChsstrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSSTRD to value 0"]
impl crate::Resettable for ChsstrdSpec {}
