#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Command Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Cmdselect {
    #[doc = "0: No operation"]
    Noop = 0,
    #[doc = "42240: 32-bit Cyclic Redundancy Code"]
    Crc32 = 42240,
    #[doc = "42241: Global Memory built-in self-test"]
    Mbist = 42241,
    #[doc = "42242: Memory Set"]
    Mset = 42242,
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
    #[doc = "No operation"]
    #[inline(always)]
    pub fn noop(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Noop)
    }
    #[doc = "32-bit Cyclic Redundancy Code"]
    #[inline(always)]
    pub fn crc32(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Crc32)
    }
    #[doc = "Global Memory built-in self-test"]
    #[inline(always)]
    pub fn mbist(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Mbist)
    }
    #[doc = "Memory Set"]
    #[inline(always)]
    pub fn mset(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdselect::Mset)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, CtrlSpec> {
        SwrstW::new(self, 0)
    }
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
