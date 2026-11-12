#[doc = "Register `CTRLD` reader"]
pub type R = crate::R<CtrldSpec>;
#[doc = "Register `CTRLD` writer"]
pub type W = crate::W<CtrldSpec>;
#[doc = "Field `CTLCKDIV` reader - Control Clock Divider"]
pub type CtlckdivR = crate::FieldReader;
#[doc = "Field `CTLCKDIV` writer - Control Clock Divider"]
pub type CtlckdivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CHNEN` reader - Digital Channel Enable"]
pub type ChnenR = crate::FieldReader;
#[doc = "Field `CHNEN` writer - Digital Channel Enable"]
pub type ChnenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ANLEN` reader - Analog Channel Enable"]
pub type AnlenR = crate::FieldReader;
#[doc = "Field `ANLEN` writer - Analog Channel Enable"]
pub type AnlenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WKUPEXP` reader - Wakeup cycles"]
pub type WkupexpR = crate::FieldReader;
#[doc = "Field `WKUPEXP` writer - Wakeup cycles"]
pub type WkupexpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Voltage Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vrefselselect {
    #[doc = "0: AVDD and AVSS"]
    AvddAvss = 0,
    #[doc = "1: External VREFH and AVSS"]
    ExternalVrefhAvss = 1,
}
impl From<Vrefselselect> for u8 {
    #[inline(always)]
    fn from(variant: Vrefselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vrefselselect {
    type Ux = u8;
}
impl crate::IsEnum for Vrefselselect {}
#[doc = "Field `VREFSEL` reader - Voltage Reference Select"]
pub type VrefselR = crate::FieldReader<Vrefselselect>;
impl VrefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vrefselselect> {
        match self.bits {
            0 => Some(Vrefselselect::AvddAvss),
            1 => Some(Vrefselselect::ExternalVrefhAvss),
            _ => None,
        }
    }
    #[doc = "AVDD and AVSS"]
    #[inline(always)]
    pub fn is_avdd_avss(&self) -> bool {
        *self == Vrefselselect::AvddAvss
    }
    #[doc = "External VREFH and AVSS"]
    #[inline(always)]
    pub fn is_external_vrefh_avss(&self) -> bool {
        *self == Vrefselselect::ExternalVrefhAvss
    }
}
#[doc = "Field `VREFSEL` writer - Voltage Reference Select"]
pub type VrefselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Vrefselselect>;
impl<'a, REG> VrefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AVDD and AVSS"]
    #[inline(always)]
    pub fn avdd_avss(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefselselect::AvddAvss)
    }
    #[doc = "External VREFH and AVSS"]
    #[inline(always)]
    pub fn external_vrefh_avss(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefselselect::ExternalVrefhAvss)
    }
}
impl R {
    #[doc = "Bits 8:13 - Control Clock Divider"]
    #[inline(always)]
    pub fn ctlckdiv(&self) -> CtlckdivR {
        CtlckdivR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - Digital Channel Enable"]
    #[inline(always)]
    pub fn chnen(&self) -> ChnenR {
        ChnenR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Analog Channel Enable"]
    #[inline(always)]
    pub fn anlen(&self) -> AnlenR {
        AnlenR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Wakeup cycles"]
    #[inline(always)]
    pub fn wkupexp(&self) -> WkupexpR {
        WkupexpR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Voltage Reference Select"]
    #[inline(always)]
    pub fn vrefsel(&self) -> VrefselR {
        VrefselR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:13 - Control Clock Divider"]
    #[inline(always)]
    pub fn ctlckdiv(&mut self) -> CtlckdivW<'_, CtrldSpec> {
        CtlckdivW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Digital Channel Enable"]
    #[inline(always)]
    pub fn chnen(&mut self) -> ChnenW<'_, CtrldSpec> {
        ChnenW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Analog Channel Enable"]
    #[inline(always)]
    pub fn anlen(&mut self) -> AnlenW<'_, CtrldSpec> {
        AnlenW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Wakeup cycles"]
    #[inline(always)]
    pub fn wkupexp(&mut self) -> WkupexpW<'_, CtrldSpec> {
        WkupexpW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Voltage Reference Select"]
    #[inline(always)]
    pub fn vrefsel(&mut self) -> VrefselW<'_, CtrldSpec> {
        VrefselW::new(self, 28)
    }
}
#[doc = "Control D Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrldSpec;
impl crate::RegisterSpec for CtrldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrld::R`](R) reader structure"]
impl crate::Readable for CtrldSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrld::W`](W) writer structure"]
impl crate::Writable for CtrldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLD to value 0"]
impl crate::Resettable for CtrldSpec {}
