#[doc = "Register `INTFLAGC` reader"]
pub type R = crate::R<IntflagcSpec>;
#[doc = "Register `INTFLAGC` writer"]
pub type W = crate::W<IntflagcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Peripheral interrupt flag status - Bridge C\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagcSpec;
impl crate::RegisterSpec for IntflagcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagc::R`](R) reader structure"]
impl crate::Readable for IntflagcSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagc::W`](W) writer structure"]
impl crate::Writable for IntflagcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTFLAGC to value 0"]
impl crate::Resettable for IntflagcSpec {}
