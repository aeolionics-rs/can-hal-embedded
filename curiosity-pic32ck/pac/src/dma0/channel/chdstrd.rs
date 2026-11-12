#[doc = "Register `CHDSTRD` reader"]
pub type R = crate::R<ChdstrdSpec>;
#[doc = "Register `CHDSTRD` writer"]
pub type W = crate::W<ChdstrdSpec>;
#[doc = "Field `DSTRD` reader - Destination Cell Stride Size"]
pub type DstrdR = crate::FieldReader<u16>;
#[doc = "Field `DSTRD` writer - Destination Cell Stride Size"]
pub type DstrdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Destination Cell Stride Size"]
    #[inline(always)]
    pub fn dstrd(&self) -> DstrdR {
        DstrdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Destination Cell Stride Size"]
    #[inline(always)]
    pub fn dstrd(&mut self) -> DstrdW<'_, ChdstrdSpec> {
        DstrdW::new(self, 0)
    }
}
#[doc = "CHANNEL DESTINATION CELL STRIDE SIZE REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chdstrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdstrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChdstrdSpec;
impl crate::RegisterSpec for ChdstrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chdstrd::R`](R) reader structure"]
impl crate::Readable for ChdstrdSpec {}
#[doc = "`write(|w| ..)` method takes [`chdstrd::W`](W) writer structure"]
impl crate::Writable for ChdstrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHDSTRD to value 0"]
impl crate::Resettable for ChdstrdSpec {}
