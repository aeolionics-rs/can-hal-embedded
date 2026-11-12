#[doc = "Register `INTFLAGAHB` reader"]
pub type R = crate::R<IntflagahbSpec>;
#[doc = "Register `INTFLAGAHB` writer"]
pub type W = crate::W<IntflagahbSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Bridge interrupt flag status\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagahb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagahb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagahbSpec;
impl crate::RegisterSpec for IntflagahbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagahb::R`](R) reader structure"]
impl crate::Readable for IntflagahbSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagahb::W`](W) writer structure"]
impl crate::Writable for IntflagahbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTFLAGAHB to value 0"]
impl crate::Resettable for IntflagahbSpec {}
