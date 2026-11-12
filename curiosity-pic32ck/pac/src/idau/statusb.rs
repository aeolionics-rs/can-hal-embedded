#[doc = "Register `STATUSB` reader"]
pub type R = crate::R<StatusbSpec>;
#[doc = "Register `STATUSB` writer"]
pub type W = crate::W<StatusbSpec>;
#[doc = "Field `CFGERR` reader - Configuration Error"]
pub type CfgerrR = crate::BitReader;
#[doc = "Field `CFGERR` writer - Configuration Error"]
pub type CfgerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configuration Error"]
    #[inline(always)]
    pub fn cfgerr(&self) -> CfgerrR {
        CfgerrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configuration Error"]
    #[inline(always)]
    pub fn cfgerr(&mut self) -> CfgerrW<'_, StatusbSpec> {
        CfgerrW::new(self, 0)
    }
}
#[doc = "Status B\n\nYou can [`read`](crate::Reg::read) this register and get [`statusb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusbSpec;
impl crate::RegisterSpec for StatusbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusb::R`](R) reader structure"]
impl crate::Readable for StatusbSpec {}
#[doc = "`write(|w| ..)` method takes [`statusb::W`](W) writer structure"]
impl crate::Writable for StatusbSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets STATUSB to value 0"]
impl crate::Resettable for StatusbSpec {}
