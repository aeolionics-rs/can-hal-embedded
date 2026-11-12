#[doc = "Register `MBFI0` reader"]
pub type R = crate::R<Mbfi0Spec>;
#[doc = "Register `MBFI0` writer"]
pub type W = crate::W<Mbfi0Spec>;
#[doc = "Field `BIDX` reader - Bit Index of injected fault"]
pub type BidxR = crate::FieldReader;
#[doc = "Field `BIDX` writer - Bit Index of injected fault"]
pub type BidxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Address matching mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ammodselect {
    #[doc = "0: Address match, fault injected when the masked master address matches with the masked bcc1.addr"]
    Addr = 0,
    #[doc = "1: Always matches, fault injected every AHB access"]
    Always = 1,
}
impl From<Ammodselect> for bool {
    #[inline(always)]
    fn from(variant: Ammodselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMMOD` reader - Address matching mode"]
pub type AmmodR = crate::BitReader<Ammodselect>;
impl AmmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ammodselect {
        match self.bits {
            false => Ammodselect::Addr,
            true => Ammodselect::Always,
        }
    }
    #[doc = "Address match, fault injected when the masked master address matches with the masked bcc1.addr"]
    #[inline(always)]
    pub fn is_addr(&self) -> bool {
        *self == Ammodselect::Addr
    }
    #[doc = "Always matches, fault injected every AHB access"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == Ammodselect::Always
    }
}
#[doc = "Field `AMMOD` writer - Address matching mode"]
pub type AmmodW<'a, REG> = crate::BitWriter<'a, REG, Ammodselect>;
impl<'a, REG> AmmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address match, fault injected when the masked master address matches with the masked bcc1.addr"]
    #[inline(always)]
    pub fn addr(self) -> &'a mut crate::W<REG> {
        self.variant(Ammodselect::Addr)
    }
    #[doc = "Always matches, fault injected every AHB access"]
    #[inline(always)]
    pub fn always(self) -> &'a mut crate::W<REG> {
        self.variant(Ammodselect::Always)
    }
}
#[doc = "Fault type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ftypeselect {
    #[doc = "0: Stuck At 0"]
    Stuckat0 = 0,
    #[doc = "1: Stuck At 1"]
    Stuckat1 = 1,
}
impl From<Ftypeselect> for bool {
    #[inline(always)]
    fn from(variant: Ftypeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTYPE` reader - Fault type"]
pub type FtypeR = crate::BitReader<Ftypeselect>;
impl FtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ftypeselect {
        match self.bits {
            false => Ftypeselect::Stuckat0,
            true => Ftypeselect::Stuckat1,
        }
    }
    #[doc = "Stuck At 0"]
    #[inline(always)]
    pub fn is_stuckat0(&self) -> bool {
        *self == Ftypeselect::Stuckat0
    }
    #[doc = "Stuck At 1"]
    #[inline(always)]
    pub fn is_stuckat1(&self) -> bool {
        *self == Ftypeselect::Stuckat1
    }
}
#[doc = "Field `FTYPE` writer - Fault type"]
pub type FtypeW<'a, REG> = crate::BitWriter<'a, REG, Ftypeselect>;
impl<'a, REG> FtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stuck At 0"]
    #[inline(always)]
    pub fn stuckat0(self) -> &'a mut crate::W<REG> {
        self.variant(Ftypeselect::Stuckat0)
    }
    #[doc = "Stuck At 1"]
    #[inline(always)]
    pub fn stuckat1(self) -> &'a mut crate::W<REG> {
        self.variant(Ftypeselect::Stuckat1)
    }
}
#[doc = "Field `AMMSK` reader - Address matching Mask (Word address)"]
pub type AmmskR = crate::FieldReader<u32>;
#[doc = "Field `AMMSK` writer - Address matching Mask (Word address)"]
pub type AmmskW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:4 - Bit Index of injected fault"]
    #[inline(always)]
    pub fn bidx(&self) -> BidxR {
        BidxR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Address matching mode"]
    #[inline(always)]
    pub fn ammod(&self) -> AmmodR {
        AmmodR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fault type"]
    #[inline(always)]
    pub fn ftype(&self) -> FtypeR {
        FtypeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Address matching Mask (Word address)"]
    #[inline(always)]
    pub fn ammsk(&self) -> AmmskR {
        AmmskR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bit Index of injected fault"]
    #[inline(always)]
    pub fn bidx(&mut self) -> BidxW<'_, Mbfi0Spec> {
        BidxW::new(self, 0)
    }
    #[doc = "Bit 6 - Address matching mode"]
    #[inline(always)]
    pub fn ammod(&mut self) -> AmmodW<'_, Mbfi0Spec> {
        AmmodW::new(self, 6)
    }
    #[doc = "Bit 7 - Fault type"]
    #[inline(always)]
    pub fn ftype(&mut self) -> FtypeW<'_, Mbfi0Spec> {
        FtypeW::new(self, 7)
    }
    #[doc = "Bits 8:31 - Address matching Mask (Word address)"]
    #[inline(always)]
    pub fn ammsk(&mut self) -> AmmskW<'_, Mbfi0Spec> {
        AmmskW::new(self, 8)
    }
}
#[doc = "Memory Bist Fault Injection 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mbfi0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbfi0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mbfi0Spec;
impl crate::RegisterSpec for Mbfi0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbfi0::R`](R) reader structure"]
impl crate::Readable for Mbfi0Spec {}
#[doc = "`write(|w| ..)` method takes [`mbfi0::W`](W) writer structure"]
impl crate::Writable for Mbfi0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBFI0 to value 0"]
impl crate::Resettable for Mbfi0Spec {}
