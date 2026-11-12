#[doc = "Register `DMAADDR` reader"]
pub type R = crate::R<DmaaddrSpec>;
#[doc = "Register `DMAADDR` writer"]
pub type W = crate::W<DmaaddrSpec>;
#[doc = "Field `ADDR10` reader - Lower 2 bits of DMA memory address"]
pub type Addr10R = crate::FieldReader;
#[doc = "Field `ADDR10` writer - Lower 2 bits of DMA memory address"]
pub type Addr10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADDR312` reader - upper 30 bits of address"]
pub type Addr312R = crate::FieldReader<u32>;
#[doc = "Field `ADDR312` writer - upper 30 bits of address"]
pub type Addr312W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - Lower 2 bits of DMA memory address"]
    #[inline(always)]
    pub fn addr10(&self) -> Addr10R {
        Addr10R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - upper 30 bits of address"]
    #[inline(always)]
    pub fn addr312(&self) -> Addr312R {
        Addr312R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - Lower 2 bits of DMA memory address"]
    #[inline(always)]
    pub fn addr10(&mut self) -> Addr10W<'_, DmaaddrSpec> {
        Addr10W::new(self, 0)
    }
    #[doc = "Bits 2:31 - upper 30 bits of address"]
    #[inline(always)]
    pub fn addr312(&mut self) -> Addr312W<'_, DmaaddrSpec> {
        Addr312W::new(self, 2)
    }
}
#[doc = "Memory Address Of The Corresponding DMA Channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaaddrSpec;
impl crate::RegisterSpec for DmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaaddr::R`](R) reader structure"]
impl crate::Readable for DmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaaddr::W`](W) writer structure"]
impl crate::Writable for DmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAADDR to value 0"]
impl crate::Resettable for DmaaddrSpec {}
