#[doc = "Register `PLL0POSTDIVA` reader"]
pub type R = crate::R<Pll0postdivaSpec>;
#[doc = "Register `PLL0POSTDIVA` writer"]
pub type W = crate::W<Pll0postdivaSpec>;
#[doc = "Field `POSTDIV0` reader - PLL output 0 clock division factor"]
pub type Postdiv0R = crate::FieldReader;
#[doc = "Field `POSTDIV0` writer - PLL output 0 clock division factor"]
pub type Postdiv0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `OUTEN0` reader - PLL output 0 enable"]
pub type Outen0R = crate::BitReader;
#[doc = "Field `OUTEN0` writer - PLL output 0 enable"]
pub type Outen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSTDIV1` reader - PLL output 0 clock division factor"]
pub type Postdiv1R = crate::FieldReader;
#[doc = "Field `POSTDIV1` writer - PLL output 0 clock division factor"]
pub type Postdiv1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `OUTEN1` reader - PLL output 0 enable"]
pub type Outen1R = crate::BitReader;
#[doc = "Field `OUTEN1` writer - PLL output 0 enable"]
pub type Outen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSTDIV2` reader - PLL output 0 clock division factor"]
pub type Postdiv2R = crate::FieldReader;
#[doc = "Field `POSTDIV2` writer - PLL output 0 clock division factor"]
pub type Postdiv2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `OUTEN2` reader - PLL output 0 enable"]
pub type Outen2R = crate::BitReader;
#[doc = "Field `OUTEN2` writer - PLL output 0 enable"]
pub type Outen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSTDIV3` reader - PLL output 0 clock division factor"]
pub type Postdiv3R = crate::FieldReader;
#[doc = "Field `POSTDIV3` writer - PLL output 0 clock division factor"]
pub type Postdiv3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `OUTEN3` reader - PLL output 0 enable"]
pub type Outen3R = crate::BitReader;
#[doc = "Field `OUTEN3` writer - PLL output 0 enable"]
pub type Outen3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - PLL output 0 clock division factor"]
    #[inline(always)]
    pub fn postdiv0(&self) -> Postdiv0R {
        Postdiv0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - PLL output 0 enable"]
    #[inline(always)]
    pub fn outen0(&self) -> Outen0R {
        Outen0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - PLL output 0 clock division factor"]
    #[inline(always)]
    pub fn postdiv1(&self) -> Postdiv1R {
        Postdiv1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - PLL output 0 enable"]
    #[inline(always)]
    pub fn outen1(&self) -> Outen1R {
        Outen1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - PLL output 0 clock division factor"]
    #[inline(always)]
    pub fn postdiv2(&self) -> Postdiv2R {
        Postdiv2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - PLL output 0 enable"]
    #[inline(always)]
    pub fn outen2(&self) -> Outen2R {
        Outen2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - PLL output 0 clock division factor"]
    #[inline(always)]
    pub fn postdiv3(&self) -> Postdiv3R {
        Postdiv3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - PLL output 0 enable"]
    #[inline(always)]
    pub fn outen3(&self) -> Outen3R {
        Outen3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLL output 0 clock division factor"]
    #[inline(always)]
    pub fn postdiv0(&mut self) -> Postdiv0W<'_, Pll0postdivaSpec> {
        Postdiv0W::new(self, 0)
    }
    #[doc = "Bit 7 - PLL output 0 enable"]
    #[inline(always)]
    pub fn outen0(&mut self) -> Outen0W<'_, Pll0postdivaSpec> {
        Outen0W::new(self, 7)
    }
    #[doc = "Bits 8:13 - PLL output 0 clock division factor"]
    #[inline(always)]
    pub fn postdiv1(&mut self) -> Postdiv1W<'_, Pll0postdivaSpec> {
        Postdiv1W::new(self, 8)
    }
    #[doc = "Bit 15 - PLL output 0 enable"]
    #[inline(always)]
    pub fn outen1(&mut self) -> Outen1W<'_, Pll0postdivaSpec> {
        Outen1W::new(self, 15)
    }
    #[doc = "Bits 16:21 - PLL output 0 clock division factor"]
    #[inline(always)]
    pub fn postdiv2(&mut self) -> Postdiv2W<'_, Pll0postdivaSpec> {
        Postdiv2W::new(self, 16)
    }
    #[doc = "Bit 23 - PLL output 0 enable"]
    #[inline(always)]
    pub fn outen2(&mut self) -> Outen2W<'_, Pll0postdivaSpec> {
        Outen2W::new(self, 23)
    }
    #[doc = "Bits 24:29 - PLL output 0 clock division factor"]
    #[inline(always)]
    pub fn postdiv3(&mut self) -> Postdiv3W<'_, Pll0postdivaSpec> {
        Postdiv3W::new(self, 24)
    }
    #[doc = "Bit 31 - PLL output 0 enable"]
    #[inline(always)]
    pub fn outen3(&mut self) -> Outen3W<'_, Pll0postdivaSpec> {
        Outen3W::new(self, 31)
    }
}
#[doc = "PLL output clock divider A\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0postdiva::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0postdiva::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll0postdivaSpec;
impl crate::RegisterSpec for Pll0postdivaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0postdiva::R`](R) reader structure"]
impl crate::Readable for Pll0postdivaSpec {}
#[doc = "`write(|w| ..)` method takes [`pll0postdiva::W`](W) writer structure"]
impl crate::Writable for Pll0postdivaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL0POSTDIVA to value 0x2020_2020"]
impl crate::Resettable for Pll0postdivaSpec {
    const RESET_VALUE: u32 = 0x2020_2020;
}
