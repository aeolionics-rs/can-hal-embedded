#[doc = "Register `CTUCH` reader"]
pub type R = crate::R<CtuchSpec>;
#[doc = "Register `CTUCH` writer"]
pub type W = crate::W<CtuchSpec>;
#[doc = "Field `CTUCH` reader - this number when multiplied by 4 represents the number of xclk cycles before the timeout occurs"]
pub type CtuchR = crate::FieldReader<u16>;
#[doc = "Field `CTUCH` writer - this number when multiplied by 4 represents the number of xclk cycles before the timeout occurs"]
pub type CtuchW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - this number when multiplied by 4 represents the number of xclk cycles before the timeout occurs"]
    #[inline(always)]
    pub fn ctuch(&self) -> CtuchR {
        CtuchR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - this number when multiplied by 4 represents the number of xclk cycles before the timeout occurs"]
    #[inline(always)]
    pub fn ctuch(&mut self) -> CtuchW<'_, CtuchSpec> {
        CtuchW::new(self, 0)
    }
}
#[doc = "Chirp Timeout. Number Of XCLK Cycles Before The Timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`ctuch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctuch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtuchSpec;
impl crate::RegisterSpec for CtuchSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctuch::R`](R) reader structure"]
impl crate::Readable for CtuchSpec {}
#[doc = "`write(|w| ..)` method takes [`ctuch::W`](W) writer structure"]
impl crate::Writable for CtuchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTUCH to value 0x4074"]
impl crate::Resettable for CtuchSpec {
    const RESET_VALUE: u16 = 0x4074;
}
