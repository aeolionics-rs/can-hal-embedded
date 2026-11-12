#[doc = "Register `DBGCTRL` reader"]
pub type R = crate::R<DbgctrlSpec>;
#[doc = "Register `DBGCTRL` writer"]
pub type W = crate::W<DbgctrlSpec>;
#[doc = "Field `LCKUPDIS` reader - Lockup Disable"]
pub type LckupdisR = crate::BitReader;
#[doc = "Field `LCKUPDIS` writer - Lockup Disable"]
pub type LckupdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lockup Disable"]
    #[inline(always)]
    pub fn lckupdis(&self) -> LckupdisR {
        LckupdisR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lockup Disable"]
    #[inline(always)]
    pub fn lckupdis(&mut self) -> LckupdisW<'_, DbgctrlSpec> {
        LckupdisW::new(self, 0)
    }
}
#[doc = "Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgctrlSpec;
impl crate::RegisterSpec for DbgctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgctrl::R`](R) reader structure"]
impl crate::Readable for DbgctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgctrl::W`](W) writer structure"]
impl crate::Writable for DbgctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGCTRL to value 0"]
impl crate::Resettable for DbgctrlSpec {}
