#[doc = "Register `CTHHSRTN` reader"]
pub type R = crate::R<CthhsrtnSpec>;
#[doc = "Register `CTHHSRTN` writer"]
pub type W = crate::W<CthhsrtnSpec>;
#[doc = "Field `CTHHSRTN` reader - the delay from the end of high speed resume signaling to enabling utm normal operating mode."]
pub type CthhsrtnR = crate::FieldReader<u16>;
#[doc = "Field `CTHHSRTN` writer - the delay from the end of high speed resume signaling to enabling utm normal operating mode."]
pub type CthhsrtnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the delay from the end of high speed resume signaling to enabling utm normal operating mode."]
    #[inline(always)]
    pub fn cthhsrtn(&self) -> CthhsrtnR {
        CthhsrtnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - the delay from the end of high speed resume signaling to enabling utm normal operating mode."]
    #[inline(always)]
    pub fn cthhsrtn(&mut self) -> CthhsrtnW<'_, CthhsrtnSpec> {
        CthhsrtnW::new(self, 0)
    }
}
#[doc = "Sets The Delay From The End Of High-Speed Resume Signaling (Acting As A Host) To Enable The UTM Normal Operating Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`cthhsrtn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cthhsrtn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CthhsrtnSpec;
impl crate::RegisterSpec for CthhsrtnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cthhsrtn::R`](R) reader structure"]
impl crate::Readable for CthhsrtnSpec {}
#[doc = "`write(|w| ..)` method takes [`cthhsrtn::W`](W) writer structure"]
impl crate::Writable for CthhsrtnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTHHSRTN to value 0x32"]
impl crate::Resettable for CthhsrtnSpec {
    const RESET_VALUE: u16 = 0x32;
}
