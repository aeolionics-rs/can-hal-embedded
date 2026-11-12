#[doc = "Register `STATUSC` reader"]
pub type R = crate::R<StatuscSpec>;
#[doc = "Core State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stateselect {
    #[doc = "0: State Machine Ready"]
    Idle = 0,
    #[doc = "1: CRC32 Operation Ongoing"]
    Crc32 = 1,
    #[doc = "4: Memory Set"]
    Mset = 4,
    #[doc = "8: MBIST fill memory with zeroes"]
    MbistFill = 8,
    #[doc = "9: SET1 Phase: read '0' write '1'"]
    MbistSet1 = 9,
    #[doc = "10: SET2 Phase: read '1' write '0'"]
    MbistSet2 = 10,
    #[doc = "11: SET2B Phase: read '0' write '1'"]
    MbistSet2b = 11,
    #[doc = "12: CLEAR1 Phase: read '1' write '0'"]
    MbistClear1 = 12,
    #[doc = "13: CLEAR2 Phase: read '0' write '1'"]
    MbistClear2 = 13,
    #[doc = "14: CLEAR2B Phase: read '1' write '0'"]
    MbistClear2b = 14,
    #[doc = "15: READ Phase: check memory is cleared"]
    MbistRead = 15,
}
impl From<Stateselect> for u8 {
    #[inline(always)]
    fn from(variant: Stateselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stateselect {
    type Ux = u8;
}
impl crate::IsEnum for Stateselect {}
#[doc = "Field `STATE` reader - Core State"]
pub type StateR = crate::FieldReader<Stateselect>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stateselect> {
        match self.bits {
            0 => Some(Stateselect::Idle),
            1 => Some(Stateselect::Crc32),
            4 => Some(Stateselect::Mset),
            8 => Some(Stateselect::MbistFill),
            9 => Some(Stateselect::MbistSet1),
            10 => Some(Stateselect::MbistSet2),
            11 => Some(Stateselect::MbistSet2b),
            12 => Some(Stateselect::MbistClear1),
            13 => Some(Stateselect::MbistClear2),
            14 => Some(Stateselect::MbistClear2b),
            15 => Some(Stateselect::MbistRead),
            _ => None,
        }
    }
    #[doc = "State Machine Ready"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Stateselect::Idle
    }
    #[doc = "CRC32 Operation Ongoing"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == Stateselect::Crc32
    }
    #[doc = "Memory Set"]
    #[inline(always)]
    pub fn is_mset(&self) -> bool {
        *self == Stateselect::Mset
    }
    #[doc = "MBIST fill memory with zeroes"]
    #[inline(always)]
    pub fn is_mbist_fill(&self) -> bool {
        *self == Stateselect::MbistFill
    }
    #[doc = "SET1 Phase: read '0' write '1'"]
    #[inline(always)]
    pub fn is_mbist_set1(&self) -> bool {
        *self == Stateselect::MbistSet1
    }
    #[doc = "SET2 Phase: read '1' write '0'"]
    #[inline(always)]
    pub fn is_mbist_set2(&self) -> bool {
        *self == Stateselect::MbistSet2
    }
    #[doc = "SET2B Phase: read '0' write '1'"]
    #[inline(always)]
    pub fn is_mbist_set2b(&self) -> bool {
        *self == Stateselect::MbistSet2b
    }
    #[doc = "CLEAR1 Phase: read '1' write '0'"]
    #[inline(always)]
    pub fn is_mbist_clear1(&self) -> bool {
        *self == Stateselect::MbistClear1
    }
    #[doc = "CLEAR2 Phase: read '0' write '1'"]
    #[inline(always)]
    pub fn is_mbist_clear2(&self) -> bool {
        *self == Stateselect::MbistClear2
    }
    #[doc = "CLEAR2B Phase: read '1' write '0'"]
    #[inline(always)]
    pub fn is_mbist_clear2b(&self) -> bool {
        *self == Stateselect::MbistClear2b
    }
    #[doc = "READ Phase: check memory is cleared"]
    #[inline(always)]
    pub fn is_mbist_read(&self) -> bool {
        *self == Stateselect::MbistRead
    }
}
#[doc = "Field `INDEX` reader - MBIST, MSA, PMSA bit Index"]
pub type IndexR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Core State"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - MBIST, MSA, PMSA bit Index"]
    #[inline(always)]
    pub fn index(&self) -> IndexR {
        IndexR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[doc = "Status C\n\nYou can [`read`](crate::Reg::read) this register and get [`statusc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatuscSpec;
impl crate::RegisterSpec for StatuscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusc::R`](R) reader structure"]
impl crate::Readable for StatuscSpec {}
#[doc = "`reset()` method sets STATUSC to value 0"]
impl crate::Resettable for StatuscSpec {}
