#[doc = "Register `INTFLAGD` reader"]
pub type R = crate::R<IntflagdSpec>;
#[doc = "Register `INTFLAGD` writer"]
pub type W = crate::W<IntflagdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Peripheral interrupt flag status - Bridge D\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagdSpec;
impl crate::RegisterSpec for IntflagdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagd::R`](R) reader structure"]
impl crate::Readable for IntflagdSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagd::W`](W) writer structure"]
impl crate::Writable for IntflagdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTFLAGD to value 0"]
impl crate::Resettable for IntflagdSpec {}
