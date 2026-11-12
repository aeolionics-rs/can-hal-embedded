#[doc = "Register `CA1R` reader"]
pub type R = crate::R<Ca1rSpec>;
#[doc = "Register `CA1R` writer"]
pub type W = crate::W<Ca1rSpec>;
#[doc = "Clock Multiplier\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkmultselect {
    #[doc = "0: Clock Multiplier is Not Supported"]
    No = 0,
}
impl From<Clkmultselect> for u8 {
    #[inline(always)]
    fn from(variant: Clkmultselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkmultselect {
    type Ux = u8;
}
impl crate::IsEnum for Clkmultselect {}
#[doc = "Field `CLKMULT` reader - Clock Multiplier"]
pub type ClkmultR = crate::FieldReader<Clkmultselect>;
impl ClkmultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkmultselect> {
        match self.bits {
            0 => Some(Clkmultselect::No),
            _ => None,
        }
    }
    #[doc = "Clock Multiplier is Not Supported"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Clkmultselect::No
    }
}
#[doc = "Field `CLKMULT` writer - Clock Multiplier"]
pub type ClkmultW<'a, REG> = crate::FieldWriter<'a, REG, 8, Clkmultselect>;
impl<'a, REG> ClkmultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock Multiplier is Not Supported"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Clkmultselect::No)
    }
}
impl R {
    #[doc = "Bits 16:23 - Clock Multiplier"]
    #[inline(always)]
    pub fn clkmult(&self) -> ClkmultR {
        ClkmultR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Clock Multiplier"]
    #[inline(always)]
    pub fn clkmult(&mut self) -> ClkmultW<'_, Ca1rSpec> {
        ClkmultW::new(self, 16)
    }
}
#[doc = "Capabilities 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ca1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ca1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ca1rSpec;
impl crate::RegisterSpec for Ca1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ca1r::R`](R) reader structure"]
impl crate::Readable for Ca1rSpec {}
#[doc = "`write(|w| ..)` method takes [`ca1r::W`](W) writer structure"]
impl crate::Writable for Ca1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CA1R to value 0x0001_0070"]
impl crate::Resettable for Ca1rSpec {
    const RESET_VALUE: u32 = 0x0001_0070;
}
