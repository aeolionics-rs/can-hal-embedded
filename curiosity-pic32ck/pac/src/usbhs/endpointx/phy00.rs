#[doc = "Register `PHY00` reader"]
pub type R = crate::R<Phy00Spec>;
#[doc = "Register `PHY00` writer"]
pub type W = crate::W<Phy00Spec>;
#[doc = "enables pre-emphasis\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Preempselect {
    #[doc = "0: Enable pre-emphasis during SOF and EOP"]
    Sofeop = 0,
    #[doc = "1: Enable pre-emphasis during chirp"]
    Chirp = 1,
    #[doc = "2: Enable pre-emphasis in non-chirp state"]
    Nonchirp = 2,
    #[doc = "3: Always enable pre-emphasis"]
    Always = 3,
}
impl From<Preempselect> for u8 {
    #[inline(always)]
    fn from(variant: Preempselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Preempselect {
    type Ux = u8;
}
impl crate::IsEnum for Preempselect {}
#[doc = "Field `PREEMP` reader - enables pre-emphasis"]
pub type PreempR = crate::FieldReader<Preempselect>;
impl PreempR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Preempselect> {
        match self.bits {
            0 => Some(Preempselect::Sofeop),
            1 => Some(Preempselect::Chirp),
            2 => Some(Preempselect::Nonchirp),
            3 => Some(Preempselect::Always),
            _ => None,
        }
    }
    #[doc = "Enable pre-emphasis during SOF and EOP"]
    #[inline(always)]
    pub fn is_sofeop(&self) -> bool {
        *self == Preempselect::Sofeop
    }
    #[doc = "Enable pre-emphasis during chirp"]
    #[inline(always)]
    pub fn is_chirp(&self) -> bool {
        *self == Preempselect::Chirp
    }
    #[doc = "Enable pre-emphasis in non-chirp state"]
    #[inline(always)]
    pub fn is_nonchirp(&self) -> bool {
        *self == Preempselect::Nonchirp
    }
    #[doc = "Always enable pre-emphasis"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == Preempselect::Always
    }
}
#[doc = "Field `PREEMP` writer - enables pre-emphasis"]
pub type PreempW<'a, REG> = crate::FieldWriter<'a, REG, 3, Preempselect>;
impl<'a, REG> PreempW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable pre-emphasis during SOF and EOP"]
    #[inline(always)]
    pub fn sofeop(self) -> &'a mut crate::W<REG> {
        self.variant(Preempselect::Sofeop)
    }
    #[doc = "Enable pre-emphasis during chirp"]
    #[inline(always)]
    pub fn chirp(self) -> &'a mut crate::W<REG> {
        self.variant(Preempselect::Chirp)
    }
    #[doc = "Enable pre-emphasis in non-chirp state"]
    #[inline(always)]
    pub fn nonchirp(self) -> &'a mut crate::W<REG> {
        self.variant(Preempselect::Nonchirp)
    }
    #[doc = "Always enable pre-emphasis"]
    #[inline(always)]
    pub fn always(self) -> &'a mut crate::W<REG> {
        self.variant(Preempselect::Always)
    }
}
#[doc = "adjust FS/LS slew rate\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Slewrateselect {
    #[doc = "0: Highest slew rate"]
    High = 0,
    #[doc = "1: Middle slew rate"]
    Middle0 = 1,
    #[doc = "2: Middle slew rate"]
    Middle1 = 2,
    #[doc = "3: Smallest slew rate"]
    Smallest = 3,
}
impl From<Slewrateselect> for u8 {
    #[inline(always)]
    fn from(variant: Slewrateselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Slewrateselect {
    type Ux = u8;
}
impl crate::IsEnum for Slewrateselect {}
#[doc = "Field `SLEWRATE` reader - adjust FS/LS slew rate"]
pub type SlewrateR = crate::FieldReader<Slewrateselect>;
impl SlewrateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slewrateselect {
        match self.bits {
            0 => Slewrateselect::High,
            1 => Slewrateselect::Middle0,
            2 => Slewrateselect::Middle1,
            3 => Slewrateselect::Smallest,
            _ => unreachable!(),
        }
    }
    #[doc = "Highest slew rate"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Slewrateselect::High
    }
    #[doc = "Middle slew rate"]
    #[inline(always)]
    pub fn is_middle0(&self) -> bool {
        *self == Slewrateselect::Middle0
    }
    #[doc = "Middle slew rate"]
    #[inline(always)]
    pub fn is_middle1(&self) -> bool {
        *self == Slewrateselect::Middle1
    }
    #[doc = "Smallest slew rate"]
    #[inline(always)]
    pub fn is_smallest(&self) -> bool {
        *self == Slewrateselect::Smallest
    }
}
#[doc = "Field `SLEWRATE` writer - adjust FS/LS slew rate"]
pub type SlewrateW<'a, REG> = crate::FieldWriter<'a, REG, 2, Slewrateselect, crate::Safe>;
impl<'a, REG> SlewrateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Highest slew rate"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Slewrateselect::High)
    }
    #[doc = "Middle slew rate"]
    #[inline(always)]
    pub fn middle0(self) -> &'a mut crate::W<REG> {
        self.variant(Slewrateselect::Middle0)
    }
    #[doc = "Middle slew rate"]
    #[inline(always)]
    pub fn middle1(self) -> &'a mut crate::W<REG> {
        self.variant(Slewrateselect::Middle1)
    }
    #[doc = "Smallest slew rate"]
    #[inline(always)]
    pub fn smallest(self) -> &'a mut crate::W<REG> {
        self.variant(Slewrateselect::Smallest)
    }
}
#[doc = "RX clock phase select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxphsselselect {
    #[doc = "0: Earliest phase"]
    Earliest = 0,
    #[doc = "7: Latest phase"]
    Latest = 7,
}
impl From<Rxphsselselect> for u8 {
    #[inline(always)]
    fn from(variant: Rxphsselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxphsselselect {
    type Ux = u8;
}
impl crate::IsEnum for Rxphsselselect {}
#[doc = "Field `RXPHSSEL` reader - RX clock phase select"]
pub type RxphsselR = crate::FieldReader<Rxphsselselect>;
impl RxphsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxphsselselect> {
        match self.bits {
            0 => Some(Rxphsselselect::Earliest),
            7 => Some(Rxphsselselect::Latest),
            _ => None,
        }
    }
    #[doc = "Earliest phase"]
    #[inline(always)]
    pub fn is_earliest(&self) -> bool {
        *self == Rxphsselselect::Earliest
    }
    #[doc = "Latest phase"]
    #[inline(always)]
    pub fn is_latest(&self) -> bool {
        *self == Rxphsselselect::Latest
    }
}
#[doc = "Field `RXPHSSEL` writer - RX clock phase select"]
pub type RxphsselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rxphsselselect>;
impl<'a, REG> RxphsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Earliest phase"]
    #[inline(always)]
    pub fn earliest(self) -> &'a mut crate::W<REG> {
        self.variant(Rxphsselselect::Earliest)
    }
    #[doc = "Latest phase"]
    #[inline(always)]
    pub fn latest(self) -> &'a mut crate::W<REG> {
        self.variant(Rxphsselselect::Latest)
    }
}
impl R {
    #[doc = "Bits 0:2 - enables pre-emphasis"]
    #[inline(always)]
    pub fn preemp(&self) -> PreempR {
        PreempR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - adjust FS/LS slew rate"]
    #[inline(always)]
    pub fn slewrate(&self) -> SlewrateR {
        SlewrateR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - RX clock phase select"]
    #[inline(always)]
    pub fn rxphssel(&self) -> RxphsselR {
        RxphsselR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - enables pre-emphasis"]
    #[inline(always)]
    pub fn preemp(&mut self) -> PreempW<'_, Phy00Spec> {
        PreempW::new(self, 0)
    }
    #[doc = "Bits 3:4 - adjust FS/LS slew rate"]
    #[inline(always)]
    pub fn slewrate(&mut self) -> SlewrateW<'_, Phy00Spec> {
        SlewrateW::new(self, 3)
    }
    #[doc = "Bits 5:7 - RX clock phase select"]
    #[inline(always)]
    pub fn rxphssel(&mut self) -> RxphsselW<'_, Phy00Spec> {
        RxphsselW::new(self, 5)
    }
}
#[doc = "USBHPHY Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`phy00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phy00Spec;
impl crate::RegisterSpec for Phy00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy00::R`](R) reader structure"]
impl crate::Readable for Phy00Spec {}
#[doc = "`write(|w| ..)` method takes [`phy00::W`](W) writer structure"]
impl crate::Writable for Phy00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY00 to value 0x19"]
impl crate::Resettable for Phy00Spec {
    const RESET_VALUE: u32 = 0x19;
}
