#[doc = "Register `CFGA` reader"]
pub type R = crate::R<CfgaSpec>;
#[doc = "Register `CFGA` writer"]
pub type W = crate::W<CfgaSpec>;
#[doc = "Field `REFNUM` reader - Number of Reference Clock Cycles"]
pub type RefnumR = crate::FieldReader;
#[doc = "Field `REFNUM` writer - Number of Reference Clock Cycles"]
pub type RefnumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Measurement Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msrselselect {
    #[doc = "0: GCLK Input Clock"]
    Gclk = 0,
    #[doc = "1: CPU Input Clock"]
    Cpu = 1,
}
impl From<Msrselselect> for u8 {
    #[inline(always)]
    fn from(variant: Msrselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msrselselect {
    type Ux = u8;
}
impl crate::IsEnum for Msrselselect {}
#[doc = "Field `MSRSEL` reader - Measurement Clock Selection"]
pub type MsrselR = crate::FieldReader<Msrselselect>;
impl MsrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Msrselselect> {
        match self.bits {
            0 => Some(Msrselselect::Gclk),
            1 => Some(Msrselselect::Cpu),
            _ => None,
        }
    }
    #[doc = "GCLK Input Clock"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == Msrselselect::Gclk
    }
    #[doc = "CPU Input Clock"]
    #[inline(always)]
    pub fn is_cpu(&self) -> bool {
        *self == Msrselselect::Cpu
    }
}
#[doc = "Field `MSRSEL` writer - Measurement Clock Selection"]
pub type MsrselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Msrselselect>;
impl<'a, REG> MsrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GCLK Input Clock"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut crate::W<REG> {
        self.variant(Msrselselect::Gclk)
    }
    #[doc = "CPU Input Clock"]
    #[inline(always)]
    pub fn cpu(self) -> &'a mut crate::W<REG> {
        self.variant(Msrselselect::Cpu)
    }
}
#[doc = "Divide Reference Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Divrefselect {
    #[doc = "0: The reference clock is divided by 1"]
    Div1 = 0,
    #[doc = "1: The reference clock is divided by 8"]
    Div8 = 1,
}
impl From<Divrefselect> for bool {
    #[inline(always)]
    fn from(variant: Divrefselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVREF` reader - Divide Reference Clock"]
pub type DivrefR = crate::BitReader<Divrefselect>;
impl DivrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Divrefselect {
        match self.bits {
            false => Divrefselect::Div1,
            true => Divrefselect::Div8,
        }
    }
    #[doc = "The reference clock is divided by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Divrefselect::Div1
    }
    #[doc = "The reference clock is divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Divrefselect::Div8
    }
}
#[doc = "Field `DIVREF` writer - Divide Reference Clock"]
pub type DivrefW<'a, REG> = crate::BitWriter<'a, REG, Divrefselect>;
impl<'a, REG> DivrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The reference clock is divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Divrefselect::Div1)
    }
    #[doc = "The reference clock is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Divrefselect::Div8)
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of Reference Clock Cycles"]
    #[inline(always)]
    pub fn refnum(&self) -> RefnumR {
        RefnumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Measurement Clock Selection"]
    #[inline(always)]
    pub fn msrsel(&self) -> MsrselR {
        MsrselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Divide Reference Clock"]
    #[inline(always)]
    pub fn divref(&self) -> DivrefR {
        DivrefR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of Reference Clock Cycles"]
    #[inline(always)]
    pub fn refnum(&mut self) -> RefnumW<'_, CfgaSpec> {
        RefnumW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Measurement Clock Selection"]
    #[inline(always)]
    pub fn msrsel(&mut self) -> MsrselW<'_, CfgaSpec> {
        MsrselW::new(self, 8)
    }
    #[doc = "Bit 15 - Divide Reference Clock"]
    #[inline(always)]
    pub fn divref(&mut self) -> DivrefW<'_, CfgaSpec> {
        DivrefW::new(self, 15)
    }
}
#[doc = "Config A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfga::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfga::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgaSpec;
impl crate::RegisterSpec for CfgaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfga::R`](R) reader structure"]
impl crate::Readable for CfgaSpec {}
#[doc = "`write(|w| ..)` method takes [`cfga::W`](W) writer structure"]
impl crate::Writable for CfgaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGA to value 0"]
impl crate::Resettable for CfgaSpec {}
