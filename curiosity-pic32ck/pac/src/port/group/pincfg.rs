#[doc = "Register `PINCFG[%s]` reader"]
pub type R = crate::R<PincfgSpec>;
#[doc = "Register `PINCFG[%s]` writer"]
pub type W = crate::W<PincfgSpec>;
#[doc = "Field `PMUXEN` reader - Peripheral Multiplexer Enable"]
pub type PmuxenR = crate::BitReader;
#[doc = "Field `PMUXEN` writer - Peripheral Multiplexer Enable"]
pub type PmuxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN` reader - Input Enable"]
pub type InenR = crate::BitReader;
#[doc = "Field `INEN` writer - Input Enable"]
pub type InenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULLEN` reader - Pull Enable"]
pub type PullenR = crate::BitReader;
#[doc = "Field `PULLEN` writer - Pull Enable"]
pub type PullenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODRAIN` reader - Open Drain Output"]
pub type OdrainR = crate::BitReader;
#[doc = "Field `ODRAIN` writer - Open Drain Output"]
pub type OdrainW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Output Driver Slew Rate Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Slewlimselect {
    #[doc = "0: Slew rate control disabled (fast rise/fall time operation)"]
    Fast = 0,
    #[doc = "1: Slew rate control enabled (4x slower)"]
    Slow4 = 1,
    #[doc = "2: Slew rate control enabled (8x slower)"]
    Slow8 = 2,
    #[doc = "3: Slew rate control enabled (12x slower)"]
    Slow12 = 3,
}
impl From<Slewlimselect> for u8 {
    #[inline(always)]
    fn from(variant: Slewlimselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Slewlimselect {
    type Ux = u8;
}
impl crate::IsEnum for Slewlimselect {}
#[doc = "Field `SLEWLIM` reader - Output Driver Slew Rate Selection"]
pub type SlewlimR = crate::FieldReader<Slewlimselect>;
impl SlewlimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slewlimselect {
        match self.bits {
            0 => Slewlimselect::Fast,
            1 => Slewlimselect::Slow4,
            2 => Slewlimselect::Slow8,
            3 => Slewlimselect::Slow12,
            _ => unreachable!(),
        }
    }
    #[doc = "Slew rate control disabled (fast rise/fall time operation)"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == Slewlimselect::Fast
    }
    #[doc = "Slew rate control enabled (4x slower)"]
    #[inline(always)]
    pub fn is_slow4(&self) -> bool {
        *self == Slewlimselect::Slow4
    }
    #[doc = "Slew rate control enabled (8x slower)"]
    #[inline(always)]
    pub fn is_slow8(&self) -> bool {
        *self == Slewlimselect::Slow8
    }
    #[doc = "Slew rate control enabled (12x slower)"]
    #[inline(always)]
    pub fn is_slow12(&self) -> bool {
        *self == Slewlimselect::Slow12
    }
}
#[doc = "Field `SLEWLIM` writer - Output Driver Slew Rate Selection"]
pub type SlewlimW<'a, REG> = crate::FieldWriter<'a, REG, 2, Slewlimselect, crate::Safe>;
impl<'a, REG> SlewlimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slew rate control disabled (fast rise/fall time operation)"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(Slewlimselect::Fast)
    }
    #[doc = "Slew rate control enabled (4x slower)"]
    #[inline(always)]
    pub fn slow4(self) -> &'a mut crate::W<REG> {
        self.variant(Slewlimselect::Slow4)
    }
    #[doc = "Slew rate control enabled (8x slower)"]
    #[inline(always)]
    pub fn slow8(self) -> &'a mut crate::W<REG> {
        self.variant(Slewlimselect::Slow8)
    }
    #[doc = "Slew rate control enabled (12x slower)"]
    #[inline(always)]
    pub fn slow12(self) -> &'a mut crate::W<REG> {
        self.variant(Slewlimselect::Slow12)
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&self) -> PmuxenR {
        PmuxenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Enable"]
    #[inline(always)]
    pub fn inen(&self) -> InenR {
        InenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&self) -> PullenR {
        PullenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Open Drain Output"]
    #[inline(always)]
    pub fn odrain(&self) -> OdrainR {
        OdrainR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Output Driver Slew Rate Selection"]
    #[inline(always)]
    pub fn slewlim(&self) -> SlewlimR {
        SlewlimR::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&mut self) -> PmuxenW<'_, PincfgSpec> {
        PmuxenW::new(self, 0)
    }
    #[doc = "Bit 1 - Input Enable"]
    #[inline(always)]
    pub fn inen(&mut self) -> InenW<'_, PincfgSpec> {
        InenW::new(self, 1)
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&mut self) -> PullenW<'_, PincfgSpec> {
        PullenW::new(self, 2)
    }
    #[doc = "Bit 3 - Open Drain Output"]
    #[inline(always)]
    pub fn odrain(&mut self) -> OdrainW<'_, PincfgSpec> {
        OdrainW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Output Driver Slew Rate Selection"]
    #[inline(always)]
    pub fn slewlim(&mut self) -> SlewlimW<'_, PincfgSpec> {
        SlewlimW::new(self, 4)
    }
}
#[doc = "Pin Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PincfgSpec;
impl crate::RegisterSpec for PincfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pincfg::R`](R) reader structure"]
impl crate::Readable for PincfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pincfg::W`](W) writer structure"]
impl crate::Writable for PincfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINCFG[%s] to value 0"]
impl crate::Resettable for PincfgSpec {}
