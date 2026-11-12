#[doc = "Register `FRAMECTRL` reader"]
pub type R = crate::R<FramectrlSpec>;
#[doc = "Register `FRAMECTRL` writer"]
pub type W = crate::W<FramectrlSpec>;
#[doc = "Frame sync pulse width in serial words\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frmsypwselect {
    #[doc = "0: Frame sync pulse is one clock wide"]
    Sypw0 = 0,
    #[doc = "1: Frame sync pulse is one slots wide"]
    Sypw1 = 1,
    #[doc = "2: Frame sync pulse is two slots wide"]
    Sypw2 = 2,
    #[doc = "3: Frame sync pulse is three slots wide"]
    Sypw3 = 3,
    #[doc = "4: Frame sync pulse is four slots wide"]
    Sypw4 = 4,
    #[doc = "5: Frame sync pulse is five slots wide"]
    Sypw5 = 5,
    #[doc = "6: Frame sync pulse is six slots wide"]
    Sypw6 = 6,
    #[doc = "7: Frame sync pulse is seven slots wide"]
    Sypw7 = 7,
    #[doc = "8: Frame sync pulse is eight slots wide"]
    Sypw8 = 8,
    #[doc = "9: Frame sync pulse is sixteen slots wide"]
    Sypw16 = 9,
    #[doc = "10: Frame sync pulse is thirty one slots wide"]
    Sypw31 = 10,
}
impl From<Frmsypwselect> for u8 {
    #[inline(always)]
    fn from(variant: Frmsypwselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frmsypwselect {
    type Ux = u8;
}
impl crate::IsEnum for Frmsypwselect {}
#[doc = "Field `FRMSYPW` reader - Frame sync pulse width in serial words"]
pub type FrmsypwR = crate::FieldReader<Frmsypwselect>;
impl FrmsypwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frmsypwselect> {
        match self.bits {
            0 => Some(Frmsypwselect::Sypw0),
            1 => Some(Frmsypwselect::Sypw1),
            2 => Some(Frmsypwselect::Sypw2),
            3 => Some(Frmsypwselect::Sypw3),
            4 => Some(Frmsypwselect::Sypw4),
            5 => Some(Frmsypwselect::Sypw5),
            6 => Some(Frmsypwselect::Sypw6),
            7 => Some(Frmsypwselect::Sypw7),
            8 => Some(Frmsypwselect::Sypw8),
            9 => Some(Frmsypwselect::Sypw16),
            10 => Some(Frmsypwselect::Sypw31),
            _ => None,
        }
    }
    #[doc = "Frame sync pulse is one clock wide"]
    #[inline(always)]
    pub fn is_sypw0(&self) -> bool {
        *self == Frmsypwselect::Sypw0
    }
    #[doc = "Frame sync pulse is one slots wide"]
    #[inline(always)]
    pub fn is_sypw1(&self) -> bool {
        *self == Frmsypwselect::Sypw1
    }
    #[doc = "Frame sync pulse is two slots wide"]
    #[inline(always)]
    pub fn is_sypw2(&self) -> bool {
        *self == Frmsypwselect::Sypw2
    }
    #[doc = "Frame sync pulse is three slots wide"]
    #[inline(always)]
    pub fn is_sypw3(&self) -> bool {
        *self == Frmsypwselect::Sypw3
    }
    #[doc = "Frame sync pulse is four slots wide"]
    #[inline(always)]
    pub fn is_sypw4(&self) -> bool {
        *self == Frmsypwselect::Sypw4
    }
    #[doc = "Frame sync pulse is five slots wide"]
    #[inline(always)]
    pub fn is_sypw5(&self) -> bool {
        *self == Frmsypwselect::Sypw5
    }
    #[doc = "Frame sync pulse is six slots wide"]
    #[inline(always)]
    pub fn is_sypw6(&self) -> bool {
        *self == Frmsypwselect::Sypw6
    }
    #[doc = "Frame sync pulse is seven slots wide"]
    #[inline(always)]
    pub fn is_sypw7(&self) -> bool {
        *self == Frmsypwselect::Sypw7
    }
    #[doc = "Frame sync pulse is eight slots wide"]
    #[inline(always)]
    pub fn is_sypw8(&self) -> bool {
        *self == Frmsypwselect::Sypw8
    }
    #[doc = "Frame sync pulse is sixteen slots wide"]
    #[inline(always)]
    pub fn is_sypw16(&self) -> bool {
        *self == Frmsypwselect::Sypw16
    }
    #[doc = "Frame sync pulse is thirty one slots wide"]
    #[inline(always)]
    pub fn is_sypw31(&self) -> bool {
        *self == Frmsypwselect::Sypw31
    }
}
#[doc = "Field `FRMSYPW` writer - Frame sync pulse width in serial words"]
pub type FrmsypwW<'a, REG> = crate::FieldWriter<'a, REG, 4, Frmsypwselect>;
impl<'a, REG> FrmsypwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Frame sync pulse is one clock wide"]
    #[inline(always)]
    pub fn sypw0(self) -> &'a mut crate::W<REG> {
        self.variant(Frmsypwselect::Sypw0)
    }
    #[doc = "Frame sync pulse is one slots wide"]
    #[inline(always)]
    pub fn sypw1(self) -> &'a mut crate::W<REG> {
        self.variant(Frmsypwselect::Sypw1)
    }
    #[doc = "Frame sync pulse is two slots wide"]
    #[inline(always)]
    pub fn sypw2(self) -> &'a mut crate::W<REG> {
        self.variant(Frmsypwselect::Sypw2)
    }
    #[doc = "Frame sync pulse is three slots wide"]
    #[inline(always)]
    pub fn sypw3(self) -> &'a mut crate::W<REG> {
        self.variant(Frmsypwselect::Sypw3)
    }
    #[doc = "Frame sync pulse is four slots wide"]
    #[inline(always)]
    pub fn sypw4(self) -> &'a mut crate::W<REG> {
        self.variant(Frmsypwselect::Sypw4)
    }
    #[doc = "Frame sync pulse is five slots wide"]
    #[inline(always)]
    pub fn sypw5(self) -> &'a mut crate::W<REG> {
        self.variant(Frmsypwselect::Sypw5)
    }
    #[doc = "Frame sync pulse is six slots wide"]
    #[inline(always)]
    pub fn sypw6(self) -> &'a mut crate::W<REG> {
        self.variant(Frmsypwselect::Sypw6)
    }
    #[doc = "Frame sync pulse is seven slots wide"]
    #[inline(always)]
    pub fn sypw7(self) -> &'a mut crate::W<REG> {
        self.variant(Frmsypwselect::Sypw7)
    }
    #[doc = "Frame sync pulse is eight slots wide"]
    #[inline(always)]
    pub fn sypw8(self) -> &'a mut crate::W<REG> {
        self.variant(Frmsypwselect::Sypw8)
    }
    #[doc = "Frame sync pulse is sixteen slots wide"]
    #[inline(always)]
    pub fn sypw16(self) -> &'a mut crate::W<REG> {
        self.variant(Frmsypwselect::Sypw16)
    }
    #[doc = "Frame sync pulse is thirty one slots wide"]
    #[inline(always)]
    pub fn sypw31(self) -> &'a mut crate::W<REG> {
        self.variant(Frmsypwselect::Sypw31)
    }
}
#[doc = "Field `FRMCOINC` reader - Frame Sync Pulse Edge Select bit"]
pub type FrmcoincR = crate::BitReader;
#[doc = "Field `FRMCOINC` writer - Frame Sync Pulse Edge Select bit"]
pub type FrmcoincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMPOL` reader - Frame Sync / Slave Select Polarity bit"]
pub type FrmpolR = crate::BitReader;
#[doc = "Field `FRMPOL` writer - Frame Sync / Slave Select Polarity bit"]
pub type FrmpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMSLV` reader - Frame Sync Pulse Direction Control bit"]
pub type FrmslvR = crate::BitReader;
#[doc = "Field `FRMSLV` writer - Frame Sync Pulse Direction Control bit"]
pub type FrmslvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Frame sync pulse counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frmcntselect {
    #[doc = "0: Generate a Frame sync pulse on each serial word"]
    Cnt1 = 0,
    #[doc = "1: Number of slots per frame sync pulse is 2"]
    Cnt2 = 1,
    #[doc = "2: Number of slots per frame sync pulse is 3"]
    Cnt3 = 2,
    #[doc = "3: Number of slots per frame sync pulse is 4"]
    Cnt4 = 3,
    #[doc = "4: Number of slots per frame sync pulse is 5"]
    Cnt5 = 4,
    #[doc = "5: Number of slots per frame sync pulse is 6"]
    Cnt6 = 5,
    #[doc = "6: Number of slots per frame sync pulse is 8"]
    Cnt8 = 6,
    #[doc = "7: Number of slots per frame sync pulse is 10"]
    Cnt10 = 7,
    #[doc = "8: Number of slots per frame sync pulse is 12"]
    Cnt12 = 8,
    #[doc = "9: Number of slots per frame sync pulse is 14"]
    Cnt14 = 9,
    #[doc = "10: Number of slots per frame sync pulse is 16"]
    Cnt16 = 10,
    #[doc = "11: Number of slots per frame sync pulse is 18"]
    Cnt18 = 11,
    #[doc = "12: Number of slots per frame sync pulse is 20"]
    Cnt20 = 12,
    #[doc = "13: Number of slots per frame sync pulse is 22"]
    Cnt22 = 13,
    #[doc = "14: Number of slots per frame sync pulse is 24"]
    Cnt24 = 14,
    #[doc = "15: Number of slots per frame sync pulse is 26"]
    Cnt26 = 15,
    #[doc = "16: Number of slots per frame sync pulse is 28"]
    Cnt28 = 16,
    #[doc = "17: Number of slots per frame sync pulse is 30"]
    Cnt30 = 17,
    #[doc = "18: Number of slots per frame sync pulse is 32"]
    Cnt32 = 18,
}
impl From<Frmcntselect> for u8 {
    #[inline(always)]
    fn from(variant: Frmcntselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frmcntselect {
    type Ux = u8;
}
impl crate::IsEnum for Frmcntselect {}
#[doc = "Field `FRMCNT` reader - Frame sync pulse counter"]
pub type FrmcntR = crate::FieldReader<Frmcntselect>;
impl FrmcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frmcntselect> {
        match self.bits {
            0 => Some(Frmcntselect::Cnt1),
            1 => Some(Frmcntselect::Cnt2),
            2 => Some(Frmcntselect::Cnt3),
            3 => Some(Frmcntselect::Cnt4),
            4 => Some(Frmcntselect::Cnt5),
            5 => Some(Frmcntselect::Cnt6),
            6 => Some(Frmcntselect::Cnt8),
            7 => Some(Frmcntselect::Cnt10),
            8 => Some(Frmcntselect::Cnt12),
            9 => Some(Frmcntselect::Cnt14),
            10 => Some(Frmcntselect::Cnt16),
            11 => Some(Frmcntselect::Cnt18),
            12 => Some(Frmcntselect::Cnt20),
            13 => Some(Frmcntselect::Cnt22),
            14 => Some(Frmcntselect::Cnt24),
            15 => Some(Frmcntselect::Cnt26),
            16 => Some(Frmcntselect::Cnt28),
            17 => Some(Frmcntselect::Cnt30),
            18 => Some(Frmcntselect::Cnt32),
            _ => None,
        }
    }
    #[doc = "Generate a Frame sync pulse on each serial word"]
    #[inline(always)]
    pub fn is_cnt1(&self) -> bool {
        *self == Frmcntselect::Cnt1
    }
    #[doc = "Number of slots per frame sync pulse is 2"]
    #[inline(always)]
    pub fn is_cnt2(&self) -> bool {
        *self == Frmcntselect::Cnt2
    }
    #[doc = "Number of slots per frame sync pulse is 3"]
    #[inline(always)]
    pub fn is_cnt3(&self) -> bool {
        *self == Frmcntselect::Cnt3
    }
    #[doc = "Number of slots per frame sync pulse is 4"]
    #[inline(always)]
    pub fn is_cnt4(&self) -> bool {
        *self == Frmcntselect::Cnt4
    }
    #[doc = "Number of slots per frame sync pulse is 5"]
    #[inline(always)]
    pub fn is_cnt5(&self) -> bool {
        *self == Frmcntselect::Cnt5
    }
    #[doc = "Number of slots per frame sync pulse is 6"]
    #[inline(always)]
    pub fn is_cnt6(&self) -> bool {
        *self == Frmcntselect::Cnt6
    }
    #[doc = "Number of slots per frame sync pulse is 8"]
    #[inline(always)]
    pub fn is_cnt8(&self) -> bool {
        *self == Frmcntselect::Cnt8
    }
    #[doc = "Number of slots per frame sync pulse is 10"]
    #[inline(always)]
    pub fn is_cnt10(&self) -> bool {
        *self == Frmcntselect::Cnt10
    }
    #[doc = "Number of slots per frame sync pulse is 12"]
    #[inline(always)]
    pub fn is_cnt12(&self) -> bool {
        *self == Frmcntselect::Cnt12
    }
    #[doc = "Number of slots per frame sync pulse is 14"]
    #[inline(always)]
    pub fn is_cnt14(&self) -> bool {
        *self == Frmcntselect::Cnt14
    }
    #[doc = "Number of slots per frame sync pulse is 16"]
    #[inline(always)]
    pub fn is_cnt16(&self) -> bool {
        *self == Frmcntselect::Cnt16
    }
    #[doc = "Number of slots per frame sync pulse is 18"]
    #[inline(always)]
    pub fn is_cnt18(&self) -> bool {
        *self == Frmcntselect::Cnt18
    }
    #[doc = "Number of slots per frame sync pulse is 20"]
    #[inline(always)]
    pub fn is_cnt20(&self) -> bool {
        *self == Frmcntselect::Cnt20
    }
    #[doc = "Number of slots per frame sync pulse is 22"]
    #[inline(always)]
    pub fn is_cnt22(&self) -> bool {
        *self == Frmcntselect::Cnt22
    }
    #[doc = "Number of slots per frame sync pulse is 24"]
    #[inline(always)]
    pub fn is_cnt24(&self) -> bool {
        *self == Frmcntselect::Cnt24
    }
    #[doc = "Number of slots per frame sync pulse is 26"]
    #[inline(always)]
    pub fn is_cnt26(&self) -> bool {
        *self == Frmcntselect::Cnt26
    }
    #[doc = "Number of slots per frame sync pulse is 28"]
    #[inline(always)]
    pub fn is_cnt28(&self) -> bool {
        *self == Frmcntselect::Cnt28
    }
    #[doc = "Number of slots per frame sync pulse is 30"]
    #[inline(always)]
    pub fn is_cnt30(&self) -> bool {
        *self == Frmcntselect::Cnt30
    }
    #[doc = "Number of slots per frame sync pulse is 32"]
    #[inline(always)]
    pub fn is_cnt32(&self) -> bool {
        *self == Frmcntselect::Cnt32
    }
}
#[doc = "Field `FRMCNT` writer - Frame sync pulse counter"]
pub type FrmcntW<'a, REG> = crate::FieldWriter<'a, REG, 5, Frmcntselect>;
impl<'a, REG> FrmcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generate a Frame sync pulse on each serial word"]
    #[inline(always)]
    pub fn cnt1(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt1)
    }
    #[doc = "Number of slots per frame sync pulse is 2"]
    #[inline(always)]
    pub fn cnt2(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt2)
    }
    #[doc = "Number of slots per frame sync pulse is 3"]
    #[inline(always)]
    pub fn cnt3(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt3)
    }
    #[doc = "Number of slots per frame sync pulse is 4"]
    #[inline(always)]
    pub fn cnt4(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt4)
    }
    #[doc = "Number of slots per frame sync pulse is 5"]
    #[inline(always)]
    pub fn cnt5(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt5)
    }
    #[doc = "Number of slots per frame sync pulse is 6"]
    #[inline(always)]
    pub fn cnt6(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt6)
    }
    #[doc = "Number of slots per frame sync pulse is 8"]
    #[inline(always)]
    pub fn cnt8(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt8)
    }
    #[doc = "Number of slots per frame sync pulse is 10"]
    #[inline(always)]
    pub fn cnt10(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt10)
    }
    #[doc = "Number of slots per frame sync pulse is 12"]
    #[inline(always)]
    pub fn cnt12(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt12)
    }
    #[doc = "Number of slots per frame sync pulse is 14"]
    #[inline(always)]
    pub fn cnt14(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt14)
    }
    #[doc = "Number of slots per frame sync pulse is 16"]
    #[inline(always)]
    pub fn cnt16(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt16)
    }
    #[doc = "Number of slots per frame sync pulse is 18"]
    #[inline(always)]
    pub fn cnt18(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt18)
    }
    #[doc = "Number of slots per frame sync pulse is 20"]
    #[inline(always)]
    pub fn cnt20(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt20)
    }
    #[doc = "Number of slots per frame sync pulse is 22"]
    #[inline(always)]
    pub fn cnt22(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt22)
    }
    #[doc = "Number of slots per frame sync pulse is 24"]
    #[inline(always)]
    pub fn cnt24(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt24)
    }
    #[doc = "Number of slots per frame sync pulse is 26"]
    #[inline(always)]
    pub fn cnt26(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt26)
    }
    #[doc = "Number of slots per frame sync pulse is 28"]
    #[inline(always)]
    pub fn cnt28(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt28)
    }
    #[doc = "Number of slots per frame sync pulse is 30"]
    #[inline(always)]
    pub fn cnt30(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt30)
    }
    #[doc = "Number of slots per frame sync pulse is 32"]
    #[inline(always)]
    pub fn cnt32(self) -> &'a mut crate::W<REG> {
        self.variant(Frmcntselect::Cnt32)
    }
}
#[doc = "TDM Number of Bits in a Slot Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tdmsszselect {
    #[doc = "0: Slot Size 8"]
    Sltsz8 = 0,
    #[doc = "1: Slot Size 12"]
    Sltsz12 = 1,
    #[doc = "2: Slot Size 16"]
    Sltsz16 = 2,
    #[doc = "3: Slot Size 20"]
    Sltsz20 = 3,
    #[doc = "4: Slot Size 24"]
    Sltsz24 = 4,
    #[doc = "5: Slot Size 28"]
    Sltsz28 = 5,
    #[doc = "6: Slot Size 32"]
    Sltsz32 = 6,
}
impl From<Tdmsszselect> for u8 {
    #[inline(always)]
    fn from(variant: Tdmsszselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tdmsszselect {
    type Ux = u8;
}
impl crate::IsEnum for Tdmsszselect {}
#[doc = "Field `TDMSSZ` reader - TDM Number of Bits in a Slot Size"]
pub type TdmsszR = crate::FieldReader<Tdmsszselect>;
impl TdmsszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tdmsszselect> {
        match self.bits {
            0 => Some(Tdmsszselect::Sltsz8),
            1 => Some(Tdmsszselect::Sltsz12),
            2 => Some(Tdmsszselect::Sltsz16),
            3 => Some(Tdmsszselect::Sltsz20),
            4 => Some(Tdmsszselect::Sltsz24),
            5 => Some(Tdmsszselect::Sltsz28),
            6 => Some(Tdmsszselect::Sltsz32),
            _ => None,
        }
    }
    #[doc = "Slot Size 8"]
    #[inline(always)]
    pub fn is_sltsz8(&self) -> bool {
        *self == Tdmsszselect::Sltsz8
    }
    #[doc = "Slot Size 12"]
    #[inline(always)]
    pub fn is_sltsz12(&self) -> bool {
        *self == Tdmsszselect::Sltsz12
    }
    #[doc = "Slot Size 16"]
    #[inline(always)]
    pub fn is_sltsz16(&self) -> bool {
        *self == Tdmsszselect::Sltsz16
    }
    #[doc = "Slot Size 20"]
    #[inline(always)]
    pub fn is_sltsz20(&self) -> bool {
        *self == Tdmsszselect::Sltsz20
    }
    #[doc = "Slot Size 24"]
    #[inline(always)]
    pub fn is_sltsz24(&self) -> bool {
        *self == Tdmsszselect::Sltsz24
    }
    #[doc = "Slot Size 28"]
    #[inline(always)]
    pub fn is_sltsz28(&self) -> bool {
        *self == Tdmsszselect::Sltsz28
    }
    #[doc = "Slot Size 32"]
    #[inline(always)]
    pub fn is_sltsz32(&self) -> bool {
        *self == Tdmsszselect::Sltsz32
    }
}
#[doc = "Field `TDMSSZ` writer - TDM Number of Bits in a Slot Size"]
pub type TdmsszW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tdmsszselect>;
impl<'a, REG> TdmsszW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slot Size 8"]
    #[inline(always)]
    pub fn sltsz8(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmsszselect::Sltsz8)
    }
    #[doc = "Slot Size 12"]
    #[inline(always)]
    pub fn sltsz12(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmsszselect::Sltsz12)
    }
    #[doc = "Slot Size 16"]
    #[inline(always)]
    pub fn sltsz16(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmsszselect::Sltsz16)
    }
    #[doc = "Slot Size 20"]
    #[inline(always)]
    pub fn sltsz20(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmsszselect::Sltsz20)
    }
    #[doc = "Slot Size 24"]
    #[inline(always)]
    pub fn sltsz24(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmsszselect::Sltsz24)
    }
    #[doc = "Slot Size 28"]
    #[inline(always)]
    pub fn sltsz28(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmsszselect::Sltsz28)
    }
    #[doc = "Slot Size 32"]
    #[inline(always)]
    pub fn sltsz32(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmsszselect::Sltsz32)
    }
}
#[doc = "TDM Number of Bits in a Word Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tdmwszselect {
    #[doc = "0: Word Size 8"]
    Wdsz8 = 0,
    #[doc = "1: Word Size 12"]
    Wdsz12 = 1,
    #[doc = "2: Word Size 16"]
    Wdsz16 = 2,
    #[doc = "3: Word Size 20"]
    Wdsz20 = 3,
    #[doc = "4: Word Size 24"]
    Wdsz24 = 4,
    #[doc = "5: Word Size 28"]
    Wdsz28 = 5,
    #[doc = "6: Word Size 32"]
    Wdsz32 = 6,
}
impl From<Tdmwszselect> for u8 {
    #[inline(always)]
    fn from(variant: Tdmwszselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tdmwszselect {
    type Ux = u8;
}
impl crate::IsEnum for Tdmwszselect {}
#[doc = "Field `TDMWSZ` reader - TDM Number of Bits in a Word Size"]
pub type TdmwszR = crate::FieldReader<Tdmwszselect>;
impl TdmwszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tdmwszselect> {
        match self.bits {
            0 => Some(Tdmwszselect::Wdsz8),
            1 => Some(Tdmwszselect::Wdsz12),
            2 => Some(Tdmwszselect::Wdsz16),
            3 => Some(Tdmwszselect::Wdsz20),
            4 => Some(Tdmwszselect::Wdsz24),
            5 => Some(Tdmwszselect::Wdsz28),
            6 => Some(Tdmwszselect::Wdsz32),
            _ => None,
        }
    }
    #[doc = "Word Size 8"]
    #[inline(always)]
    pub fn is_wdsz8(&self) -> bool {
        *self == Tdmwszselect::Wdsz8
    }
    #[doc = "Word Size 12"]
    #[inline(always)]
    pub fn is_wdsz12(&self) -> bool {
        *self == Tdmwszselect::Wdsz12
    }
    #[doc = "Word Size 16"]
    #[inline(always)]
    pub fn is_wdsz16(&self) -> bool {
        *self == Tdmwszselect::Wdsz16
    }
    #[doc = "Word Size 20"]
    #[inline(always)]
    pub fn is_wdsz20(&self) -> bool {
        *self == Tdmwszselect::Wdsz20
    }
    #[doc = "Word Size 24"]
    #[inline(always)]
    pub fn is_wdsz24(&self) -> bool {
        *self == Tdmwszselect::Wdsz24
    }
    #[doc = "Word Size 28"]
    #[inline(always)]
    pub fn is_wdsz28(&self) -> bool {
        *self == Tdmwszselect::Wdsz28
    }
    #[doc = "Word Size 32"]
    #[inline(always)]
    pub fn is_wdsz32(&self) -> bool {
        *self == Tdmwszselect::Wdsz32
    }
}
#[doc = "Field `TDMWSZ` writer - TDM Number of Bits in a Word Size"]
pub type TdmwszW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tdmwszselect>;
impl<'a, REG> TdmwszW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Word Size 8"]
    #[inline(always)]
    pub fn wdsz8(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmwszselect::Wdsz8)
    }
    #[doc = "Word Size 12"]
    #[inline(always)]
    pub fn wdsz12(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmwszselect::Wdsz12)
    }
    #[doc = "Word Size 16"]
    #[inline(always)]
    pub fn wdsz16(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmwszselect::Wdsz16)
    }
    #[doc = "Word Size 20"]
    #[inline(always)]
    pub fn wdsz20(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmwszselect::Wdsz20)
    }
    #[doc = "Word Size 24"]
    #[inline(always)]
    pub fn wdsz24(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmwszselect::Wdsz24)
    }
    #[doc = "Word Size 28"]
    #[inline(always)]
    pub fn wdsz28(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmwszselect::Wdsz28)
    }
    #[doc = "Word Size 32"]
    #[inline(always)]
    pub fn wdsz32(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmwszselect::Wdsz32)
    }
}
impl R {
    #[doc = "Bits 0:3 - Frame sync pulse width in serial words"]
    #[inline(always)]
    pub fn frmsypw(&self) -> FrmsypwR {
        FrmsypwR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Frame Sync Pulse Edge Select bit"]
    #[inline(always)]
    pub fn frmcoinc(&self) -> FrmcoincR {
        FrmcoincR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - Frame Sync / Slave Select Polarity bit"]
    #[inline(always)]
    pub fn frmpol(&self) -> FrmpolR {
        FrmpolR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Frame Sync Pulse Direction Control bit"]
    #[inline(always)]
    pub fn frmslv(&self) -> FrmslvR {
        FrmslvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Frame sync pulse counter"]
    #[inline(always)]
    pub fn frmcnt(&self) -> FrmcntR {
        FrmcntR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - TDM Number of Bits in a Slot Size"]
    #[inline(always)]
    pub fn tdmssz(&self) -> TdmsszR {
        TdmsszR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 29:31 - TDM Number of Bits in a Word Size"]
    #[inline(always)]
    pub fn tdmwsz(&self) -> TdmwszR {
        TdmwszR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Frame sync pulse width in serial words"]
    #[inline(always)]
    pub fn frmsypw(&mut self) -> FrmsypwW<'_, FramectrlSpec> {
        FrmsypwW::new(self, 0)
    }
    #[doc = "Bit 8 - Frame Sync Pulse Edge Select bit"]
    #[inline(always)]
    pub fn frmcoinc(&mut self) -> FrmcoincW<'_, FramectrlSpec> {
        FrmcoincW::new(self, 8)
    }
    #[doc = "Bit 13 - Frame Sync / Slave Select Polarity bit"]
    #[inline(always)]
    pub fn frmpol(&mut self) -> FrmpolW<'_, FramectrlSpec> {
        FrmpolW::new(self, 13)
    }
    #[doc = "Bit 14 - Frame Sync Pulse Direction Control bit"]
    #[inline(always)]
    pub fn frmslv(&mut self) -> FrmslvW<'_, FramectrlSpec> {
        FrmslvW::new(self, 14)
    }
    #[doc = "Bits 16:20 - Frame sync pulse counter"]
    #[inline(always)]
    pub fn frmcnt(&mut self) -> FrmcntW<'_, FramectrlSpec> {
        FrmcntW::new(self, 16)
    }
    #[doc = "Bits 24:26 - TDM Number of Bits in a Slot Size"]
    #[inline(always)]
    pub fn tdmssz(&mut self) -> TdmsszW<'_, FramectrlSpec> {
        TdmsszW::new(self, 24)
    }
    #[doc = "Bits 29:31 - TDM Number of Bits in a Word Size"]
    #[inline(always)]
    pub fn tdmwsz(&mut self) -> TdmwszW<'_, FramectrlSpec> {
        TdmwszW::new(self, 29)
    }
}
#[doc = "SPI Control Frame Register\n\nYou can [`read`](crate::Reg::read) this register and get [`framectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FramectrlSpec;
impl crate::RegisterSpec for FramectrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framectrl::R`](R) reader structure"]
impl crate::Readable for FramectrlSpec {}
#[doc = "`write(|w| ..)` method takes [`framectrl::W`](W) writer structure"]
impl crate::Writable for FramectrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMECTRL to value 0"]
impl crate::Resettable for FramectrlSpec {}
