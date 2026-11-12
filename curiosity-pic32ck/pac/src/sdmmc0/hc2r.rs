#[doc = "Register `HC2R` reader"]
pub type R = crate::R<Hc2rSpec>;
#[doc = "Register `HC2R` writer"]
pub type W = crate::W<Hc2rSpec>;
#[doc = "Asynchronous Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asintenselect {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Asintenselect> for bool {
    #[inline(always)]
    fn from(variant: Asintenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASINTEN` reader - Asynchronous Interrupt Enable"]
pub type AsintenR = crate::BitReader<Asintenselect>;
impl AsintenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asintenselect {
        match self.bits {
            false => Asintenselect::Disabled,
            true => Asintenselect::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Asintenselect::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Asintenselect::Enabled
    }
}
#[doc = "Field `ASINTEN` writer - Asynchronous Interrupt Enable"]
pub type AsintenW<'a, REG> = crate::BitWriter<'a, REG, Asintenselect>;
impl<'a, REG> AsintenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Asintenselect::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Asintenselect::Enabled)
    }
}
#[doc = "Preset Value Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvalenselect {
    #[doc = "0: SDCLK and Driver Strength are controlled by Host Controller"]
    Host = 0,
    #[doc = "1: Automatic Selection by Preset Value is Enabled"]
    Auto = 1,
}
impl From<Pvalenselect> for bool {
    #[inline(always)]
    fn from(variant: Pvalenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVALEN` reader - Preset Value Enable"]
pub type PvalenR = crate::BitReader<Pvalenselect>;
impl PvalenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvalenselect {
        match self.bits {
            false => Pvalenselect::Host,
            true => Pvalenselect::Auto,
        }
    }
    #[doc = "SDCLK and Driver Strength are controlled by Host Controller"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == Pvalenselect::Host
    }
    #[doc = "Automatic Selection by Preset Value is Enabled"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Pvalenselect::Auto
    }
}
#[doc = "Field `PVALEN` writer - Preset Value Enable"]
pub type PvalenW<'a, REG> = crate::BitWriter<'a, REG, Pvalenselect>;
impl<'a, REG> PvalenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDCLK and Driver Strength are controlled by Host Controller"]
    #[inline(always)]
    pub fn host(self) -> &'a mut crate::W<REG> {
        self.variant(Pvalenselect::Host)
    }
    #[doc = "Automatic Selection by Preset Value is Enabled"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Pvalenselect::Auto)
    }
}
impl R {
    #[doc = "Bit 14 - Asynchronous Interrupt Enable"]
    #[inline(always)]
    pub fn asinten(&self) -> AsintenR {
        AsintenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Preset Value Enable"]
    #[inline(always)]
    pub fn pvalen(&self) -> PvalenR {
        PvalenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Asynchronous Interrupt Enable"]
    #[inline(always)]
    pub fn asinten(&mut self) -> AsintenW<'_, Hc2rSpec> {
        AsintenW::new(self, 14)
    }
    #[doc = "Bit 15 - Preset Value Enable"]
    #[inline(always)]
    pub fn pvalen(&mut self) -> PvalenW<'_, Hc2rSpec> {
        PvalenW::new(self, 15)
    }
}
#[doc = "Host Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hc2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hc2rSpec;
impl crate::RegisterSpec for Hc2rSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hc2r::R`](R) reader structure"]
impl crate::Readable for Hc2rSpec {}
#[doc = "`write(|w| ..)` method takes [`hc2r::W`](W) writer structure"]
impl crate::Writable for Hc2rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HC2R to value 0"]
impl crate::Resettable for Hc2rSpec {}
