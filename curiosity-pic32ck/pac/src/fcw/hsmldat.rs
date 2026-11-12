#[doc = "Register `HSMLDAT[%s]` reader"]
pub type R = crate::R<HsmldatSpec>;
#[doc = "Register `HSMLDAT[%s]` writer"]
pub type W = crate::W<HsmldatSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "HSM Tamper Lower Overwrite Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsmldat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsmldat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsmldatSpec;
impl crate::RegisterSpec for HsmldatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsmldat::R`](R) reader structure"]
impl crate::Readable for HsmldatSpec {}
#[doc = "`write(|w| ..)` method takes [`hsmldat::W`](W) writer structure"]
impl crate::Writable for HsmldatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSMLDAT[%s] to value 0xffff_ffff"]
impl crate::Resettable for HsmldatSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
