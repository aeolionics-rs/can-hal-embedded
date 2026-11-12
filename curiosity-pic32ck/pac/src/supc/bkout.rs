#[doc = "Register `BKOUT` reader"]
pub type R = crate::R<BkoutSpec>;
#[doc = "Register `BKOUT` writer"]
pub type W = crate::W<BkoutSpec>;
#[doc = "Field `EN0` reader - Enable Output"]
pub type En0R = crate::BitReader;
#[doc = "Field `EN0` writer - Enable Output"]
pub type En0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN1` reader - Enable Output"]
pub type En1R = crate::BitReader;
#[doc = "Field `EN1` writer - Enable Output"]
pub type En1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR0` reader - Clear Output"]
pub type Clr0R = crate::BitReader;
#[doc = "Field `CLR0` writer - Clear Output"]
pub type Clr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR1` reader - Clear Output"]
pub type Clr1R = crate::BitReader;
#[doc = "Field `CLR1` writer - Clear Output"]
pub type Clr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET0` reader - Set Output"]
pub type Set0R = crate::BitReader;
#[doc = "Field `SET0` writer - Set Output"]
pub type Set0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET1` reader - Set Output"]
pub type Set1R = crate::BitReader;
#[doc = "Field `SET1` writer - Set Output"]
pub type Set1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Toggle Output Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tglom0select {
    #[doc = "0: The output does not toggle."]
    Disable = 0,
    #[doc = "1: The output toggles on RTC event."]
    Rtctgl = 1,
    #[doc = "2: The output is set when the device enters backup mode or battery backup mode. The output should then be cleared by software."]
    Bkuptgl = 2,
}
impl From<Tglom0select> for u8 {
    #[inline(always)]
    fn from(variant: Tglom0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tglom0select {
    type Ux = u8;
}
impl crate::IsEnum for Tglom0select {}
#[doc = "Field `TGLOM0` reader - Toggle Output Mode"]
pub type Tglom0R = crate::FieldReader<Tglom0select>;
impl Tglom0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tglom0select> {
        match self.bits {
            0 => Some(Tglom0select::Disable),
            1 => Some(Tglom0select::Rtctgl),
            2 => Some(Tglom0select::Bkuptgl),
            _ => None,
        }
    }
    #[doc = "The output does not toggle."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tglom0select::Disable
    }
    #[doc = "The output toggles on RTC event."]
    #[inline(always)]
    pub fn is_rtctgl(&self) -> bool {
        *self == Tglom0select::Rtctgl
    }
    #[doc = "The output is set when the device enters backup mode or battery backup mode. The output should then be cleared by software."]
    #[inline(always)]
    pub fn is_bkuptgl(&self) -> bool {
        *self == Tglom0select::Bkuptgl
    }
}
#[doc = "Field `TGLOM0` writer - Toggle Output Mode"]
pub type Tglom0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Tglom0select>;
impl<'a, REG> Tglom0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The output does not toggle."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tglom0select::Disable)
    }
    #[doc = "The output toggles on RTC event."]
    #[inline(always)]
    pub fn rtctgl(self) -> &'a mut crate::W<REG> {
        self.variant(Tglom0select::Rtctgl)
    }
    #[doc = "The output is set when the device enters backup mode or battery backup mode. The output should then be cleared by software."]
    #[inline(always)]
    pub fn bkuptgl(self) -> &'a mut crate::W<REG> {
        self.variant(Tglom0select::Bkuptgl)
    }
}
#[doc = "Toggle Output Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tglom1select {
    #[doc = "0: The output does not toggle."]
    Disable = 0,
    #[doc = "1: The output toggles on RTC event."]
    Rtctgl = 1,
    #[doc = "2: The output is set when the device enters backup mode or battery backup mode. The output should then be cleared by software."]
    Bkuptgl = 2,
}
impl From<Tglom1select> for u8 {
    #[inline(always)]
    fn from(variant: Tglom1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tglom1select {
    type Ux = u8;
}
impl crate::IsEnum for Tglom1select {}
#[doc = "Field `TGLOM1` reader - Toggle Output Mode"]
pub type Tglom1R = crate::FieldReader<Tglom1select>;
impl Tglom1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tglom1select> {
        match self.bits {
            0 => Some(Tglom1select::Disable),
            1 => Some(Tglom1select::Rtctgl),
            2 => Some(Tglom1select::Bkuptgl),
            _ => None,
        }
    }
    #[doc = "The output does not toggle."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tglom1select::Disable
    }
    #[doc = "The output toggles on RTC event."]
    #[inline(always)]
    pub fn is_rtctgl(&self) -> bool {
        *self == Tglom1select::Rtctgl
    }
    #[doc = "The output is set when the device enters backup mode or battery backup mode. The output should then be cleared by software."]
    #[inline(always)]
    pub fn is_bkuptgl(&self) -> bool {
        *self == Tglom1select::Bkuptgl
    }
}
#[doc = "Field `TGLOM1` writer - Toggle Output Mode"]
pub type Tglom1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Tglom1select>;
impl<'a, REG> Tglom1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The output does not toggle."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tglom1select::Disable)
    }
    #[doc = "The output toggles on RTC event."]
    #[inline(always)]
    pub fn rtctgl(self) -> &'a mut crate::W<REG> {
        self.variant(Tglom1select::Rtctgl)
    }
    #[doc = "The output is set when the device enters backup mode or battery backup mode. The output should then be cleared by software."]
    #[inline(always)]
    pub fn bkuptgl(self) -> &'a mut crate::W<REG> {
        self.variant(Tglom1select::Bkuptgl)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Output"]
    #[inline(always)]
    pub fn en0(&self) -> En0R {
        En0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Output"]
    #[inline(always)]
    pub fn en1(&self) -> En1R {
        En1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear Output"]
    #[inline(always)]
    pub fn clr0(&self) -> Clr0R {
        Clr0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear Output"]
    #[inline(always)]
    pub fn clr1(&self) -> Clr1R {
        Clr1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Set Output"]
    #[inline(always)]
    pub fn set0(&self) -> Set0R {
        Set0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Set Output"]
    #[inline(always)]
    pub fn set1(&self) -> Set1R {
        Set1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Toggle Output Mode"]
    #[inline(always)]
    pub fn tglom0(&self) -> Tglom0R {
        Tglom0R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Toggle Output Mode"]
    #[inline(always)]
    pub fn tglom1(&self) -> Tglom1R {
        Tglom1R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Output"]
    #[inline(always)]
    pub fn en0(&mut self) -> En0W<'_, BkoutSpec> {
        En0W::new(self, 0)
    }
    #[doc = "Bit 2 - Enable Output"]
    #[inline(always)]
    pub fn en1(&mut self) -> En1W<'_, BkoutSpec> {
        En1W::new(self, 2)
    }
    #[doc = "Bit 8 - Clear Output"]
    #[inline(always)]
    pub fn clr0(&mut self) -> Clr0W<'_, BkoutSpec> {
        Clr0W::new(self, 8)
    }
    #[doc = "Bit 10 - Clear Output"]
    #[inline(always)]
    pub fn clr1(&mut self) -> Clr1W<'_, BkoutSpec> {
        Clr1W::new(self, 10)
    }
    #[doc = "Bit 16 - Set Output"]
    #[inline(always)]
    pub fn set0(&mut self) -> Set0W<'_, BkoutSpec> {
        Set0W::new(self, 16)
    }
    #[doc = "Bit 18 - Set Output"]
    #[inline(always)]
    pub fn set1(&mut self) -> Set1W<'_, BkoutSpec> {
        Set1W::new(self, 18)
    }
    #[doc = "Bits 24:25 - Toggle Output Mode"]
    #[inline(always)]
    pub fn tglom0(&mut self) -> Tglom0W<'_, BkoutSpec> {
        Tglom0W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Toggle Output Mode"]
    #[inline(always)]
    pub fn tglom1(&mut self) -> Tglom1W<'_, BkoutSpec> {
        Tglom1W::new(self, 26)
    }
}
#[doc = "Backup Output Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bkout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BkoutSpec;
impl crate::RegisterSpec for BkoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkout::R`](R) reader structure"]
impl crate::Readable for BkoutSpec {}
#[doc = "`write(|w| ..)` method takes [`bkout::W`](W) writer structure"]
impl crate::Writable for BkoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BKOUT to value 0"]
impl crate::Resettable for BkoutSpec {}
