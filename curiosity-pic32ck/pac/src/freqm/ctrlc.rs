#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Window Monitor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Winmodeselect {
    #[doc = "0: No window mode (default)"]
    Disable = 0,
    #[doc = "1: VALUE > WINLT"]
    Mode1 = 1,
    #[doc = "2: VALUE < WINUT"]
    Mode2 = 2,
    #[doc = "3: WINLT < VALUE < WINUT"]
    Mode3 = 3,
    #[doc = "4: !(WINLT < VALUE < WINUT)"]
    Mode4 = 4,
}
impl From<Winmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Winmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Winmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Winmodeselect {}
#[doc = "Field `WINMODE` reader - Window Monitor Mode"]
pub type WinmodeR = crate::FieldReader<Winmodeselect>;
impl WinmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Winmodeselect> {
        match self.bits {
            0 => Some(Winmodeselect::Disable),
            1 => Some(Winmodeselect::Mode1),
            2 => Some(Winmodeselect::Mode2),
            3 => Some(Winmodeselect::Mode3),
            4 => Some(Winmodeselect::Mode4),
            _ => None,
        }
    }
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Winmodeselect::Disable
    }
    #[doc = "VALUE > WINLT"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Winmodeselect::Mode1
    }
    #[doc = "VALUE < WINUT"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == Winmodeselect::Mode2
    }
    #[doc = "WINLT < VALUE < WINUT"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == Winmodeselect::Mode3
    }
    #[doc = "!(WINLT < VALUE < WINUT)"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == Winmodeselect::Mode4
    }
}
#[doc = "Field `WINMODE` writer - Window Monitor Mode"]
pub type WinmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Winmodeselect>;
impl<'a, REG> WinmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Disable)
    }
    #[doc = "VALUE > WINLT"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode1)
    }
    #[doc = "VALUE < WINUT"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode2)
    }
    #[doc = "WINLT < VALUE < WINUT"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode3)
    }
    #[doc = "!(WINLT < VALUE < WINUT)"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode4)
    }
}
impl R {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WinmodeR {
        WinmodeR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&mut self) -> WinmodeW<'_, CtrlcSpec> {
        WinmodeW::new(self, 0)
    }
}
#[doc = "Control C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlcSpec;
impl crate::RegisterSpec for CtrlcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CtrlcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CtrlcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CtrlcSpec {}
