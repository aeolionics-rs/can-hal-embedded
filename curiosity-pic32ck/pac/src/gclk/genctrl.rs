#[doc = "Register `GENCTRL[%s]` reader"]
pub type R = crate::R<GenctrlSpec>;
#[doc = "Register `GENCTRL[%s]` writer"]
pub type W = crate::W<GenctrlSpec>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Srcselect {
    #[doc = "0: XOSC0 oscillator output"]
    Xosc = 0,
    #[doc = "1: Generator input pad"]
    Gclkin = 1,
    #[doc = "2: Generic clock generator 1 output"]
    Gclkgen1 = 2,
    #[doc = "3: OSCULP32K oscillator output"]
    Osculp32k = 3,
    #[doc = "4: XOSC32K oscillator output"]
    Xosc32k = 4,
    #[doc = "5: DFLL output"]
    Dfll = 5,
    #[doc = "6: Frac Divider PLL0 output 0"]
    Pll0_0 = 6,
    #[doc = "7: PLL0 output 1"]
    Pll0_1 = 7,
    #[doc = "8: PLL0 output 2"]
    Pll0_2 = 8,
    #[doc = "9: PLL0 output 3"]
    Pll0_3 = 9,
    #[doc = "10: PLL0 output 4"]
    Pll0_4 = 10,
    #[doc = "11: PLL0 output 5"]
    Pll0_5 = 11,
}
impl From<Srcselect> for u8 {
    #[inline(always)]
    fn from(variant: Srcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Srcselect {
    type Ux = u8;
}
impl crate::IsEnum for Srcselect {}
#[doc = "Field `SRC` reader - Source Select"]
pub type SrcR = crate::FieldReader<Srcselect>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Srcselect> {
        match self.bits {
            0 => Some(Srcselect::Xosc),
            1 => Some(Srcselect::Gclkin),
            2 => Some(Srcselect::Gclkgen1),
            3 => Some(Srcselect::Osculp32k),
            4 => Some(Srcselect::Xosc32k),
            5 => Some(Srcselect::Dfll),
            6 => Some(Srcselect::Pll0_0),
            7 => Some(Srcselect::Pll0_1),
            8 => Some(Srcselect::Pll0_2),
            9 => Some(Srcselect::Pll0_3),
            10 => Some(Srcselect::Pll0_4),
            11 => Some(Srcselect::Pll0_5),
            _ => None,
        }
    }
    #[doc = "XOSC0 oscillator output"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == Srcselect::Xosc
    }
    #[doc = "Generator input pad"]
    #[inline(always)]
    pub fn is_gclkin(&self) -> bool {
        *self == Srcselect::Gclkin
    }
    #[doc = "Generic clock generator 1 output"]
    #[inline(always)]
    pub fn is_gclkgen1(&self) -> bool {
        *self == Srcselect::Gclkgen1
    }
    #[doc = "OSCULP32K oscillator output"]
    #[inline(always)]
    pub fn is_osculp32k(&self) -> bool {
        *self == Srcselect::Osculp32k
    }
    #[doc = "XOSC32K oscillator output"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == Srcselect::Xosc32k
    }
    #[doc = "DFLL output"]
    #[inline(always)]
    pub fn is_dfll(&self) -> bool {
        *self == Srcselect::Dfll
    }
    #[doc = "Frac Divider PLL0 output 0"]
    #[inline(always)]
    pub fn is_pll0_0(&self) -> bool {
        *self == Srcselect::Pll0_0
    }
    #[doc = "PLL0 output 1"]
    #[inline(always)]
    pub fn is_pll0_1(&self) -> bool {
        *self == Srcselect::Pll0_1
    }
    #[doc = "PLL0 output 2"]
    #[inline(always)]
    pub fn is_pll0_2(&self) -> bool {
        *self == Srcselect::Pll0_2
    }
    #[doc = "PLL0 output 3"]
    #[inline(always)]
    pub fn is_pll0_3(&self) -> bool {
        *self == Srcselect::Pll0_3
    }
    #[doc = "PLL0 output 4"]
    #[inline(always)]
    pub fn is_pll0_4(&self) -> bool {
        *self == Srcselect::Pll0_4
    }
    #[doc = "PLL0 output 5"]
    #[inline(always)]
    pub fn is_pll0_5(&self) -> bool {
        *self == Srcselect::Pll0_5
    }
}
#[doc = "Field `SRC` writer - Source Select"]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Srcselect>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XOSC0 oscillator output"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Xosc)
    }
    #[doc = "Generator input pad"]
    #[inline(always)]
    pub fn gclkin(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Gclkin)
    }
    #[doc = "Generic clock generator 1 output"]
    #[inline(always)]
    pub fn gclkgen1(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Gclkgen1)
    }
    #[doc = "OSCULP32K oscillator output"]
    #[inline(always)]
    pub fn osculp32k(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Osculp32k)
    }
    #[doc = "XOSC32K oscillator output"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Xosc32k)
    }
    #[doc = "DFLL output"]
    #[inline(always)]
    pub fn dfll(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Dfll)
    }
    #[doc = "Frac Divider PLL0 output 0"]
    #[inline(always)]
    pub fn pll0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Pll0_0)
    }
    #[doc = "PLL0 output 1"]
    #[inline(always)]
    pub fn pll0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Pll0_1)
    }
    #[doc = "PLL0 output 2"]
    #[inline(always)]
    pub fn pll0_2(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Pll0_2)
    }
    #[doc = "PLL0 output 3"]
    #[inline(always)]
    pub fn pll0_3(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Pll0_3)
    }
    #[doc = "PLL0 output 4"]
    #[inline(always)]
    pub fn pll0_4(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Pll0_4)
    }
    #[doc = "PLL0 output 5"]
    #[inline(always)]
    pub fn pll0_5(self) -> &'a mut crate::W<REG> {
        self.variant(Srcselect::Pll0_5)
    }
}
#[doc = "Field `GENEN` reader - Generic Clock Generator Enable"]
pub type GenenR = crate::BitReader;
#[doc = "Field `GENEN` writer - Generic Clock Generator Enable"]
pub type GenenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDC` reader - Improve Duty Cycle"]
pub type IdcR = crate::BitReader;
#[doc = "Field `IDC` writer - Improve Duty Cycle"]
pub type IdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOV` reader - Output Off Value"]
pub type OovR = crate::BitReader;
#[doc = "Field `OOV` writer - Output Off Value"]
pub type OovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE` reader - Output Enable"]
pub type OeR = crate::BitReader;
#[doc = "Field `OE` writer - Output Enable"]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Divide Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Divselselect {
    #[doc = "0: Divide input directly by divider factor"]
    Div1 = 0,
    #[doc = "1: Divide input by 2^(divider factor+ 1)"]
    Div2 = 1,
}
impl From<Divselselect> for bool {
    #[inline(always)]
    fn from(variant: Divselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVSEL` reader - Divide Selection"]
