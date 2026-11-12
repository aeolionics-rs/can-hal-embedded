#[doc = "Register `PLL0CTRL` reader"]
pub type R = crate::R<Pll0ctrlSpec>;
#[doc = "Register `PLL0CTRL` writer"]
pub type W = crate::W<Pll0ctrlSpec>;
#[doc = "Field `ENABLE` reader - PLL Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - PLL Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WrtlockR = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reference selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refselselect {
    #[doc = "0: Dedicated GCLK clock reference"]
    Gclk = 0,
    #[doc = "1: XOSC clock reference"]
    Xosc = 1,
    #[doc = "2: DFLL48M clock reference"]
    Dfll48m = 2,
}
impl From<Refselselect> for u8 {
    #[inline(always)]
    fn from(variant: Refselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refselselect {
    type Ux = u8;
}
impl crate::IsEnum for Refselselect {}
#[doc = "Field `REFSEL` reader - Reference selection"]
pub type RefselR = crate::FieldReader<Refselselect>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Refselselect> {
        match self.bits {
            0 => Some(Refselselect::Gclk),
            1 => Some(Refselselect::Xosc),
            2 => Some(Refselselect::Dfll48m),
            _ => None,
        }
    }
    #[doc = "Dedicated GCLK clock reference"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == Refselselect::Gclk
    }
    #[doc = "XOSC clock reference"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == Refselselect::Xosc
    }
    #[doc = "DFLL48M clock reference"]
    #[inline(always)]
    pub fn is_dfll48m(&self) -> bool {
        *self == Refselselect::Dfll48m
    }
}
#[doc = "Field `REFSEL` writer - Reference selection"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Refselselect>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Dedicated GCLK clock reference"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Gclk)
    }
    #[doc = "XOSC clock reference"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Xosc)
    }
    #[doc = "DFLL48M clock reference"]
    #[inline(always)]
    pub fn dfll48m(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Dfll48m)
    }
}
#[doc = "Bandwidth selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bwselselect {
    #[doc = "1: Fpfd is greater than or equal to 4MHz and less than 10MHz"]
    Bwsel1 = 1,
    #[doc = "2: Fpfd is greater than or equal to 10MHz and less than 20MHz"]
    Bwsel2 = 2,
    #[doc = "3: Fpfd is greater than or equal to 20MHz and less than 30MHz"]
    Bwsel3 = 3,
    #[doc = "4: Fpfd is greater than or equal to 30MHz and less than 60MHz"]
    Bwsel4 = 4,
}
impl From<Bwselselect> for u8 {
    #[inline(always)]
    fn from(variant: Bwselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bwselselect {
    type Ux = u8;
}
impl crate::IsEnum for Bwselselect {}
#[doc = "Field `BWSEL` reader - Bandwidth selection"]
pub type BwselR = crate::FieldReader<Bwselselect>;
impl BwselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bwselselect> {
        match self.bits {
            1 => Some(Bwselselect::Bwsel1),
            2 => Some(Bwselselect::Bwsel2),
            3 => Some(Bwselselect::Bwsel3),
            4 => Some(Bwselselect::Bwsel4),
            _ => None,
        }
    }
    #[doc = "Fpfd is greater than or equal to 4MHz and less than 10MHz"]
    #[inline(always)]
    pub fn is_bwsel1(&self) -> bool {
        *self == Bwselselect::Bwsel1
    }
    #[doc = "Fpfd is greater than or equal to 10MHz and less than 20MHz"]
    #[inline(always)]
    pub fn is_bwsel2(&self) -> bool {
        *self == Bwselselect::Bwsel2
    }
    #[doc = "Fpfd is greater than or equal to 20MHz and less than 30MHz"]
    #[inline(always)]
    pub fn is_bwsel3(&self) -> bool {
        *self == Bwselselect::Bwsel3
    }
    #[doc = "Fpfd is greater than or equal to 30MHz and less than 60MHz"]
    #[inline(always)]
    pub fn is_bwsel4(&self) -> bool {
        *self == Bwselselect::Bwsel4
    }
}
#[doc = "Field `BWSEL` writer - Bandwidth selection"]
pub type BwselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Bwselselect>;
impl<'a, REG> BwselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Fpfd is greater than or equal to 4MHz and less than 10MHz"]
    #[inline(always)]
    pub fn bwsel1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwselselect::Bwsel1)
    }
    #[doc = "Fpfd is greater than or equal to 10MHz and less than 20MHz"]
    #[inline(always)]
    pub fn bwsel2(self) -> &'a mut crate::W<REG> {
        self.variant(Bwselselect::Bwsel2)
    }
    #[doc = "Fpfd is greater than or equal to 20MHz and less than 30MHz"]
    #[inline(always)]
    pub fn bwsel3(self) -> &'a mut crate::W<REG> {
        self.variant(Bwselselect::Bwsel3)
    }
    #[doc = "Fpfd is greater than or equal to 30MHz and less than 60MHz"]
    #[inline(always)]
    pub fn bwsel4(self) -> &'a mut crate::W<REG> {
        self.variant(Bwselselect::Bwsel4)
    }
}
impl R {
    #[doc = "Bit 1 - PLL Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WrtlockR {
        WrtlockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Reference selection"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Bandwidth selection"]
    #[inline(always)]
    pub fn bwsel(&self) -> BwselR {
        BwselR::new(((self.bits >> 11) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - PLL Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Pll0ctrlSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WrtlockW<'_, Pll0ctrlSpec> {
        WrtlockW::new(self, 2)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> OndemandW<'_, Pll0ctrlSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Reference selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<'_, Pll0ctrlSpec> {
        RefselW::new(self, 8)
    }
    #[doc = "Bits 11:13 - Bandwidth selection"]
    #[inline(always)]
    pub fn bwsel(&mut self) -> BwselW<'_, Pll0ctrlSpec> {
        BwselW::new(self, 11)
    }
}
#[doc = "PLL Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll0ctrlSpec;
impl crate::RegisterSpec for Pll0ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0ctrl::R`](R) reader structure"]
impl crate::Readable for Pll0ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pll0ctrl::W`](W) writer structure"]
impl crate::Writable for Pll0ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL0CTRL to value 0"]
impl crate::Resettable for Pll0ctrlSpec {}
