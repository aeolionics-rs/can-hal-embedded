#[doc = "Register `FIFOX[%s]` reader"]
pub type R = crate::R<FifoxSpec>;
#[doc = "Register `FIFOX[%s]` writer"]
pub type W = crate::W<FifoxSpec>;
#[doc = "Field `FIFOADDR` reader - Writing To These Addresses Loads Data Into The TXFIFO For The Corresponding Endpoint"]
pub type FifoaddrR = crate::FieldReader<u32>;
#[doc = "Field `FIFOADDR` writer - Writing To These Addresses Loads Data Into The TXFIFO For The Corresponding Endpoint"]
pub type FifoaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Writing To These Addresses Loads Data Into The TXFIFO For The Corresponding Endpoint"]
    #[inline(always)]
    pub fn fifoaddr(&self) -> FifoaddrR {
        FifoaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing To These Addresses Loads Data Into The TXFIFO For The Corresponding Endpoint"]
    #[inline(always)]
    pub fn fifoaddr(&mut self) -> FifoaddrW<'_, FifoxSpec> {
        FifoaddrW::new(self, 0)
    }
}
#[doc = "Address Range Provides 16 Addresses For CPU Access To The FIFO's For Each Endpoint\n\nYou can [`read`](crate::Reg::read) this register and get [`fifox::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifox::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoxSpec;
impl crate::RegisterSpec for FifoxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifox::R`](R) reader structure"]
impl crate::Readable for FifoxSpec {}
#[doc = "`write(|w| ..)` method takes [`fifox::W`](W) writer structure"]
impl crate::Writable for FifoxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFOX[%s] to value 0"]
impl crate::Resettable for FifoxSpec {}
