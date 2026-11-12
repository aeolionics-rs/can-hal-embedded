#[doc = "Register `CHNCFG5` reader"]
pub type R = crate::R<Chncfg5Spec>;
#[doc = "Register `CHNCFG5` writer"]
pub type W = crate::W<Chncfg5Spec>;
#[doc = "Channel 8 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc8select {
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
impl From<Trgsrc8select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc8select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc8select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc8select {}
#[doc = "Field `TRGSRC8` reader - Channel 8 Trigger Source"]
pub type Trgsrc8R = crate::FieldReader<Trgsrc8select>;
impl Trgsrc8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc8select {
        match self.bits {
            0 => Trgsrc8select::NoTrigger,
            1 => Trgsrc8select::GlobalSoftwareTrigger,
            2 => Trgsrc8select::GlobalLevelTrigger,
            3 => Trgsrc8select::ScanTrigger,
            4 => Trgsrc8select::SyncTrigger,
            5 => Trgsrc8select::EventUser0,
            6 => Trgsrc8select::EventUser1,
            7 => Trgsrc8select::EventUser2,
            8 => Trgsrc8select::EventUser3,
            9 => Trgsrc8select::EventUser4,
            10 => Trgsrc8select::EventUser5,
            11 => Trgsrc8select::EventUser6,
            12 => Trgsrc8select::EventUser7,
            13 => Trgsrc8select::EventUser8,
            14 => Trgsrc8select::EventUser9,
            15 => Trgsrc8select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc8select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc8select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc8select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc8select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc8select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc8select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc8select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc8select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc8select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc8select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc8select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc8select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc8select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc8select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc8select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc8select::EventUser10
    }
}
#[doc = "Field `TRGSRC8` writer - Channel 8 Trigger Source"]
pub type Trgsrc8W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc8select, crate::Safe>;
impl<'a, REG> Trgsrc8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc8select::EventUser10)
    }
}
#[doc = "Channel 9 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc9select {
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
impl From<Trgsrc9select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc9select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc9select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc9select {}
#[doc = "Field `TRGSRC9` reader - Channel 9 Trigger Source"]
pub type Trgsrc9R = crate::FieldReader<Trgsrc9select>;
impl Trgsrc9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc9select {
        match self.bits {
            0 => Trgsrc9select::NoTrigger,
            1 => Trgsrc9select::GlobalSoftwareTrigger,
            2 => Trgsrc9select::GlobalLevelTrigger,
            3 => Trgsrc9select::ScanTrigger,
            4 => Trgsrc9select::SyncTrigger,
            5 => Trgsrc9select::EventUser0,
            6 => Trgsrc9select::EventUser1,
            7 => Trgsrc9select::EventUser2,
            8 => Trgsrc9select::EventUser3,
            9 => Trgsrc9select::EventUser4,
            10 => Trgsrc9select::EventUser5,
            11 => Trgsrc9select::EventUser6,
            12 => Trgsrc9select::EventUser7,
            13 => Trgsrc9select::EventUser8,
            14 => Trgsrc9select::EventUser9,
            15 => Trgsrc9select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc9select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc9select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc9select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc9select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc9select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc9select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc9select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc9select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc9select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc9select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc9select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc9select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc9select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc9select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc9select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc9select::EventUser10
    }
}
#[doc = "Field `TRGSRC9` writer - Channel 9 Trigger Source"]
pub type Trgsrc9W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc9select, crate::Safe>;
impl<'a, REG> Trgsrc9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc9select::EventUser10)
    }
}
#[doc = "Channel 10 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc10select {
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
impl From<Trgsrc10select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc10select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc10select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc10select {}
#[doc = "Field `TRGSRC10` reader - Channel 10 Trigger Source"]
pub type Trgsrc10R = crate::FieldReader<Trgsrc10select>;
impl Trgsrc10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc10select {
        match self.bits {
            0 => Trgsrc10select::NoTrigger,
            1 => Trgsrc10select::GlobalSoftwareTrigger,
            2 => Trgsrc10select::GlobalLevelTrigger,
            3 => Trgsrc10select::ScanTrigger,
            4 => Trgsrc10select::SyncTrigger,
            5 => Trgsrc10select::EventUser0,
            6 => Trgsrc10select::EventUser1,
            7 => Trgsrc10select::EventUser2,
            8 => Trgsrc10select::EventUser3,
            9 => Trgsrc10select::EventUser4,
            10 => Trgsrc10select::EventUser5,
            11 => Trgsrc10select::EventUser6,
            12 => Trgsrc10select::EventUser7,
            13 => Trgsrc10select::EventUser8,
            14 => Trgsrc10select::EventUser9,
            15 => Trgsrc10select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc10select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc10select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc10select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc10select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc10select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc10select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc10select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc10select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc10select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc10select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc10select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc10select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc10select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc10select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc10select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc10select::EventUser10
    }
}
#[doc = "Field `TRGSRC10` writer - Channel 10 Trigger Source"]
pub type Trgsrc10W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc10select, crate::Safe>;
impl<'a, REG> Trgsrc10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc10select::EventUser10)
    }
}
#[doc = "Channel 11 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc11select {
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
impl From<Trgsrc11select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc11select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc11select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc11select {}
#[doc = "Field `TRGSRC11` reader - Channel 11 Trigger Source"]
pub type Trgsrc11R = crate::FieldReader<Trgsrc11select>;
impl Trgsrc11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc11select {
        match self.bits {
            0 => Trgsrc11select::NoTrigger,
            1 => Trgsrc11select::GlobalSoftwareTrigger,
            2 => Trgsrc11select::GlobalLevelTrigger,
            3 => Trgsrc11select::ScanTrigger,
            4 => Trgsrc11select::SyncTrigger,
            5 => Trgsrc11select::EventUser0,
            6 => Trgsrc11select::EventUser1,
            7 => Trgsrc11select::EventUser2,
            8 => Trgsrc11select::EventUser3,
            9 => Trgsrc11select::EventUser4,
            10 => Trgsrc11select::EventUser5,
            11 => Trgsrc11select::EventUser6,
            12 => Trgsrc11select::EventUser7,
            13 => Trgsrc11select::EventUser8,
            14 => Trgsrc11select::EventUser9,
            15 => Trgsrc11select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc11select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc11select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc11select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc11select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc11select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc11select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc11select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc11select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc11select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc11select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc11select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc11select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc11select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc11select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc11select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc11select::EventUser10
    }
}
#[doc = "Field `TRGSRC11` writer - Channel 11 Trigger Source"]
pub type Trgsrc11W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc11select, crate::Safe>;
impl<'a, REG> Trgsrc11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc11select::EventUser10)
    }
}
#[doc = "Channel 12 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc12select {
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
impl From<Trgsrc12select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc12select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc12select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc12select {}
#[doc = "Field `TRGSRC12` reader - Channel 12 Trigger Source"]
pub type Trgsrc12R = crate::FieldReader<Trgsrc12select>;
impl Trgsrc12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc12select {
        match self.bits {
            0 => Trgsrc12select::NoTrigger,
            1 => Trgsrc12select::GlobalSoftwareTrigger,
            2 => Trgsrc12select::GlobalLevelTrigger,
            3 => Trgsrc12select::ScanTrigger,
            4 => Trgsrc12select::SyncTrigger,
            5 => Trgsrc12select::EventUser0,
            6 => Trgsrc12select::EventUser1,
            7 => Trgsrc12select::EventUser2,
            8 => Trgsrc12select::EventUser3,
            9 => Trgsrc12select::EventUser4,
            10 => Trgsrc12select::EventUser5,
            11 => Trgsrc12select::EventUser6,
            12 => Trgsrc12select::EventUser7,
            13 => Trgsrc12select::EventUser8,
            14 => Trgsrc12select::EventUser9,
            15 => Trgsrc12select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc12select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc12select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc12select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc12select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc12select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc12select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc12select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc12select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc12select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc12select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc12select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc12select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc12select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc12select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc12select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc12select::EventUser10
    }
}
#[doc = "Field `TRGSRC12` writer - Channel 12 Trigger Source"]
pub type Trgsrc12W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc12select, crate::Safe>;
impl<'a, REG> Trgsrc12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc12select::EventUser10)
    }
}
#[doc = "Channel 13 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc13select {
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
impl From<Trgsrc13select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc13select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc13select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc13select {}
#[doc = "Field `TRGSRC13` reader - Channel 13 Trigger Source"]
pub type Trgsrc13R = crate::FieldReader<Trgsrc13select>;
impl Trgsrc13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc13select {
        match self.bits {
            0 => Trgsrc13select::NoTrigger,
            1 => Trgsrc13select::GlobalSoftwareTrigger,
            2 => Trgsrc13select::GlobalLevelTrigger,
            3 => Trgsrc13select::ScanTrigger,
            4 => Trgsrc13select::SyncTrigger,
            5 => Trgsrc13select::EventUser0,
            6 => Trgsrc13select::EventUser1,
            7 => Trgsrc13select::EventUser2,
            8 => Trgsrc13select::EventUser3,
            9 => Trgsrc13select::EventUser4,
            10 => Trgsrc13select::EventUser5,
            11 => Trgsrc13select::EventUser6,
            12 => Trgsrc13select::EventUser7,
            13 => Trgsrc13select::EventUser8,
            14 => Trgsrc13select::EventUser9,
            15 => Trgsrc13select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc13select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc13select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc13select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc13select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc13select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc13select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc13select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc13select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc13select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc13select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc13select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc13select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc13select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc13select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc13select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc13select::EventUser10
    }
}
#[doc = "Field `TRGSRC13` writer - Channel 13 Trigger Source"]
pub type Trgsrc13W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc13select, crate::Safe>;
impl<'a, REG> Trgsrc13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc13select::EventUser10)
    }
}
#[doc = "Channel 14 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc14select {
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
impl From<Trgsrc14select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc14select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc14select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc14select {}
#[doc = "Field `TRGSRC14` reader - Channel 14 Trigger Source"]
pub type Trgsrc14R = crate::FieldReader<Trgsrc14select>;
impl Trgsrc14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc14select {
        match self.bits {
            0 => Trgsrc14select::NoTrigger,
            1 => Trgsrc14select::GlobalSoftwareTrigger,
            2 => Trgsrc14select::GlobalLevelTrigger,
            3 => Trgsrc14select::ScanTrigger,
            4 => Trgsrc14select::SyncTrigger,
            5 => Trgsrc14select::EventUser0,
            6 => Trgsrc14select::EventUser1,
            7 => Trgsrc14select::EventUser2,
            8 => Trgsrc14select::EventUser3,
            9 => Trgsrc14select::EventUser4,
            10 => Trgsrc14select::EventUser5,
            11 => Trgsrc14select::EventUser6,
            12 => Trgsrc14select::EventUser7,
            13 => Trgsrc14select::EventUser8,
            14 => Trgsrc14select::EventUser9,
            15 => Trgsrc14select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc14select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc14select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc14select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc14select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc14select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc14select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc14select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc14select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc14select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc14select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc14select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc14select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc14select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc14select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc14select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc14select::EventUser10
    }
}
#[doc = "Field `TRGSRC14` writer - Channel 14 Trigger Source"]
pub type Trgsrc14W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc14select, crate::Safe>;
impl<'a, REG> Trgsrc14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc14select::EventUser10)
    }
}
#[doc = "Channel 15 Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsrc15select {
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
impl From<Trgsrc15select> for u8 {
    #[inline(always)]
    fn from(variant: Trgsrc15select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsrc15select {
    type Ux = u8;
}
impl crate::IsEnum for Trgsrc15select {}
#[doc = "Field `TRGSRC15` reader - Channel 15 Trigger Source"]
pub type Trgsrc15R = crate::FieldReader<Trgsrc15select>;
impl Trgsrc15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc15select {
        match self.bits {
            0 => Trgsrc15select::NoTrigger,
            1 => Trgsrc15select::GlobalSoftwareTrigger,
            2 => Trgsrc15select::GlobalLevelTrigger,
            3 => Trgsrc15select::ScanTrigger,
            4 => Trgsrc15select::SyncTrigger,
            5 => Trgsrc15select::EventUser0,
            6 => Trgsrc15select::EventUser1,
            7 => Trgsrc15select::EventUser2,
            8 => Trgsrc15select::EventUser3,
            9 => Trgsrc15select::EventUser4,
            10 => Trgsrc15select::EventUser5,
            11 => Trgsrc15select::EventUser6,
            12 => Trgsrc15select::EventUser7,
            13 => Trgsrc15select::EventUser8,
            14 => Trgsrc15select::EventUser9,
            15 => Trgsrc15select::EventUser10,
            _ => unreachable!(),
        }
    }
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == Trgsrc15select::NoTrigger
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn is_global_software_trigger(&self) -> bool {
        *self == Trgsrc15select::GlobalSoftwareTrigger
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn is_global_level_trigger(&self) -> bool {
        *self == Trgsrc15select::GlobalLevelTrigger
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn is_scan_trigger(&self) -> bool {
        *self == Trgsrc15select::ScanTrigger
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn is_sync_trigger(&self) -> bool {
        *self == Trgsrc15select::SyncTrigger
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn is_event_user0(&self) -> bool {
        *self == Trgsrc15select::EventUser0
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn is_event_user1(&self) -> bool {
        *self == Trgsrc15select::EventUser1
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn is_event_user2(&self) -> bool {
        *self == Trgsrc15select::EventUser2
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn is_event_user3(&self) -> bool {
        *self == Trgsrc15select::EventUser3
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn is_event_user4(&self) -> bool {
        *self == Trgsrc15select::EventUser4
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn is_event_user5(&self) -> bool {
        *self == Trgsrc15select::EventUser5
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn is_event_user6(&self) -> bool {
        *self == Trgsrc15select::EventUser6
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn is_event_user7(&self) -> bool {
        *self == Trgsrc15select::EventUser7
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn is_event_user8(&self) -> bool {
        *self == Trgsrc15select::EventUser8
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn is_event_user9(&self) -> bool {
        *self == Trgsrc15select::EventUser9
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn is_event_user10(&self) -> bool {
        *self == Trgsrc15select::EventUser10
    }
}
#[doc = "Field `TRGSRC15` writer - Channel 15 Trigger Source"]
pub type Trgsrc15W<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsrc15select, crate::Safe>;
impl<'a, REG> Trgsrc15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Trigger (NOP)"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::NoTrigger)
    }
    #[doc = "Global Software Trigger"]
    #[inline(always)]
    pub fn global_software_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::GlobalSoftwareTrigger)
    }
    #[doc = "Global Level Software Trigger"]
    #[inline(always)]
    pub fn global_level_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::GlobalLevelTrigger)
    }
    #[doc = "SCANTRG - Scan Trigger"]
    #[inline(always)]
    pub fn scan_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::ScanTrigger)
    }
    #[doc = "STRIG Synchronous Trigger"]
    #[inline(always)]
    pub fn sync_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::SyncTrigger)
    }
    #[doc = "ADC Trigger Event User 0"]
    #[inline(always)]
    pub fn event_user0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::EventUser0)
    }
    #[doc = "ADC Trigger Event User 1"]
    #[inline(always)]
    pub fn event_user1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::EventUser1)
    }
    #[doc = "ADC Trigger Event User 2"]
    #[inline(always)]
    pub fn event_user2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::EventUser2)
    }
    #[doc = "ADC Trigger Event User 3"]
    #[inline(always)]
    pub fn event_user3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::EventUser3)
    }
    #[doc = "ADC Trigger Event User 4"]
    #[inline(always)]
    pub fn event_user4(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::EventUser4)
    }
    #[doc = "ADC Trigger Event User 5"]
    #[inline(always)]
    pub fn event_user5(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::EventUser5)
    }
    #[doc = "ADC Trigger Event User 6"]
    #[inline(always)]
    pub fn event_user6(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::EventUser6)
    }
    #[doc = "ADC Trigger Event User 7"]
    #[inline(always)]
    pub fn event_user7(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::EventUser7)
    }
    #[doc = "ADC Trigger Event User 8"]
    #[inline(always)]
    pub fn event_user8(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::EventUser8)
    }
    #[doc = "ADC Trigger Event User 9"]
    #[inline(always)]
    pub fn event_user9(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::EventUser9)
    }
    #[doc = "ADC Trigger Event User 10"]
    #[inline(always)]
    pub fn event_user10(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc15select::EventUser10)
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel 8 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc8(&self) -> Trgsrc8R {
        Trgsrc8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 9 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc9(&self) -> Trgsrc9R {
        Trgsrc9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Channel 10 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc10(&self) -> Trgsrc10R {
        Trgsrc10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Channel 11 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc11(&self) -> Trgsrc11R {
        Trgsrc11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Channel 12 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc12(&self) -> Trgsrc12R {
        Trgsrc12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Channel 13 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc13(&self) -> Trgsrc13R {
        Trgsrc13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Channel 14 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc14(&self) -> Trgsrc14R {
        Trgsrc14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Channel 15 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc15(&self) -> Trgsrc15R {
        Trgsrc15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel 8 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc8(&mut self) -> Trgsrc8W<'_, Chncfg5Spec> {
        Trgsrc8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Channel 9 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc9(&mut self) -> Trgsrc9W<'_, Chncfg5Spec> {
        Trgsrc9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Channel 10 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc10(&mut self) -> Trgsrc10W<'_, Chncfg5Spec> {
        Trgsrc10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Channel 11 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc11(&mut self) -> Trgsrc11W<'_, Chncfg5Spec> {
        Trgsrc11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Channel 12 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc12(&mut self) -> Trgsrc12W<'_, Chncfg5Spec> {
        Trgsrc12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Channel 13 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc13(&mut self) -> Trgsrc13W<'_, Chncfg5Spec> {
        Trgsrc13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Channel 14 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc14(&mut self) -> Trgsrc14W<'_, Chncfg5Spec> {
        Trgsrc14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Channel 15 Trigger Source"]
    #[inline(always)]
    pub fn trgsrc15(&mut self) -> Trgsrc15W<'_, Chncfg5Spec> {
        Trgsrc15W::new(self, 28)
    }
}
#[doc = "Channel Configuration 5 (TRGSRC)\n\nYou can [`read`](crate::Reg::read) this register and get [`chncfg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chncfg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chncfg5Spec;
impl crate::RegisterSpec for Chncfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chncfg5::R`](R) reader structure"]
impl crate::Readable for Chncfg5Spec {}
#[doc = "`write(|w| ..)` method takes [`chncfg5::W`](W) writer structure"]
impl crate::Writable for Chncfg5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNCFG5 to value 0"]
impl crate::Resettable for Chncfg5Spec {}
