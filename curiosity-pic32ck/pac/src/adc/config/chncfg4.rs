#[doc = "Register `CHNCFG4` reader"]
pub type R = crate::R<Chncfg4Spec>;
#[doc = "Register `CHNCFG4` writer"]
pub type W = crate::W<Chncfg4Spec>;
#[doc = "Channel 0 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc0select {
    #[doc = "0: No Trigger (NOP)"]
    NoTrigger = 0,
    #[doc = "1: Global Software Trigger"]
    GlobalSoftwareTrigger = 1,
    #[doc = "2: Global Level Software Trigger"]
    GlobalLevelTrigger = 2,
    #[doc = "3: SCANTRG - Scan Trigger"]
    ScanTrigger = 3,
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
impl From<Trgsrc0select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc0select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc0select {}
#[doc = "Field `TRGSRC0` reader - Channel 0 Trigger Source"]
pub type Trgsrc0R = crate::FieldReader<Trgsrc0select>;
impl Trgsrc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc0select {
        match self.bits {
            0 => Trgsrc0select::NoTrigger,
            1 => Trgsrc0select::GlobalSoftwareTrigger,
            2 => Trgsrc0select::GlobalLevelTrigger,
            3 => Trgsrc0select::ScanTrigger,
            4 => Trgsrc0select::SyncTrigger,
            5 => Trgsrc0select::EventUser0,
            6 => Trgsrc0select::EventUser1,
            7 => Trgsrc0select::EventUser2,
            8 => Trgsrc0select::EventUser3,
            9 => Trgsrc0select::EventUser4,
            10 => Trgsrc0select::EventUser5,
            11 => Trgsrc0select::EventUser6,
            12 => Trgsrc0select::EventUser7,
            13 => Trgsrc0select::EventUser8,
            14 => Trgsrc0select::EventUser9,
            15 => Trgsrc0select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc0select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc0select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc0select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc0select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc0select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc0select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc0select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc0select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc0select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc0select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc0select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc0select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc0select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc0select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc0select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc0select::EventUser10
    }
}
#[doc = "Field `TRGSRC0` writer - Channel 0 Trigger Source"]
pub type Trgsrc0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc0select, crate::Safe>;
impl<'a, REG> Trgsrc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc0select::EventUser10)
    }
}
#[doc = "Channel 1 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc1select {
    #[doc = "0: No Trigger (NOP)"]
    NoTrigger = 0,
    #[doc = "1: Global Software Trigger"]
    GlobalSoftwareTrigger = 1,
    #[doc = "2: Global Level Software Trigger"]
    GlobalLevelTrigger = 2,
    #[doc = "3: SCANTRG - Scan Trigger"]
    ScanTrigger = 3,
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
impl From<Trgsrc1select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc1select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc1select {}
#[doc = "Field `TRGSRC1` reader - Channel 1 Trigger Source"]
pub type Trgsrc1R = crate::FieldReader<Trgsrc1select>;
impl Trgsrc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc1select {
        match self.bits {
            0 => Trgsrc1select::NoTrigger,
            1 => Trgsrc1select::GlobalSoftwareTrigger,
            2 => Trgsrc1select::GlobalLevelTrigger,
            3 => Trgsrc1select::ScanTrigger,
            4 => Trgsrc1select::SyncTrigger,
            5 => Trgsrc1select::EventUser0,
            6 => Trgsrc1select::EventUser1,
            7 => Trgsrc1select::EventUser2,
            8 => Trgsrc1select::EventUser3,
            9 => Trgsrc1select::EventUser4,
            10 => Trgsrc1select::EventUser5,
            11 => Trgsrc1select::EventUser6,
            12 => Trgsrc1select::EventUser7,
            13 => Trgsrc1select::EventUser8,
            14 => Trgsrc1select::EventUser9,
            15 => Trgsrc1select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc1select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc1select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc1select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc1select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc1select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc1select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc1select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc1select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc1select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc1select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc1select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc1select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc1select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc1select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc1select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc1select::EventUser10
    }
}
#[doc = "Field `TRGSRC1` writer - Channel 1 Trigger Source"]
pub type Trgsrc1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc1select, crate::Safe>;
impl<'a, REG> Trgsrc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc1select::EventUser10)
    }
}
#[doc = "Channel 2 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc2select {
    #[doc = "0: No Trigger (NOP)"]
    NoTrigger = 0,
    #[doc = "1: Global Software Trigger"]
    GlobalSoftwareTrigger = 1,
    #[doc = "2: Global Level Software Trigger"]
    GlobalLevelTrigger = 2,
    #[doc = "3: SCANTRG - Scan Trigger"]
    ScanTrigger = 3,
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
impl From<Trgsrc2select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc2select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc2select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc2select {}
#[doc = "Field `TRGSRC2` reader - Channel 2 Trigger Source"]
pub type Trgsrc2R = crate::FieldReader<Trgsrc2select>;
impl Trgsrc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc2select {
        match self.bits {
            0 => Trgsrc2select::NoTrigger,
            1 => Trgsrc2select::GlobalSoftwareTrigger,
            2 => Trgsrc2select::GlobalLevelTrigger,
            3 => Trgsrc2select::ScanTrigger,
            4 => Trgsrc2select::SyncTrigger,
            5 => Trgsrc2select::EventUser0,
            6 => Trgsrc2select::EventUser1,
            7 => Trgsrc2select::EventUser2,
            8 => Trgsrc2select::EventUser3,
            9 => Trgsrc2select::EventUser4,
            10 => Trgsrc2select::EventUser5,
            11 => Trgsrc2select::EventUser6,
            12 => Trgsrc2select::EventUser7,
            13 => Trgsrc2select::EventUser8,
            14 => Trgsrc2select::EventUser9,
            15 => Trgsrc2select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc2select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc2select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc2select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc2select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc2select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc2select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc2select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc2select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc2select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc2select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc2select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc2select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc2select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc2select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc2select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc2select::EventUser10
    }
}
#[doc = "Field `TRGSRC2` writer - Channel 2 Trigger Source"]
pub type Trgsrc2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc2select, crate::Safe>;
impl<'a, REG> Trgsrc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc2select::EventUser10)
    }
}
#[doc = "Channel 3 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc3select {
    #[doc = "0: No Trigger (NOP)"]
    NoTrigger = 0,
    #[doc = "1: Global Software Trigger"]
    GlobalSoftwareTrigger = 1,
    #[doc = "2: Global Level Software Trigger"]
    GlobalLevelTrigger = 2,
    #[doc = "3: SCANTRG - Scan Trigger"]
    ScanTrigger = 3,
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
impl From<Trgsrc3select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc3select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc3select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc3select {}
#[doc = "Field `TRGSRC3` reader - Channel 3 Trigger Source"]
pub type Trgsrc3R = crate::FieldReader<Trgsrc3select>;
impl Trgsrc3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc3select {
        match self.bits {
            0 => Trgsrc3select::NoTrigger,
            1 => Trgsrc3select::GlobalSoftwareTrigger,
            2 => Trgsrc3select::GlobalLevelTrigger,
            3 => Trgsrc3select::ScanTrigger,
            4 => Trgsrc3select::SyncTrigger,
            5 => Trgsrc3select::EventUser0,
            6 => Trgsrc3select::EventUser1,
            7 => Trgsrc3select::EventUser2,
            8 => Trgsrc3select::EventUser3,
            9 => Trgsrc3select::EventUser4,
            10 => Trgsrc3select::EventUser5,
            11 => Trgsrc3select::EventUser6,
            12 => Trgsrc3select::EventUser7,
            13 => Trgsrc3select::EventUser8,
            14 => Trgsrc3select::EventUser9,
            15 => Trgsrc3select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc3select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc3select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc3select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc3select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc3select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc3select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc3select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc3select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc3select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc3select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc3select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc3select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc3select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc3select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc3select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc3select::EventUser10
    }
}
#[doc = "Field `TRGSRC3` writer - Channel 3 Trigger Source"]
pub type Trgsrc3W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc3select, crate::Safe>;
impl<'a, REG> Trgsrc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc3select::EventUser10)
    }
}
#[doc = "Channel 4 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc4select {
    #[doc = "0: No Trigger (NOP)"]
    NoTrigger = 0,
    #[doc = "1: Global Software Trigger"]
    GlobalSoftwareTrigger = 1,
    #[doc = "2: Global Level Software Trigger"]
    GlobalLevelTrigger = 2,
    #[doc = "3: SCANTRG - Scan Trigger"]
    ScanTrigger = 3,
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
impl From<Trgsrc4select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc4select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc4select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc4select {}
#[doc = "Field `TRGSRC4` reader - Channel 4 Trigger Source"]
pub type Trgsrc4R = crate::FieldReader<Trgsrc4select>;
impl Trgsrc4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc4select {
        match self.bits {
            0 => Trgsrc4select::NoTrigger,
            1 => Trgsrc4select::GlobalSoftwareTrigger,
            2 => Trgsrc4select::GlobalLevelTrigger,
            3 => Trgsrc4select::ScanTrigger,
            4 => Trgsrc4select::SyncTrigger,
            5 => Trgsrc4select::EventUser0,
            6 => Trgsrc4select::EventUser1,
            7 => Trgsrc4select::EventUser2,
            8 => Trgsrc4select::EventUser3,
            9 => Trgsrc4select::EventUser4,
            10 => Trgsrc4select::EventUser5,
            11 => Trgsrc4select::EventUser6,
            12 => Trgsrc4select::EventUser7,
            13 => Trgsrc4select::EventUser8,
            14 => Trgsrc4select::EventUser9,
            15 => Trgsrc4select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc4select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc4select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc4select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc4select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc4select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc4select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc4select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc4select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc4select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc4select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc4select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc4select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc4select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc4select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc4select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc4select::EventUser10
    }
}
#[doc = "Field `TRGSRC4` writer - Channel 4 Trigger Source"]
pub type Trgsrc4W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc4select, crate::Safe>;
impl<'a, REG> Trgsrc4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc4select::EventUser10)
    }
}
#[doc = "Channel 5 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc5select {
    #[doc = "0: No Trigger (NOP)"]
    NoTrigger = 0,
    #[doc = "1: Global Software Trigger"]
    GlobalSoftwareTrigger = 1,
    #[doc = "2: Global Level Software Trigger"]
    GlobalLevelTrigger = 2,
    #[doc = "3: SCANTRG - Scan Trigger"]
    ScanTrigger = 3,
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
impl From<Trgsrc5select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc5select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc5select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc5select {}
#[doc = "Field `TRGSRC5` reader - Channel 5 Trigger Source"]
pub type Trgsrc5R = crate::FieldReader<Trgsrc5select>;
impl Trgsrc5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc5select {
        match self.bits {
            0 => Trgsrc5select::NoTrigger,
            1 => Trgsrc5select::GlobalSoftwareTrigger,
            2 => Trgsrc5select::GlobalLevelTrigger,
            3 => Trgsrc5select::ScanTrigger,
            4 => Trgsrc5select::SyncTrigger,
            5 => Trgsrc5select::EventUser0,
            6 => Trgsrc5select::EventUser1,
            7 => Trgsrc5select::EventUser2,
            8 => Trgsrc5select::EventUser3,
            9 => Trgsrc5select::EventUser4,
            10 => Trgsrc5select::EventUser5,
            11 => Trgsrc5select::EventUser6,
            12 => Trgsrc5select::EventUser7,
            13 => Trgsrc5select::EventUser8,
            14 => Trgsrc5select::EventUser9,
            15 => Trgsrc5select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc5select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc5select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc5select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc5select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc5select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc5select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc5select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc5select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc5select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc5select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc5select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc5select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc5select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc5select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc5select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc5select::EventUser10
    }
}
#[doc = "Field `TRGSRC5` writer - Channel 5 Trigger Source"]
pub type Trgsrc5W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc5select, crate::Safe>;
impl<'a, REG> Trgsrc5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc5select::EventUser10)
    }
}
#[doc = "Channel 6 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc6select {
    #[doc = "0: No Trigger (NOP)"]
    NoTrigger = 0,
    #[doc = "1: Global Software Trigger"]
    GlobalSoftwareTrigger = 1,
    #[doc = "2: Global Level Software Trigger"]
    GlobalLevelTrigger = 2,
    #[doc = "3: SCANTRG - Scan Trigger"]
    ScanTrigger = 3,
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
impl From<Trgsrc6select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc6select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc6select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc6select {}
#[doc = "Field `TRGSRC6` reader - Channel 6 Trigger Source"]
pub type Trgsrc6R = crate::FieldReader<Trgsrc6select>;
impl Trgsrc6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc6select {
        match self.bits {
            0 => Trgsrc6select::NoTrigger,
            1 => Trgsrc6select::GlobalSoftwareTrigger,
            2 => Trgsrc6select::GlobalLevelTrigger,
            3 => Trgsrc6select::ScanTrigger,
            4 => Trgsrc6select::SyncTrigger,
            5 => Trgsrc6select::EventUser0,
            6 => Trgsrc6select::EventUser1,
            7 => Trgsrc6select::EventUser2,
            8 => Trgsrc6select::EventUser3,
            9 => Trgsrc6select::EventUser4,
            10 => Trgsrc6select::EventUser5,
            11 => Trgsrc6select::EventUser6,
            12 => Trgsrc6select::EventUser7,
            13 => Trgsrc6select::EventUser8,
            14 => Trgsrc6select::EventUser9,
            15 => Trgsrc6select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc6select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc6select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc6select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc6select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc6select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc6select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc6select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc6select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc6select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc6select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc6select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc6select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc6select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc6select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc6select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc6select::EventUser10
    }
}
#[doc = "Field `TRGSRC6` writer - Channel 6 Trigger Source"]
pub type Trgsrc6W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc6select, crate::Safe>;
impl<'a, REG> Trgsrc6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc6select::EventUser10)
    }
}
#[doc = "Channel 7 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc7select {
    #[doc = "0: No Trigger (NOP)"]
    NoTrigger = 0,
    #[doc = "1: Global Software Trigger"]
    GlobalSoftwareTrigger = 1,
    #[doc = "2: Global Level Software Trigger"]
    GlobalLevelTrigger = 2,
    #[doc = "3: SCANTRG - Scan Trigger"]
    ScanTrigger = 3,
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
impl From<Trgsrc7select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc7select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc7select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc7select {}
#[doc = "Field `TRGSRC7` reader - Channel 7 Trigger Source"]
pub type Trgsrc7R = crate::FieldReader<Trgsrc7select>;
impl Trgsrc7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc7select {
        match self.bits {
            0 => Trgsrc7select::NoTrigger,
            1 => Trgsrc7select::GlobalSoftwareTrigger,
            2 => Trgsrc7select::GlobalLevelTrigger,
            3 => Trgsrc7select::ScanTrigger,
            4 => Trgsrc7select::SyncTrigger,
            5 => Trgsrc7select::EventUser0,
            6 => Trgsrc7select::EventUser1,
            7 => Trgsrc7select::EventUser2,
            8 => Trgsrc7select::EventUser3,
            9 => Trgsrc7select::EventUser4,
            10 => Trgsrc7select::EventUser5,
            11 => Trgsrc7select::EventUser6,
            12 => Trgsrc7select::EventUser7,
            13 => Trgsrc7select::EventUser8,
            14 => Trgsrc7select::EventUser9,
            15 => Trgsrc7select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc7select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc7select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc7select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc7select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc7select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc7select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc7select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc7select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc7select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc7select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc7select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc7select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc7select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc7select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc7select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc7select::EventUser10
    }
}
#[doc = "Field `TRGSRC7` writer - Channel 7 Trigger Source"]
pub type Trgsrc7W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc7select, crate::Safe>;
impl<'a, REG> Trgsrc7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc7select::EventUser10)
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel 0 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc0(&self) -> Trgsrc0R {
        Trgsrc0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 1 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc1(&self) -> Trgsrc1R {
        Trgsrc1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Channel 2 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc2(&self) -> Trgsrc2R {
        Trgsrc2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Channel 3 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc3(&self) -> Trgsrc3R {
        Trgsrc3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Channel 4 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc4(&self) -> Trgsrc4R {
        Trgsrc4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Channel 5 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc5(&self) -> Trgsrc5R {
        Trgsrc5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Channel 6 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc6(&self) -> Trgsrc6R {
        Trgsrc6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Channel 7 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc7(&self) -> Trgsrc7R {
        Trgsrc7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel 0 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc0(&mut self) -> Trgsrc0W<'_, Chncfg4Spec> {
        Trgsrc0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Channel 1 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc1(&mut self) -> Trgsrc1W<'_, Chncfg4Spec> {
        Trgsrc1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Channel 2 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc2(&mut self) -> Trgsrc2W<'_, Chncfg4Spec> {
        Trgsrc2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Channel 3 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc3(&mut self) -> Trgsrc3W<'_, Chncfg4Spec> {
        Trgsrc3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Channel 4 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc4(&mut self) -> Trgsrc4W<'_, Chncfg4Spec> {
        Trgsrc4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Channel 5 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc5(&mut self) -> Trgsrc5W<'_, Chncfg4Spec> {
        Trgsrc5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Channel 6 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc6(&mut self) -> Trgsrc6W<'_, Chncfg4Spec> {
        Trgsrc6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Channel 7 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc7(&mut self) -> Trgsrc7W<'_, Chncfg4Spec> {
        Trgsrc7W::new(self, 28)
    }
}
#[doc = "Channel Configuration 4 (TRGSRC)\n\nYou can [`read`](crate::Reg::read) this register and get [`chncfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chncfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chncfg4Spec;
impl crate::RegisterSpec for Chncfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chncfg4::R`](R) reader structure"]
impl crate::Readable for Chncfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`chncfg4::W`](W) writer structure"]
impl crate::Writable for Chncfg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNCFG4 to value 0"]
impl crate::Resettable for Chncfg4Spec {}
