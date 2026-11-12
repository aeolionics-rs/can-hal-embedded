#[doc = "Register `RCTRL_WATERMARK_MODE` writer"]
pub type W = crate::W<RctrlWatermarkModeSpec>;
#[doc = "Field `ARG` writer - Command Argument (Watermark)"]
pub type ArgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdselect {
    #[doc = "92: Write IDAU region x Size (RSTATUSB\\[x\\].SIZE=ARG)"]
    Wrsz = 92,
}
impl From<Cmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Cmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Cmdselect {}
#[doc = "Field `CMD` writer - Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 8, Cmdselect>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Write IDAU region x Size (RSTATUSB\\[x\\].SIZE=ARG)"]
    #[inline(always)]
    pub fn wrsz(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Wrsz)
    }
}
impl W {
    #[doc = "Bits 0:23 - Command Argument (Watermark)"]
    #[inline(always)]
    pub fn arg(&mut self) -> ArgW<'_, RctrlWatermarkModeSpec> {
        ArgW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, RctrlWatermarkModeSpec> {
        CmdW::new(self, 24)
    }
}
#[doc = "Region Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rctrl_watermark_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RctrlWatermarkModeSpec;
impl crate::RegisterSpec for RctrlWatermarkModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rctrl_watermark_mode::W`](W) writer structure"]
impl crate::Writable for RctrlWatermarkModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCTRL_WATERMARK_MODE to value 0"]
impl crate::Resettable for RctrlWatermarkModeSpec {}
