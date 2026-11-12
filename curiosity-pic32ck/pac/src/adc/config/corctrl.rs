#[doc = "Register `CORCTRL` reader"]
pub type R = crate::R<CorctrlSpec>;
#[doc = "Register `CORCTRL` writer"]
pub type W = crate::W<CorctrlSpec>;
#[doc = "Field `SAMC` reader - Sample Count"]
pub type SamcR = crate::FieldReader<u16>;
#[doc = "Field `SAMC` writer - Sample Count"]
pub type SamcW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Selects Resolution\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selresselect {
    #[doc = "0: 6 bits"]
    _6Bits = 0,
    #[doc = "1: 8 bits"]
    _8Bits = 1,
    #[doc = "2: 10 bits"]
    _10Bits = 2,
    #[doc = "3: 12 bits (default)"]
    _12Bits = 3,
}
impl From<Selresselect> for u8 {
    #[inline(always)]
    fn from(variant: Selresselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selresselect {
    type Ux = u8;
}
impl crate::IsEnum for Selresselect {}
#[doc = "Field `SELRES` reader - Selects Resolution"]
pub type SelresR = crate::FieldReader<Selresselect>;
impl SelresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selresselect {
        match self.bits {
            0 => Selresselect::_6Bits,
            1 => Selresselect::_8Bits,
            2 => Selresselect::_10Bits,
            3 => Selresselect::_12Bits,
            _ => unreachable!(),
        }
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn is_6_bits(&self) -> bool {
        *self == Selresselect::_6Bits
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_8_bits(&self) -> bool {
        *self == Selresselect::_8Bits
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn is_10_bits(&self) -> bool {
        *self == Selresselect::_10Bits
    }
    #[doc = "12 bits (default)"]
    #[inline(always)]
    pub fn is_12_bits(&self) -> bool {
        *self == Selresselect::_12Bits
    }
}
#[doc = "Field `SELRES` writer - Selects Resolution"]
pub type SelresW<'a, REG> = crate::FieldWriter<'a, REG, 2, Selresselect, crate::Safe>;
impl<'a, REG> SelresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn _6_bits(self) -> &'a mut crate::W<REG> {
        self.variant(Selresselect::_6Bits)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8_bits(self) -> &'a mut crate::W<REG> {
        self.variant(Selresselect::_8Bits)
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn _10_bits(self) -> &'a mut crate::W<REG> {
        self.variant(Selresselect::_10Bits)
    }
    #[doc = "12 bits (default)"]
    #[inline(always)]
    pub fn _12_bits(self) -> &'a mut crate::W<REG> {
        self.variant(Selresselect::_12Bits)
    }
}
#[doc = "Field `EIS` reader - Early Interrupt Select"]
pub type EisR = crate::FieldReader;
#[doc = "Field `EIS` writer - Early Interrupt Select"]
pub type EisW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EIRQOVR` reader - Interrupt Type Select"]
pub type EirqovrR = crate::BitReader;
#[doc = "Field `EIRQOVR` writer - Interrupt Type Select"]
pub type EirqovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SCAN trigger source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Strgsrcselect {
    #[doc = "0: No Trigger (NOP)"]
    NoTrigger = 0,
    #[doc = "1: Global Software Trigger"]
    GlobalSoftwareTrigger = 1,
    #[doc = "2: Global Level Software Trigger"]
    GlobalLevelTrigger = 2,
    #[doc = "4: STRIG Synchronous Trigger"]
    SyncTrigger = 4,
    #[doc = "5: ADC Trigger Event User 0"]
    EventUser0 = 5,
    #[doc = "6: ADC Trigger Event User 1"]
    EventUser1 = 6,
    #[doc = "7: ADC Trigger Event User 2"]
    EventUser2 = 7,
    #[doc = "8: ADC Trigger Event User 3"]
    EventUser3 = 8,
    #[doc = "9: ADC Trigger Event User 4"]
    EventUser4 = 9,
    #[doc = "10: ADC Trigger Event User 5"]
    EventUser5 = 10,
    #[doc = "11: ADC Trigger Event User 6"]
    EventUser6 = 11,
    #[doc = "12: ADC Trigger Event User 7"]
    EventUser7 = 12,
    #[doc = "13: ADC Trigger Event User 8"]
    EventUser8 = 13,
    #[doc = "14: ADC Trigger Event User 9"]
    EventUser9 = 14,
    #[doc = "15: ADC Trigger Event User 10"]
    EventUser10 = 15,
}
impl From<Strgsrcselect> for u8 {
    #[inline(always)]
    fn from(variant: Strgsrcselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Strgsrcselect {
    type Ux = u8;
}
impl crate::IsEnum for Strgsrcselect {}
#[doc = "Field `STRGSRC` reader - SCAN trigger source selection"]
pub type StrgsrcR = crate::FieldReader<Strgsrcselect>;
impl StrgsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Strgsrcselect> {
        match self.bits {
            0 => Some(Strgsrcselect::NoTrigger),
            1 => Some(Strgsrcselect::GlobalSoftwareTrigger),
            2 => Some(Strgsrcselect::GlobalLevelTrigger),
            4 => Some(Strgsrcselect::SyncTrigger),
            5 => Some(Strgsrcselect::EventUser0),
            6 => Some(Strgsrcselect::EventUser1),
            7 => Some(Strgsrcselect::EventUser2),
            8 => Some(Strgsrcselect::EventUser3),
            9 => Some(Strgsrcselect::EventUser4),
            10 => Some(Strgsrcselect::EventUser5),
            11 => Some(Strgsrcselect::EventUser6),
            12 => Some(Strgsrcselect::EventUser7),
            13 => Some(Strgsrcselect::EventUser8),
            14 => Some(Strgsrcselect::EventUser9),
            15 => Some(Strgsrcselect::EventUser10),
            _ => None,
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Strgsrcselect::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Strgsrcselect::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Strgsrcselect::GlobalLevelTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Strgsrcselect::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Strgsrcselect::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Strgsrcselect::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Strgsrcselect::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Strgsrcselect::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Strgsrcselect::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Strgsrcselect::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Strgsrcselect::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Strgsrcselect::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Strgsrcselect::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Strgsrcselect::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Strgsrcselect::EventUser10
    }
}
#[doc = "Field `STRGSRC` writer - SCAN trigger source selection"]
pub type StrgsrcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Strgsrcselect>;
impl<'a, REG> StrgsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Strgsrcselect::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Strgsrcselect::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Strgsrcselect::GlobalLevelTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Strgsrcselect::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Strgsrcselect::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Strgsrcselect::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Strgsrcselect::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Strgsrcselect::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Strgsrcselect::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Strgsrcselect::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Strgsrcselect::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Strgsrcselect::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Strgsrcselect::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Strgsrcselect::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Strgsrcselect::EventUser10)
    }
}
#[doc = "Field `STRGLVL` reader - Scan Trigger Level Sensitivity"]
pub type StrglvlR = crate::BitReader;
#[doc = "Field `STRGLVL` writer - Scan Trigger Level Sensitivity"]
pub type StrglvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCNRTDS` reader - SCAN Re-trigger Disable"]
pub type ScnrtdsR = crate::BitReader;
#[doc = "Field `SCNRTDS` writer - SCAN Re-trigger Disable"]
pub type ScnrtdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDIV` reader - Division Ratio for SARCORE clock"]
pub type AdcdivR = crate::FieldReader;
#[doc = "Field `ADCDIV` writer - Division Ratio for SARCORE clock"]
pub type AdcdivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:9 - Sample Count"]
    #[inline(always)]
    pub fn samc(&self) -> SamcR {
        SamcR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:11 - Selects Resolution"]
    #[inline(always)]
    pub fn selres(&self) -> SelresR {
        SelresR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Early Interrupt Select"]
    #[inline(always)]
    pub fn eis(&self) -> EisR {
        EisR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Interrupt Type Select"]
    #[inline(always)]
    pub fn eirqovr(&self) -> EirqovrR {
        EirqovrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - SCAN trigger source selection"]
    #[inline(always)]
    pub fn strgsrc(&self) -> StrgsrcR {
        StrgsrcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Scan Trigger Level Sensitivity"]
    #[inline(always)]
    pub fn strglvl(&self) -> StrglvlR {
        StrglvlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SCAN Re-trigger Disable"]
    #[inline(always)]
    pub fn scnrtds(&self) -> ScnrtdsR {
        ScnrtdsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Division Ratio for SARCORE clock"]
    #[inline(always)]
    pub fn adcdiv(&self) -> AdcdivR {
        AdcdivR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sample Count"]
    #[inline(always)]
    pub fn samc(&mut self) -> SamcW<'_, CorctrlSpec> {
        SamcW::new(self, 0)
    }
    #[doc = "Bits 10:11 - Selects Resolution"]
    #[inline(always)]
    pub fn selres(&mut self) -> SelresW<'_, CorctrlSpec> {
        SelresW::new(self, 10)
    }
    #[doc = "Bits 12:14 - Early Interrupt Select"]
    #[inline(always)]
    pub fn eis(&mut self) -> EisW<'_, CorctrlSpec> {
        EisW::new(self, 12)
    }
    #[doc = "Bit 15 - Interrupt Type Select"]
    #[inline(always)]
    pub fn eirqovr(&mut self) -> EirqovrW<'_, CorctrlSpec> {
        EirqovrW::new(self, 15)
    }
    #[doc = "Bits 16:19 - SCAN trigger source selection"]
    #[inline(always)]
    pub fn strgsrc(&mut self) -> StrgsrcW<'_, CorctrlSpec> {
        StrgsrcW::new(self, 16)
    }
    #[doc = "Bit 21 - Scan Trigger Level Sensitivity"]
    #[inline(always)]
    pub fn strglvl(&mut self) -> StrglvlW<'_, CorctrlSpec> {
        StrglvlW::new(self, 21)
    }
    #[doc = "Bit 22 - SCAN Re-trigger Disable"]
    #[inline(always)]
    pub fn scnrtds(&mut self) -> ScnrtdsW<'_, CorctrlSpec> {
        ScnrtdsW::new(self, 22)
    }
    #[doc = "Bits 24:30 - Division Ratio for SARCORE clock"]
    #[inline(always)]
    pub fn adcdiv(&mut self) -> AdcdivW<'_, CorctrlSpec> {
        AdcdivW::new(self, 24)
    }
}
#[doc = "SARCORE Control\n\nYou can [`read`](crate::Reg::read) this register and get [`corctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`corctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CorctrlSpec;
impl crate::RegisterSpec for CorctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`corctrl::R`](R) reader structure"]
impl crate::Readable for CorctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`corctrl::W`](W) writer structure"]
impl crate::Writable for CorctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORCTRL to value 0x0c00"]
impl crate::Resettable for CorctrlSpec {
    const RESET_VALUE: u32 = 0x0c00;
}
