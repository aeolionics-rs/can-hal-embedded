#[doc = "Register `DATA_MBIST_MODE` reader"]
pub type R = crate::R<DataMbistModeSpec>;
#[doc = "Register `DATA_MBIST_MODE` writer"]
pub type W = crate::W<DataMbistModeSpec>;
#[doc = "MBIST state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stateselect {
    #[doc = "9: SET1 Phase: read '0' write '1'"]
    Set1 = 9,
    #[doc = "10: SET2 Phase: read '1' write '0'"]
    Set2 = 10,
    #[doc = "11: SET2B Phase: read '0' write '1'"]
    Set2b = 11,
    #[doc = "12: CLEAR1 Phase: read '1' write '0'"]
    Clear1 = 12,
    #[doc = "13: CLEAR2 Phase: read '0' write '1'"]
    Clear2 = 13,
    #[doc = "14: CLEAR2B Phase: read '1' write '0'"]
    Clear2b = 14,
    #[doc = "15: READ Phase: check memory is cleared"]
    Read = 15,
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
#[doc = "Field `STATE` reader - MBIST state"]
pub type StateR = crate::FieldReader<Stateselect>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stateselect> {
        match self.bits {
            9 => Some(Stateselect::Set1),
            10 => Some(Stateselect::Set2),
            11 => Some(Stateselect::Set2b),
            12 => Some(Stateselect::Clear1),
            13 => Some(Stateselect::Clear2),
            14 => Some(Stateselect::Clear2b),
            15 => Some(Stateselect::Read),
            _ => None,
        }
    }
    #[doc = "SET1 Phase: read '0' write '1'"]
    #[inline(always)]
    pub fn is_set1(&self) -> bool {
        *self == Stateselect::Set1
    }
    #[doc = "SET2 Phase: read '1' write '0'"]
    #[inline(always)]
    pub fn is_set2(&self) -> bool {
        *self == Stateselect::Set2
    }
    #[doc = "SET2B Phase: read '0' write '1'"]
    #[inline(always)]
    pub fn is_set2b(&self) -> bool {
        *self == Stateselect::Set2b
    }
    #[doc = "CLEAR1 Phase: read '1' write '0'"]
    #[inline(always)]
    pub fn is_clear1(&self) -> bool {
        *self == Stateselect::Clear1
    }
    #[doc = "CLEAR2 Phase: read '0' write '1'"]
    #[inline(always)]
    pub fn is_clear2(&self) -> bool {
        *self == Stateselect::Clear2
    }
    #[doc = "CLEAR2B Phase: read '1' write '0'"]
    #[inline(always)]
    pub fn is_clear2b(&self) -> bool {
        *self == Stateselect::Clear2b
    }
    #[doc = "READ Phase: check memory is cleared"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Stateselect::Read
    }
}
#[doc = "Field `STATE` writer - MBIST state"]
pub type StateW<'a, REG> = crate::FieldWriter<'a, REG, 5, Stateselect>;
impl<'a, REG> StateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SET1 Phase: read '0' write '1'"]
    #[inline(always)]
    pub fn set1(self) -> &'a mut crate::W<REG> {
        self.variant(Stateselect::Set1)
    }
    #[doc = "SET2 Phase: read '1' write '0'"]
    #[inline(always)]
    pub fn set2(self) -> &'a mut crate::W<REG> {
        self.variant(Stateselect::Set2)
    }
    #[doc = "SET2B Phase: read '0' write '1'"]
    #[inline(always)]
    pub fn set2b(self) -> &'a mut crate::W<REG> {
        self.variant(Stateselect::Set2b)
    }
    #[doc = "CLEAR1 Phase: read '1' write '0'"]
    #[inline(always)]
    pub fn clear1(self) -> &'a mut crate::W<REG> {
        self.variant(Stateselect::Clear1)
    }
    #[doc = "CLEAR2 Phase: read '0' write '1'"]
    #[inline(always)]
    pub fn clear2(self) -> &'a mut crate::W<REG> {
        self.variant(Stateselect::Clear2)
    }
    #[doc = "CLEAR2B Phase: read '1' write '0'"]
    #[inline(always)]
    pub fn clear2b(self) -> &'a mut crate::W<REG> {
        self.variant(Stateselect::Clear2b)
    }
    #[doc = "READ Phase: check memory is cleared"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Stateselect::Read)
    }
}
#[doc = "Field `INDEX` reader - MBIST bit Index"]
pub type IndexR = crate::FieldReader;
#[doc = "Field `INDEX` writer - MBIST bit Index"]
pub type IndexW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - MBIST state"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - MBIST bit Index"]
    #[inline(always)]
    pub fn index(&self) -> IndexR {
        IndexR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - MBIST state"]
    #[inline(always)]
    pub fn state(&mut self) -> StateW<'_, DataMbistModeSpec> {
        StateW::new(self, 0)
    }
    #[doc = "Bits 8:12 - MBIST bit Index"]
    #[inline(always)]
    pub fn index(&mut self) -> IndexW<'_, DataMbistModeSpec> {
        IndexW::new(self, 8)
    }
}
#[doc = "Data\n\nYou can [`read`](crate::Reg::read) this register and get [`data_mbist_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_mbist_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataMbistModeSpec;
impl crate::RegisterSpec for DataMbistModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_mbist_mode::R`](R) reader structure"]
impl crate::Readable for DataMbistModeSpec {}
#[doc = "`write(|w| ..)` method takes [`data_mbist_mode::W`](W) writer structure"]
impl crate::Writable for DataMbistModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_MBIST_MODE to value 0"]
impl crate::Resettable for DataMbistModeSpec {}
