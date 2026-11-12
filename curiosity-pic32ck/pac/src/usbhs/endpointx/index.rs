#[doc = "Register `INDEX` reader"]
pub type R = crate::R<IndexSpec>;
#[doc = "Register `INDEX` writer"]
pub type W = crate::W<IndexSpec>;
#[doc = "The Selected Endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selepselect {
    #[doc = "0: Endpoint 0 Select"]
    Ep0 = 0,
    #[doc = "1: Endpoint 1 Select"]
    Ep1 = 1,
    #[doc = "2: Endpoint 2 Select"]
    Ep2 = 2,
    #[doc = "3: Endpoint 3 Select"]
    Ep3 = 3,
    #[doc = "4: Endpoint 4 Select"]
    Ep4 = 4,
    #[doc = "5: Endpoint 5 Select"]
    Ep5 = 5,
    #[doc = "6: Endpoint 6 Select"]
    Ep6 = 6,
    #[doc = "7: Endpoint 7 Select"]
    Ep7 = 7,
    #[doc = "8: Endpoint 8 Select"]
    Ep8 = 8,
}
impl From<Selepselect> for u8 {
    #[inline(always)]
    fn from(variant: Selepselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selepselect {
    type Ux = u8;
}
impl crate::IsEnum for Selepselect {}
#[doc = "Field `SELEP` reader - The Selected Endpoint"]
pub type SelepR = crate::FieldReader<Selepselect>;
impl SelepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Selepselect> {
        match self.bits {
            0 => Some(Selepselect::Ep0),
            1 => Some(Selepselect::Ep1),
            2 => Some(Selepselect::Ep2),
            3 => Some(Selepselect::Ep3),
            4 => Some(Selepselect::Ep4),
            5 => Some(Selepselect::Ep5),
            6 => Some(Selepselect::Ep6),
            7 => Some(Selepselect::Ep7),
            8 => Some(Selepselect::Ep8),
            _ => None,
        }
    }
    #[doc = "Endpoint 0 Select"]
    #[inline(always)]
    pub fn is_ep0(&self) -> bool {
        *self == Selepselect::Ep0
    }
    #[doc = "Endpoint 1 Select"]
    #[inline(always)]
    pub fn is_ep1(&self) -> bool {
        *self == Selepselect::Ep1
    }
    #[doc = "Endpoint 2 Select"]
    #[inline(always)]
    pub fn is_ep2(&self) -> bool {
        *self == Selepselect::Ep2
    }
    #[doc = "Endpoint 3 Select"]
    #[inline(always)]
    pub fn is_ep3(&self) -> bool {
        *self == Selepselect::Ep3
    }
    #[doc = "Endpoint 4 Select"]
    #[inline(always)]
    pub fn is_ep4(&self) -> bool {
        *self == Selepselect::Ep4
    }
    #[doc = "Endpoint 5 Select"]
    #[inline(always)]
    pub fn is_ep5(&self) -> bool {
        *self == Selepselect::Ep5
    }
    #[doc = "Endpoint 6 Select"]
    #[inline(always)]
    pub fn is_ep6(&self) -> bool {
        *self == Selepselect::Ep6
    }
    #[doc = "Endpoint 7 Select"]
    #[inline(always)]
    pub fn is_ep7(&self) -> bool {
        *self == Selepselect::Ep7
    }
    #[doc = "Endpoint 8 Select"]
    #[inline(always)]
    pub fn is_ep8(&self) -> bool {
        *self == Selepselect::Ep8
    }
}
#[doc = "Field `SELEP` writer - The Selected Endpoint"]
pub type SelepW<'a, REG> = crate::FieldWriter<'a, REG, 4, Selepselect>;
impl<'a, REG> SelepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Endpoint 0 Select"]
    #[inline(always)]
    pub fn ep0(self) -> &'a mut crate::W<REG> {
        self.variant(Selepselect::Ep0)
    }
    #[doc = "Endpoint 1 Select"]
    #[inline(always)]
    pub fn ep1(self) -> &'a mut crate::W<REG> {
        self.variant(Selepselect::Ep1)
    }
    #[doc = "Endpoint 2 Select"]
    #[inline(always)]
    pub fn ep2(self) -> &'a mut crate::W<REG> {
        self.variant(Selepselect::Ep2)
    }
    #[doc = "Endpoint 3 Select"]
    #[inline(always)]
    pub fn ep3(self) -> &'a mut crate::W<REG> {
        self.variant(Selepselect::Ep3)
    }
    #[doc = "Endpoint 4 Select"]
    #[inline(always)]
    pub fn ep4(self) -> &'a mut crate::W<REG> {
        self.variant(Selepselect::Ep4)
    }
    #[doc = "Endpoint 5 Select"]
    #[inline(always)]
    pub fn ep5(self) -> &'a mut crate::W<REG> {
        self.variant(Selepselect::Ep5)
    }
    #[doc = "Endpoint 6 Select"]
    #[inline(always)]
    pub fn ep6(self) -> &'a mut crate::W<REG> {
        self.variant(Selepselect::Ep6)
    }
    #[doc = "Endpoint 7 Select"]
    #[inline(always)]
    pub fn ep7(self) -> &'a mut crate::W<REG> {
        self.variant(Selepselect::Ep7)
    }
    #[doc = "Endpoint 8 Select"]
    #[inline(always)]
    pub fn ep8(self) -> &'a mut crate::W<REG> {
        self.variant(Selepselect::Ep8)
    }
}
impl R {
    #[doc = "Bits 0:3 - The Selected Endpoint"]
    #[inline(always)]
    pub fn selep(&self) -> SelepR {
        SelepR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - The Selected Endpoint"]
    #[inline(always)]
    pub fn selep(&mut self) -> SelepW<'_, IndexSpec> {
        SelepW::new(self, 0)
    }
}
#[doc = "Index Is A 4-Bit Register That Determines Which Endpoint Control/Status Registers Are Accessed\n\nYou can [`read`](crate::Reg::read) this register and get [`index::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`index::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndexSpec;
impl crate::RegisterSpec for IndexSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`index::R`](R) reader structure"]
impl crate::Readable for IndexSpec {}
#[doc = "`write(|w| ..)` method takes [`index::W`](W) writer structure"]
impl crate::Writable for IndexSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INDEX to value 0"]
impl crate::Resettable for IndexSpec {}
