#[doc = "Register `FIFOPTR` reader"]
pub type R = crate::R<FifoptrSpec>;
#[doc = "Register `FIFOPTR` writer"]
pub type W = crate::W<FifoptrSpec>;
#[doc = "Field `CPUWRPTR` reader - CPU FIFO Write Pointer"]
pub type CpuwrptrR = crate::FieldReader;
#[doc = "Field `CPUWRPTR` writer - CPU FIFO Write Pointer"]
pub type CpuwrptrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CPURDPTR` reader - CPU FIFO Read Pointer"]
pub type CpurdptrR = crate::FieldReader;
#[doc = "Field `CPURDPTR` writer - CPU FIFO Read Pointer"]
pub type CpurdptrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CPU FIFO Write Pointer"]
    #[inline(always)]
    pub fn cpuwrptr(&self) -> CpuwrptrR {
        CpuwrptrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CPU FIFO Read Pointer"]
    #[inline(always)]
    pub fn cpurdptr(&self) -> CpurdptrR {
        CpurdptrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CPU FIFO Write Pointer"]
    #[inline(always)]
    pub fn cpuwrptr(&mut self) -> CpuwrptrW<'_, FifoptrSpec> {
        CpuwrptrW::new(self, 0)
    }
    #[doc = "Bits 8:11 - CPU FIFO Read Pointer"]
    #[inline(always)]
    pub fn cpurdptr(&mut self) -> CpurdptrW<'_, FifoptrSpec> {
        CpurdptrW::new(self, 8)
    }
}
#[doc = "SPIS FIFO CPU Pointers\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoptrSpec;
impl crate::RegisterSpec for FifoptrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fifoptr::R`](R) reader structure"]
impl crate::Readable for FifoptrSpec {}
#[doc = "`write(|w| ..)` method takes [`fifoptr::W`](W) writer structure"]
impl crate::Writable for FifoptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFOPTR to value 0"]
impl crate::Resettable for FifoptrSpec {}
