#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Blank the wakeup monitoring for a specific real time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Blankselect {
    #[doc = "0: No blank delay"]
    Blank0 = 0,
    #[doc = "1048575: Maximum blank delay"]
    Blankmax = 1048575,
}
impl From<Blankselect> for u32 {
    #[inline(always)]
    fn from(variant: Blankselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Blankselect {
    type Ux = u32;
}
impl crate::IsEnum for Blankselect {}
#[doc = "Field `BLANK` reader - Blank the wakeup monitoring for a specific real time"]
pub type BlankR = crate::FieldReader<Blankselect>;
impl BlankR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Blankselect> {
        match self.bits {
            0 => Some(Blankselect::Blank0),
            1048575 => Some(Blankselect::Blankmax),
            _ => None,
        }
    }
    #[doc = "No blank delay"]
    #[inline(always)]
    pub fn is_blank0(&self) -> bool {
        *self == Blankselect::Blank0
    }
    #[doc = "Maximum blank delay"]
    #[inline(always)]
    pub fn is_blankmax(&self) -> bool {
        *self == Blankselect::Blankmax
    }
}
#[doc = "Field `BLANK` writer - Blank the wakeup monitoring for a specific real time"]
pub type BlankW<'a, REG> = crate::FieldWriter<'a, REG, 20, Blankselect>;
impl<'a, REG> BlankW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No blank delay"]
    #[inline(always)]
    pub fn blank0(self) -> &'a mut crate::W<REG> {
        self.variant(Blankselect::Blank0)
    }
    #[doc = "Maximum blank delay"]
    #[inline(always)]
    pub fn blankmax(self) -> &'a mut crate::W<REG> {
        self.variant(Blankselect::Blankmax)
    }
}
impl R {
    #[doc = "Bits 0:19 - Blank the wakeup monitoring for a specific real time"]
    #[inline(always)]
    pub fn blank(&self) -> BlankR {
        BlankR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Blank the wakeup monitoring for a specific real time"]
    #[inline(always)]
    pub fn blank(&mut self) -> BlankW<'_, CtrlbSpec> {
        BlankW::new(self, 0)
    }
}
#[doc = "USBHS Control Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}
