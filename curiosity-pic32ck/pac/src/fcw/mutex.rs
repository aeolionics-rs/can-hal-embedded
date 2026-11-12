#[doc = "Register `MUTEX` reader"]
pub type R = crate::R<MutexSpec>;
#[doc = "Register `MUTEX` writer"]
pub type W = crate::W<MutexSpec>;
#[doc = "Field `LOCK` reader - Flash Write Controller (FCW) Lock by Owner"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Flash Write Controller (FCW) Lock by Owner"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OWNER` reader - Flash Write Controller (FCW) Owner ID"]
pub type OwnerR = crate::FieldReader;
#[doc = "Field `OWNER` writer - Flash Write Controller (FCW) Owner ID"]
pub type OwnerW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Flash Write Controller (FCW) Lock by Owner"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Flash Write Controller (FCW) Owner ID"]
    #[inline(always)]
    pub fn owner(&self) -> OwnerR {
        OwnerR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Write Controller (FCW) Lock by Owner"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, MutexSpec> {
        LockW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Flash Write Controller (FCW) Owner ID"]
    #[inline(always)]
    pub fn owner(&mut self) -> OwnerW<'_, MutexSpec> {
        OwnerW::new(self, 1)
    }
}
#[doc = "NVM MUTEX Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MutexSpec;
impl crate::RegisterSpec for MutexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mutex::R`](R) reader structure"]
impl crate::Readable for MutexSpec {}
#[doc = "`write(|w| ..)` method takes [`mutex::W`](W) writer structure"]
impl crate::Writable for MutexSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MUTEX to value 0"]
impl crate::Resettable for MutexSpec {}
