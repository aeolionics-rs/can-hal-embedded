#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Command Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Cmdselect {
    #[doc = "42241: Module Enable"]
    Enable = 42241,
    #[doc = "42242: Module Disable"]
    Disable = 42242,
    #[doc = "42243: Write Lock"]
    Wlck = 42243,
}
impl From<Cmdselect> for u16 {
    #[inline(always)]
    fn from(variant: Cmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdselect {
    type Ux = u16;
}
impl crate::IsEnum for Cmdselect {}
#[doc = "Field `CMD` writer - Command Register"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 16, Cmdselect>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Module Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Enable)
    }
    #[doc = "Module Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Disable)
    }
    #[doc = "Write Lock"]
    #[inline(always)]
    pub fn wlck(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Wlck)
    }
}
impl W {
    #[doc = "Bits 16:31 - Command Register"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, CtrlSpec> {
        CmdW::new(self, 16)
    }
}
#[doc = "Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
