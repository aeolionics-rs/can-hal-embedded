#[doc = "Register `BOR` reader"]
pub type R = crate::R<BorSpec>;
#[doc = "Register `BOR` writer"]
pub type W = crate::W<BorSpec>;
#[doc = "Action when Threshold Crossed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Actionselect {
    #[doc = "0: The BOR generates a reset"]
    Reset = 0,
}
impl From<Actionselect> for bool {
    #[inline(always)]
    fn from(variant: Actionselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTION` reader - Action when Threshold Crossed"]
pub type ActionR = crate::BitReader<Actionselect>;
impl ActionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Actionselect> {
        match self.bits {
            false => Some(Actionselect::Reset),
            _ => None,
        }
    }
    #[doc = "The BOR generates a reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Actionselect::Reset
    }
}
#[doc = "Field `ACTION` writer - Action when Threshold Crossed"]
pub type ActionW<'a, REG> = crate::BitWriter<'a, REG, Actionselect>;
impl<'a, REG> ActionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The BOR generates a reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Actionselect::Reset)
    }
}
#[doc = "Duty Cycle BOR Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcborpselselect {
    #[doc = "0: Not Divided"]
    Nodiv = 0,
    #[doc = "1: Divide clock by 2"]
    Div2 = 1,
    #[doc = "2: Divide clock by 4"]
    Div4 = 2,
    #[doc = "3: Divide clock by 8"]
    Div8 = 3,
    #[doc = "4: Divide clock by 16"]
    Div16 = 4,
    #[doc = "5: Divide clock by 32"]
    Div32 = 5,
    #[doc = "6: Divide clock by 64"]
    Div64 = 6,
    #[doc = "7: Divide clock by 128"]
    Div128 = 7,
}
impl From<Dcborpselselect> for u8 {
    #[inline(always)]
    fn from(variant: Dcborpselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dcborpselselect {
    type Ux = u8;
}
impl crate::IsEnum for Dcborpselselect {}
#[doc = "Field `DCBORPSEL` reader - Duty Cycle BOR Prescaler Select"]
pub type DcborpselR = crate::FieldReader<Dcborpselselect>;
impl DcborpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcborpselselect {
        match self.bits {
            0 => Dcborpselselect::Nodiv,
            1 => Dcborpselselect::Div2,
            2 => Dcborpselselect::Div4,
            3 => Dcborpselselect::Div8,
            4 => Dcborpselselect::Div16,
            5 => Dcborpselselect::Div32,
            6 => Dcborpselselect::Div64,
            7 => Dcborpselselect::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "Not Divided"]
    #[inline(always)]
    pub fn is_nodiv(&self) -> bool {
        *self == Dcborpselselect::Nodiv
    }
    #[doc = "Divide clock by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Dcborpselselect::Div2
    }
    #[doc = "Divide clock by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Dcborpselselect::Div4
    }
    #[doc = "Divide clock by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Dcborpselselect::Div8
    }
    #[doc = "Divide clock by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Dcborpselselect::Div16
    }
    #[doc = "Divide clock by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Dcborpselselect::Div32
    }
    #[doc = "Divide clock by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Dcborpselselect::Div64
    }
    #[doc = "Divide clock by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Dcborpselselect::Div128
    }
}
#[doc = "Field `DCBORPSEL` writer - Duty Cycle BOR Prescaler Select"]
pub type DcborpselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dcborpselselect, crate::Safe>;
impl<'a, REG> DcborpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not Divided"]
    #[inline(always)]
    pub fn nodiv(self) -> &'a mut crate::W<REG> {
        self.variant(Dcborpselselect::Nodiv)
    }
    #[doc = "Divide clock by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Dcborpselselect::Div2)
    }
    #[doc = "Divide clock by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Dcborpselselect::Div4)
    }
    #[doc = "Divide clock by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Dcborpselselect::Div8)
    }
    #[doc = "Divide clock by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Dcborpselselect::Div16)
    }
    #[doc = "Divide clock by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Dcborpselselect::Div32)
    }
    #[doc = "Divide clock by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Dcborpselselect::Div64)
    }
    #[doc = "Divide clock by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Dcborpselselect::Div128)
    }
}
#[doc = "BOR filtering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Borfiltselect {
    #[doc = "0: No digital filtering"]
    Nofilt = 0,
    #[doc = "1: 32us filtering"]
    Filt32us = 1,
    #[doc = "2: 125us filtering"]
    Filt125us = 2,
    #[doc = "3: 250us filtering"]
    Filt250us = 3,
}
impl From<Borfiltselect> for u8 {
    #[inline(always)]
    fn from(variant: Borfiltselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Borfiltselect {
    type Ux = u8;
}
impl crate::IsEnum for Borfiltselect {}
#[doc = "Field `BORFILT` reader - BOR filtering"]
pub type BorfiltR = crate::FieldReader<Borfiltselect>;
impl BorfiltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Borfiltselect {
        match self.bits {
            0 => Borfiltselect::Nofilt,
            1 => Borfiltselect::Filt32us,
            2 => Borfiltselect::Filt125us,
            3 => Borfiltselect::Filt250us,
            _ => unreachable!(),
        }
    }
    #[doc = "No digital filtering"]
    #[inline(always)]
    pub fn is_nofilt(&self) -> bool {
        *self == Borfiltselect::Nofilt
    }
    #[doc = "32us filtering"]
    #[inline(always)]
    pub fn is_filt32us(&self) -> bool {
        *self == Borfiltselect::Filt32us
    }
    #[doc = "125us filtering"]
    #[inline(always)]
    pub fn is_filt125us(&self) -> bool {
        *self == Borfiltselect::Filt125us
    }
    #[doc = "250us filtering"]
    #[inline(always)]
    pub fn is_filt250us(&self) -> bool {
        *self == Borfiltselect::Filt250us
    }
}
#[doc = "Field `BORFILT` writer - BOR filtering"]
pub type BorfiltW<'a, REG> = crate::FieldWriter<'a, REG, 2, Borfiltselect, crate::Safe>;
impl<'a, REG> BorfiltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No digital filtering"]
    #[inline(always)]
    pub fn nofilt(self) -> &'a mut crate::W<REG> {
        self.variant(Borfiltselect::Nofilt)
    }
    #[doc = "32us filtering"]
    #[inline(always)]
    pub fn filt32us(self) -> &'a mut crate::W<REG> {
        self.variant(Borfiltselect::Filt32us)
    }
    #[doc = "125us filtering"]
    #[inline(always)]
    pub fn filt125us(self) -> &'a mut crate::W<REG> {
        self.variant(Borfiltselect::Filt125us)
    }
    #[doc = "250us filtering"]
    #[inline(always)]
    pub fn filt250us(self) -> &'a mut crate::W<REG> {
        self.variant(Borfiltselect::Filt250us)
    }
}
impl R {
    #[doc = "Bit 0 - Action when Threshold Crossed"]
    #[inline(always)]
    pub fn action(&self) -> ActionR {
        ActionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Duty Cycle BOR Prescaler Select"]
    #[inline(always)]
    pub fn dcborpsel(&self) -> DcborpselR {
        DcborpselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - BOR filtering"]
    #[inline(always)]
    pub fn borfilt(&self) -> BorfiltR {
        BorfiltR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Action when Threshold Crossed"]
    #[inline(always)]
    pub fn action(&mut self) -> ActionW<'_, BorSpec> {
        ActionW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Duty Cycle BOR Prescaler Select"]
    #[inline(always)]
    pub fn dcborpsel(&mut self) -> DcborpselW<'_, BorSpec> {
        DcborpselW::new(self, 4)
    }
    #[doc = "Bits 8:9 - BOR filtering"]
    #[inline(always)]
    pub fn borfilt(&mut self) -> BorfiltW<'_, BorSpec> {
        BorfiltW::new(self, 8)
    }
}
#[doc = "BOR Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BorSpec;
impl crate::RegisterSpec for BorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bor::R`](R) reader structure"]
impl crate::Readable for BorSpec {}
#[doc = "`write(|w| ..)` method takes [`bor::W`](W) writer structure"]
impl crate::Writable for BorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOR to value 0"]
impl crate::Resettable for BorSpec {}
