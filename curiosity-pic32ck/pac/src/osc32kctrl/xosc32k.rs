#[doc = "Register `XOSC32K` reader"]
pub type R = crate::R<Xosc32kSpec>;
#[doc = "Register `XOSC32K` writer"]
pub type W = crate::W<Xosc32kSpec>;
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub type XtalenR = crate::BitReader;
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub type XtalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Mode"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Mode"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Startup Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startupselect {
    #[doc = "0: 1 ULP clock cycle"]
    _1cycle = 0,
    #[doc = "1: 16 ULP clock cycles"]
    _16cycles = 1,
    #[doc = "2: 32 ULP clock cycles"]
    _32cycles = 2,
    #[doc = "3: 2048 ULP clock cycles"]
    _2048cycles = 3,
    #[doc = "4: 4096 ULP clock cycles"]
    _4096cycles = 4,
    #[doc = "5: 8192 ULP clock cycles"]
    _8192cycles = 5,
    #[doc = "6: 16384 ULP clock cycles"]
    _16384cycles = 6,
    #[doc = "7: 32768 ULP clock cycles"]
    _32768cycles = 7,
    #[doc = "8: 65536 ULP clock cycles"]
    _65536cycles = 8,
    #[doc = "9: 131072 ULP clock cycles"]
    _131072cycles = 9,
    #[doc = "10: 262144 ULP clock cycles"]
    _262144cycles = 10,
}
impl From<Startupselect> for u8 {
    #[inline(always)]
    fn from(variant: Startupselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Startupselect {
    type Ux = u8;
}
impl crate::IsEnum for Startupselect {}
#[doc = "Field `STARTUP` reader - Startup Mode"]
pub type StartupR = crate::FieldReader<Startupselect>;
impl StartupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Startupselect> {
        match self.bits {
            0 => Some(Startupselect::_1cycle),
            1 => Some(Startupselect::_16cycles),
            2 => Some(Startupselect::_32cycles),
            3 => Some(Startupselect::_2048cycles),
            4 => Some(Startupselect::_4096cycles),
            5 => Some(Startupselect::_8192cycles),
            6 => Some(Startupselect::_16384cycles),
            7 => Some(Startupselect::_32768cycles),
            8 => Some(Startupselect::_65536cycles),
            9 => Some(Startupselect::_131072cycles),
            10 => Some(Startupselect::_262144cycles),
            _ => None,
        }
    }
    #[doc = "1 ULP clock cycle"]
    #[inline(always)]
    pub fn is_1cycle(&self) -> bool {
        *self == Startupselect::_1cycle
    }
    #[doc = "16 ULP clock cycles"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == Startupselect::_16cycles
    }
    #[doc = "32 ULP clock cycles"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == Startupselect::_32cycles
    }
    #[doc = "2048 ULP clock cycles"]
    #[inline(always)]
    pub fn is_2048cycles(&self) -> bool {
        *self == Startupselect::_2048cycles
    }
    #[doc = "4096 ULP clock cycles"]
    #[inline(always)]
    pub fn is_4096cycles(&self) -> bool {
        *self == Startupselect::_4096cycles
    }
    #[doc = "8192 ULP clock cycles"]
    #[inline(always)]
    pub fn is_8192cycles(&self) -> bool {
        *self == Startupselect::_8192cycles
    }
    #[doc = "16384 ULP clock cycles"]
    #[inline(always)]
    pub fn is_16384cycles(&self) -> bool {
        *self == Startupselect::_16384cycles
    }
    #[doc = "32768 ULP clock cycles"]
    #[inline(always)]
    pub fn is_32768cycles(&self) -> bool {
        *self == Startupselect::_32768cycles
    }
    #[doc = "65536 ULP clock cycles"]
    #[inline(always)]
    pub fn is_65536cycles(&self) -> bool {
        *self == Startupselect::_65536cycles
    }
    #[doc = "131072 ULP clock cycles"]
    #[inline(always)]
    pub fn is_131072cycles(&self) -> bool {
        *self == Startupselect::_131072cycles
    }
    #[doc = "262144 ULP clock cycles"]
    #[inline(always)]
    pub fn is_262144cycles(&self) -> bool {
        *self == Startupselect::_262144cycles
    }
}
#[doc = "Field `STARTUP` writer - Startup Mode"]
pub type StartupW<'a, REG> = crate::FieldWriter<'a, REG, 4, Startupselect>;
impl<'a, REG> StartupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 ULP clock cycle"]
    #[inline(always)]
    pub fn _1cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::_1cycle)
    }
    #[doc = "16 ULP clock cycles"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::_16cycles)
    }
    #[doc = "32 ULP clock cycles"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::_32cycles)
    }
    #[doc = "2048 ULP clock cycles"]
    #[inline(always)]
    pub fn _2048cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::_2048cycles)
    }
    #[doc = "4096 ULP clock cycles"]
    #[inline(always)]
    pub fn _4096cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::_4096cycles)
    }
    #[doc = "8192 ULP clock cycles"]
    #[inline(always)]
    pub fn _8192cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::_8192cycles)
    }
    #[doc = "16384 ULP clock cycles"]
    #[inline(always)]
    pub fn _16384cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::_16384cycles)
    }
    #[doc = "32768 ULP clock cycles"]
    #[inline(always)]
    pub fn _32768cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::_32768cycles)
    }
    #[doc = "65536 ULP clock cycles"]
    #[inline(always)]
    pub fn _65536cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::_65536cycles)
    }
    #[doc = "131072 ULP clock cycles"]
    #[inline(always)]
    pub fn _131072cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::_131072cycles)
    }
    #[doc = "262144 ULP clock cycles"]
    #[inline(always)]
    pub fn _262144cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::_262144cycles)
    }
}
#[doc = "Control Gain Mode\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cgmselect {
    #[doc = "0: The lower Control Gain Mode value"]
    Cgm0 = 0,
    #[doc = "1: A higher Control Gain Mode value than CGM0"]
    Cgm1 = 1,
    #[doc = "2: A higher Control Gain Mode value than CGM1"]
    Cgm2 = 2,
    #[doc = "3: A higher Control Gain Mode value than CGM2"]
    Cgm3 = 3,
    #[doc = "4: A higher Control Gain Mode value than CGM3"]
    Cgm4 = 4,
    #[doc = "5: A higher Control Gain Mode value than CGM4"]
    Cgm5 = 5,
    #[doc = "6: A higher Control Gain Mode value than CGM5"]
    Cgm6 = 6,
    #[doc = "7: A higher Control Gain Mode value than CGM6"]
    Cgm7 = 7,
    #[doc = "8: A higher Control Gain Mode value than CGM7"]
    Cgm8 = 8,
    #[doc = "9: A higher Control Gain Mode value than CGM8"]
    Cgm9 = 9,
    #[doc = "10: A higher Control Gain Mode value than CGM9"]
    Cgm10 = 10,
    #[doc = "11: A higher Control Gain Mode value than CGM10"]
    Cgm11 = 11,
    #[doc = "12: A higher Control Gain Mode value than CGM11"]
    Cgm12 = 12,
    #[doc = "13: A higher Control Gain Mode value than CGM12"]
    Cgm13 = 13,
    #[doc = "14: A higher Control Gain Mode value than CGM13"]
    Cgm14 = 14,
    #[doc = "15: The highest Control Gain Mode value"]
    Cgm15 = 15,
}
impl From<Cgmselect> for u8 {
    #[inline(always)]
    fn from(variant: Cgmselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cgmselect {
    type Ux = u8;
}
impl crate::IsEnum for Cgmselect {}
#[doc = "Field `CGM` reader - Control Gain Mode"]
pub type CgmR = crate::FieldReader<Cgmselect>;
impl CgmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cgmselect {
        match self.bits {
            0 => Cgmselect::Cgm0,
            1 => Cgmselect::Cgm1,
            2 => Cgmselect::Cgm2,
            3 => Cgmselect::Cgm3,
            4 => Cgmselect::Cgm4,
            5 => Cgmselect::Cgm5,
            6 => Cgmselect::Cgm6,
            7 => Cgmselect::Cgm7,
            8 => Cgmselect::Cgm8,
            9 => Cgmselect::Cgm9,
            10 => Cgmselect::Cgm10,
            11 => Cgmselect::Cgm11,
            12 => Cgmselect::Cgm12,
            13 => Cgmselect::Cgm13,
            14 => Cgmselect::Cgm14,
            15 => Cgmselect::Cgm15,
            _ => unreachable!(),
        }
    }
    #[doc = "The lower Control Gain Mode value"]
    #[inline(always)]
    pub fn is_cgm0(&self) -> bool {
        *self == Cgmselect::Cgm0
    }
    #[doc = "A higher Control Gain Mode value than CGM0"]
    #[inline(always)]
    pub fn is_cgm1(&self) -> bool {
        *self == Cgmselect::Cgm1
    }
    #[doc = "A higher Control Gain Mode value than CGM1"]
    #[inline(always)]
    pub fn is_cgm2(&self) -> bool {
        *self == Cgmselect::Cgm2
    }
    #[doc = "A higher Control Gain Mode value than CGM2"]
    #[inline(always)]
    pub fn is_cgm3(&self) -> bool {
        *self == Cgmselect::Cgm3
    }
    #[doc = "A higher Control Gain Mode value than CGM3"]
    #[inline(always)]
    pub fn is_cgm4(&self) -> bool {
        *self == Cgmselect::Cgm4
    }
    #[doc = "A higher Control Gain Mode value than CGM4"]
    #[inline(always)]
    pub fn is_cgm5(&self) -> bool {
        *self == Cgmselect::Cgm5
    }
    #[doc = "A higher Control Gain Mode value than CGM5"]
    #[inline(always)]
    pub fn is_cgm6(&self) -> bool {
        *self == Cgmselect::Cgm6
    }
    #[doc = "A higher Control Gain Mode value than CGM6"]
    #[inline(always)]
    pub fn is_cgm7(&self) -> bool {
        *self == Cgmselect::Cgm7
    }
    #[doc = "A higher Control Gain Mode value than CGM7"]
    #[inline(always)]
    pub fn is_cgm8(&self) -> bool {
        *self == Cgmselect::Cgm8
    }
    #[doc = "A higher Control Gain Mode value than CGM8"]
    #[inline(always)]
    pub fn is_cgm9(&self) -> bool {
        *self == Cgmselect::Cgm9
    }
    #[doc = "A higher Control Gain Mode value than CGM9"]
    #[inline(always)]
    pub fn is_cgm10(&self) -> bool {
        *self == Cgmselect::Cgm10
    }
    #[doc = "A higher Control Gain Mode value than CGM10"]
    #[inline(always)]
    pub fn is_cgm11(&self) -> bool {
        *self == Cgmselect::Cgm11
    }
    #[doc = "A higher Control Gain Mode value than CGM11"]
    #[inline(always)]
    pub fn is_cgm12(&self) -> bool {
        *self == Cgmselect::Cgm12
    }
    #[doc = "A higher Control Gain Mode value than CGM12"]
    #[inline(always)]
    pub fn is_cgm13(&self) -> bool {
        *self == Cgmselect::Cgm13
    }
    #[doc = "A higher Control Gain Mode value than CGM13"]
    #[inline(always)]
    pub fn is_cgm14(&self) -> bool {
        *self == Cgmselect::Cgm14
    }
    #[doc = "The highest Control Gain Mode value"]
    #[inline(always)]
    pub fn is_cgm15(&self) -> bool {
        *self == Cgmselect::Cgm15
    }
}
#[doc = "Field `CGM` writer - Control Gain Mode"]
pub type CgmW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cgmselect, crate::Safe>;
impl<'a, REG> CgmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The lower Control Gain Mode value"]
    #[inline(always)]
    pub fn cgm0(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm0)
    }
    #[doc = "A higher Control Gain Mode value than CGM0"]
    #[inline(always)]
    pub fn cgm1(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm1)
    }
    #[doc = "A higher Control Gain Mode value than CGM1"]
    #[inline(always)]
    pub fn cgm2(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm2)
    }
    #[doc = "A higher Control Gain Mode value than CGM2"]
    #[inline(always)]
    pub fn cgm3(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm3)
    }
    #[doc = "A higher Control Gain Mode value than CGM3"]
    #[inline(always)]
    pub fn cgm4(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm4)
    }
    #[doc = "A higher Control Gain Mode value than CGM4"]
    #[inline(always)]
    pub fn cgm5(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm5)
    }
    #[doc = "A higher Control Gain Mode value than CGM5"]
    #[inline(always)]
    pub fn cgm6(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm6)
    }
    #[doc = "A higher Control Gain Mode value than CGM6"]
    #[inline(always)]
    pub fn cgm7(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm7)
    }
    #[doc = "A higher Control Gain Mode value than CGM7"]
    #[inline(always)]
    pub fn cgm8(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm8)
    }
    #[doc = "A higher Control Gain Mode value than CGM8"]
    #[inline(always)]
    pub fn cgm9(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm9)
    }
    #[doc = "A higher Control Gain Mode value than CGM9"]
    #[inline(always)]
    pub fn cgm10(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm10)
    }
    #[doc = "A higher Control Gain Mode value than CGM10"]
    #[inline(always)]
    pub fn cgm11(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm11)
    }
    #[doc = "A higher Control Gain Mode value than CGM11"]
    #[inline(always)]
    pub fn cgm12(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm12)
    }
    #[doc = "A higher Control Gain Mode value than CGM12"]
    #[inline(always)]
    pub fn cgm13(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm13)
    }
    #[doc = "A higher Control Gain Mode value than CGM13"]
    #[inline(always)]
    pub fn cgm14(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm14)
    }
    #[doc = "The highest Control Gain Mode value"]
    #[inline(always)]
    pub fn cgm15(self) -> &'a mut crate::W<REG> {
        self.variant(Cgmselect::Cgm15)
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&self) -> XtalenR {
        XtalenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Mode"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Startup Mode"]
    #[inline(always)]
    pub fn startup(&self) -> StartupR {
        StartupR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - Control Gain Mode"]
    #[inline(always)]
    pub fn cgm(&self) -> CgmR {
        CgmR::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Xosc32kSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&mut self) -> XtalenW<'_, Xosc32kSpec> {
        XtalenW::new(self, 2)
    }
    #[doc = "Bit 7 - On Demand Mode"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> OndemandW<'_, Xosc32kSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Startup Mode"]
    #[inline(always)]
    pub fn startup(&mut self) -> StartupW<'_, Xosc32kSpec> {
        StartupW::new(self, 8)
    }
    #[doc = "Bits 18:21 - Control Gain Mode"]
    #[inline(always)]
    pub fn cgm(&mut self) -> CgmW<'_, Xosc32kSpec> {
        CgmW::new(self, 18)
    }
}
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xosc32k::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc32k::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xosc32kSpec;
impl crate::RegisterSpec for Xosc32kSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xosc32k::R`](R) reader structure"]
impl crate::Readable for Xosc32kSpec {}
#[doc = "`write(|w| ..)` method takes [`xosc32k::W`](W) writer structure"]
impl crate::Writable for Xosc32kSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XOSC32K to value 0x0020_0080"]
impl crate::Resettable for Xosc32kSpec {
    const RESET_VALUE: u32 = 0x0020_0080;
}
