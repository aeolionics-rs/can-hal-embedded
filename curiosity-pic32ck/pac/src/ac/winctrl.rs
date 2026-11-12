#[doc = "Register `WINCTRL` reader"]
pub type R = crate::R<WinctrlSpec>;
#[doc = "Register `WINCTRL` writer"]
pub type W = crate::W<WinctrlSpec>;
#[doc = "Field `WEN` reader - Window n Mode Enable"]
pub type WenR = crate::BitReader;
#[doc = "Field `WEN` writer - Window n Mode Enable"]
pub type WenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Window n Interrupt Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wintselselect {
    #[doc = "0: Interrupt on signal above window"]
    Above = 0,
    #[doc = "1: Interrupt on signal inside window"]
    Inside = 1,
    #[doc = "2: Interrupt on signal below window"]
    Below = 2,
    #[doc = "3: Interrupt on signal outside window"]
    Outside = 3,
}
impl From<Wintselselect> for u8 {
    #[inline(always)]
    fn from(variant: Wintselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wintselselect {
    type Ux = u8;
}
impl crate::IsEnum for Wintselselect {}
#[doc = "Field `WINTSEL` reader - Window n Interrupt Selection"]
pub type WintselR = crate::FieldReader<Wintselselect>;
impl WintselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wintselselect {
        match self.bits {
            0 => Wintselselect::Above,
            1 => Wintselselect::Inside,
            2 => Wintselselect::Below,
            3 => Wintselselect::Outside,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt on signal above window"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == Wintselselect::Above
    }
    #[doc = "Interrupt on signal inside window"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == Wintselselect::Inside
    }
    #[doc = "Interrupt on signal below window"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == Wintselselect::Below
    }
    #[doc = "Interrupt on signal outside window"]
    #[inline(always)]
    pub fn is_outside(&self) -> bool {
        *self == Wintselselect::Outside
    }
}
#[doc = "Field `WINTSEL` writer - Window n Interrupt Selection"]
pub type WintselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wintselselect, crate::Safe>;
impl<'a, REG> WintselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt on signal above window"]
    #[inline(always)]
    pub fn above(self) -> &'a mut crate::W<REG> {
        self.variant(Wintselselect::Above)
    }
    #[doc = "Interrupt on signal inside window"]
    #[inline(always)]
    pub fn inside(self) -> &'a mut crate::W<REG> {
        self.variant(Wintselselect::Inside)
    }
    #[doc = "Interrupt on signal below window"]
    #[inline(always)]
    pub fn below(self) -> &'a mut crate::W<REG> {
        self.variant(Wintselselect::Below)
    }
    #[doc = "Interrupt on signal outside window"]
    #[inline(always)]
    pub fn outside(self) -> &'a mut crate::W<REG> {
        self.variant(Wintselselect::Outside)
    }
}
impl R {
    #[doc = "Bit 0 - Window n Mode Enable"]
    #[inline(always)]
    pub fn wen(&self) -> WenR {
        WenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Window n Interrupt Selection"]
    #[inline(always)]
    pub fn wintsel(&self) -> WintselR {
        WintselR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Window n Mode Enable"]
    #[inline(always)]
    pub fn wen(&mut self) -> WenW<'_, WinctrlSpec> {
        WenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Window n Interrupt Selection"]
    #[inline(always)]
    pub fn wintsel(&mut self) -> WintselW<'_, WinctrlSpec> {
        WintselW::new(self, 1)
    }
}
#[doc = "Window Monitor Control\n\nYou can [`read`](crate::Reg::read) this register and get [`winctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinctrlSpec;
impl crate::RegisterSpec for WinctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`winctrl::R`](R) reader structure"]
impl crate::Readable for WinctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`winctrl::W`](W) writer structure"]
impl crate::Writable for WinctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WINCTRL to value 0"]
impl crate::Resettable for WinctrlSpec {}
