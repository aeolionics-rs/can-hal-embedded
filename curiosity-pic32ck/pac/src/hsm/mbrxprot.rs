#[doc = "Register `MBRXPROT` reader"]
pub type R = crate::R<MbrxprotSpec>;
#[doc = "Register `MBRXPROT` writer"]
pub type W = crate::W<MbrxprotSpec>;
#[doc = "Field `UNPROT` reader - MAILBOX UNPROTECTED"]
pub type UnprotR = crate::BitReader;
#[doc = "Field `UNPROT` writer - MAILBOX UNPROTECTED"]
pub type UnprotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV` reader - MAILBOX MESSAGE PRIVILEDGED ACCESS"]
pub type PrivR = crate::BitReader;
#[doc = "Field `PRIV` writer - MAILBOX MESSAGE PRIVILEDGED ACCESS"]
pub type PrivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSEC` reader - MAILBOX MESSAGE NON-SECURE ACCESS"]
pub type NsecR = crate::BitReader;
#[doc = "Field `NSEC` writer - MAILBOX MESSAGE NON-SECURE ACCESS"]
pub type NsecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USER` reader - USER SIDEBAND"]
pub type UserR = crate::FieldReader;
#[doc = "Field `USER` writer - USER SIDEBAND"]
pub type UserW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 21 - MAILBOX UNPROTECTED"]
    #[inline(always)]
    pub fn unprot(&self) -> UnprotR {
        UnprotR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MAILBOX MESSAGE PRIVILEDGED ACCESS"]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MAILBOX MESSAGE NON-SECURE ACCESS"]
    #[inline(always)]
    pub fn nsec(&self) -> NsecR {
        NsecR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - USER SIDEBAND"]
    #[inline(always)]
    pub fn user(&self) -> UserR {
        UserR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 21 - MAILBOX UNPROTECTED"]
    #[inline(always)]
    pub fn unprot(&mut self) -> UnprotW<'_, MbrxprotSpec> {
        UnprotW::new(self, 21)
    }
    #[doc = "Bit 22 - MAILBOX MESSAGE PRIVILEDGED ACCESS"]
    #[inline(always)]
    pub fn priv_(&mut self) -> PrivW<'_, MbrxprotSpec> {
        PrivW::new(self, 22)
    }
    #[doc = "Bit 23 - MAILBOX MESSAGE NON-SECURE ACCESS"]
    #[inline(always)]
    pub fn nsec(&mut self) -> NsecW<'_, MbrxprotSpec> {
        NsecW::new(self, 23)
    }
    #[doc = "Bits 24:31 - USER SIDEBAND"]
    #[inline(always)]
    pub fn user(&mut self) -> UserW<'_, MbrxprotSpec> {
        UserW::new(self, 24)
    }
}
#[doc = "MAILBOX RX PROTECTION\n\nYou can [`read`](crate::Reg::read) this register and get [`mbrxprot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbrxprot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbrxprotSpec;
impl crate::RegisterSpec for MbrxprotSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbrxprot::R`](R) reader structure"]
impl crate::Readable for MbrxprotSpec {}
#[doc = "`write(|w| ..)` method takes [`mbrxprot::W`](W) writer structure"]
impl crate::Writable for MbrxprotSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBRXPROT to value 0"]
impl crate::Resettable for MbrxprotSpec {}
