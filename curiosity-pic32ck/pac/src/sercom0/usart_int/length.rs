#[doc = "Register `LENGTH` reader"]
pub type R = crate::R<LengthSpec>;
#[doc = "Register `LENGTH` writer"]
pub type W = crate::W<LengthSpec>;
#[doc = "Field `LEN` reader - Data Length"]
pub type LenR = crate::FieldReader;
#[doc = "Field `LEN` writer - Data Length"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Data Length Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lenenselect {
    #[doc = "0: Length counter is disabled"]
    Disable = 0,
    #[doc = "1: Length counter is enabled for transmit"]
    Tx = 1,
    #[doc = "2: Length counter is enabled for receive"]
    Rx = 2,
}
impl From<Lenenselect> for u8 {
    #[inline(always)]
    fn from(variant: Lenenselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lenenselect {
    type Ux = u8;
}
impl crate::IsEnum for Lenenselect {}
#[doc = "Field `LENEN` reader - Data Length Enable"]
pub type LenenR = crate::FieldReader<Lenenselect>;
impl LenenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lenenselect> {
        match self.bits {
            0 => Some(Lenenselect::Disable),
            1 => Some(Lenenselect::Tx),
            2 => Some(Lenenselect::Rx),
            _ => None,
        }
    }
    #[doc = "Length counter is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lenenselect::Disable
    }
    #[doc = "Length counter is enabled for transmit"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == Lenenselect::Tx
    }
    #[doc = "Length counter is enabled for receive"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == Lenenselect::Rx
    }
}
#[doc = "Field `LENEN` writer - Data Length Enable"]
pub type LenenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lenenselect>;
impl<'a, REG> LenenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Length counter is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lenenselect::Disable)
    }
    #[doc = "Length counter is enabled for transmit"]
    #[inline(always)]
    pub fn tx(self) -> &'a mut crate::W<REG> {
        self.variant(Lenenselect::Tx)
    }
    #[doc = "Length counter is enabled for receive"]
    #[inline(always)]
    pub fn rx(self) -> &'a mut crate::W<REG> {
        self.variant(Lenenselect::Rx)
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Data Length Enable"]
    #[inline(always)]
    pub fn lenen(&self) -> LenenR {
        LenenR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<'_, LengthSpec> {
        LenW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Data Length Enable"]
    #[inline(always)]
    pub fn lenen(&mut self) -> LenenW<'_, LengthSpec> {
        LenenW::new(self, 8)
    }
}
#[doc = "USART Length\n\nYou can [`read`](crate::Reg::read) this register and get [`length::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`length::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LengthSpec;
impl crate::RegisterSpec for LengthSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`length::R`](R) reader structure"]
impl crate::Readable for LengthSpec {}
#[doc = "`write(|w| ..)` method takes [`length::W`](W) writer structure"]
impl crate::Writable for LengthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LENGTH to value 0"]
impl crate::Resettable for LengthSpec {}
