#[doc = "Register `HSMUDAT[%s]` reader"]
pub type R = crate::R<HsmudatSpec>;
#[doc = "Register `HSMUDAT[%s]` writer"]
pub type W = crate::W<HsmudatSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "HSM Tamper Upper Overwrite Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsmudat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsmudat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsmudatSpec;
impl crate::RegisterSpec for HsmudatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsmudat::R`](R) reader structure"]
impl crate::Readable for HsmudatSpec {}
#[doc = "`write(|w| ..)` method takes [`hsmudat::W`](W) writer structure"]
impl crate::Writable for HsmudatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSMUDAT[%s] to value 0xffff_ffff"]
impl crate::Resettable for HsmudatSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