pub type DivselR = crate::BitReader<Divselselect>;
impl DivselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Divselselect {
        match self.bits {
            false => Divselselect::Div1,
            true => Divselselect::Div2,
        }
    }
    #[doc = "Divide input directly by divider factor"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Divselselect::Div1
    }
    #[doc = "Divide input by 2^(divider factor+ 1)"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Divselselect::Div2
    }
}
#[doc = "Field `DIVSEL` writer - Divide Selection"]
pub type DivselW<'a, REG> = crate::BitWriter<'a, REG, Divselselect>;
impl<'a, REG> DivselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Divide input directly by divider factor"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Divselselect::Div1)
    }
    #[doc = "Divide input by 2^(divider factor+ 1)"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Divselselect::Div2)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV` reader - Division Factor"]
pub type DivR = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Division Factor"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Source Select"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Generic Clock Generator Enable"]
    #[inline(always)]
    pub fn genen(&self) -> GenenR {
        GenenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Improve Duty Cycle"]
    #[inline(always)]
    pub fn idc(&self) -> IdcR {
        IdcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output Off Value"]
    #[inline(always)]
    pub fn oov(&self) -> OovR {
        OovR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output Enable"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Divide Selection"]
    #[inline(always)]
    pub fn divsel(&self) -> DivselR {
        DivselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Source Select"]
    #[inline(always)]
    pub fn src(&mut self) -> SrcW<'_, GenctrlSpec> {
        SrcW::new(self, 0)
    }
    #[doc = "Bit 8 - Generic Clock Generator Enable"]
    #[inline(always)]
    pub fn genen(&mut self) -> GenenW<'_, GenctrlSpec> {
        GenenW::new(self, 8)
    }
    #[doc = "Bit 9 - Improve Duty Cycle"]
    #[inline(always)]
    pub fn idc(&mut self) -> IdcW<'_, GenctrlSpec> {
        IdcW::new(self, 9)
    }
    #[doc = "Bit 10 - Output Off Value"]
    #[inline(always)]
    pub fn oov(&mut self) -> OovW<'_, GenctrlSpec> {
        OovW::new(self, 10)
    }
    #[doc = "Bit 11 - Output Enable"]
    #[inline(always)]
    pub fn oe(&mut self) -> OeW<'_, GenctrlSpec> {
        OeW::new(self, 11)
    }
    #[doc = "Bit 12 - Divide Selection"]
    #[inline(always)]
    pub fn divsel(&mut self) -> DivselW<'_, GenctrlSpec> {
        DivselW::new(self, 12)
    }
    #[doc = "Bit 13 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, GenctrlSpec> {
        RunstdbyW::new(self, 13)
    }
    #[doc = "Bits 16:31 - Division Factor"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, GenctrlSpec> {
        DivW::new(self, 16)
    }
}
#[doc = "Generic Clock Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`genctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`genctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GenctrlSpec;
impl crate::RegisterSpec for GenctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`genctrl::R`](R) reader structure"]
impl crate::Readable for GenctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`genctrl::W`](W) writer structure"]
impl crate::Writable for GenctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GENCTRL[%s] to value 0"]
impl crate::Resettable for GenctrlSpec {}
