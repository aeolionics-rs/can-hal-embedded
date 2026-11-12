#[doc = "Register `RCTRL_BLOCK_MODE` writer"]
pub type W = crate::W<RctrlBlockModeSpec>;
#[doc = "Field `ARG` writer - Command Argument (Block ID)"]
pub type ArgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdselect {
    #[doc = "90: Clear IDAU region x Non-Secure State of block ARG (RSTATUSB\\[x\\].NONSEC\\[ARG\\]=0)"]
    Clrnonsec = 90,
    #[doc = "91: Set IDAU region x Non-Secure State of block ARG (RSTATUSB\\[x\\].NONSEC\\[ARG\\]=1)"]
    Setnonsec = 91,
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
    #[doc = "Clear IDAU region x Non-Secure State of block ARG (RSTATUSB\\[x\\].NONSEC\\[ARG\\]=0)"]
    #[inline(always)]
    pub fn clrnonsec(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Clrnonsec)
    }
    #[doc = "Set IDAU region x Non-Secure State of block ARG (RSTATUSB\\[x\\].NONSEC\\[ARG\\]=1)"]
    #[inline(always)]
    pub fn setnonsec(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Setnonsec)
    }
}
impl W {
    #[doc = "Bits 0:4 - Command Argument (Block ID)"]
    #[inline(always)]
    pub fn arg(&mut self) -> ArgW<'_, RctrlBlockModeSpec> {
        ArgW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, RctrlBlockModeSpec> {
        CmdW::new(self, 24)
    }
}
#[doc = "Region Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rctrl_block_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RctrlBlockModeSpec;
impl crate::RegisterSpec for RctrlBlockModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rctrl_block_mode::W`](W) writer structure"]
impl crate::Writable for RctrlBlockModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCTRL_BLOCK_MODE to value 0"]
impl crate::Resettable for RctrlBlockModeSpec {}
