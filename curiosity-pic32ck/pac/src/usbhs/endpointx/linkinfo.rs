#[doc = "Register `LINKINFO` reader"]
pub type R = crate::R<LinkinfoSpec>;
#[doc = "Register `LINKINFO` writer"]
pub type W = crate::W<LinkinfoSpec>;
#[doc = "Field `WTID` reader - sets the delay to be applied from idpullup being asserted to iddig being considered valid in units of 4.369ms"]
pub type WtidR = crate::FieldReader;
#[doc = "Field `WTID` writer - sets the delay to be applied from idpullup being asserted to iddig being considered valid in units of 4.369ms"]
pub type WtidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTCON` reader - sets the wait to be applied to allow for the user's connect/disconnect filter in units of 533.3ns"]
pub type WtconR = crate::FieldReader;
#[doc = "Field `WTCON` writer - sets the wait to be applied to allow for the user's connect/disconnect filter in units of 533.3ns"]
pub type WtconW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - sets the delay to be applied from idpullup being asserted to iddig being considered valid in units of 4.369ms"]
    #[inline(always)]
    pub fn wtid(&self) -> WtidR {
        WtidR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - sets the wait to be applied to allow for the user's connect/disconnect filter in units of 533.3ns"]
    #[inline(always)]
    pub fn wtcon(&self) -> WtconR {
        WtconR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - sets the delay to be applied from idpullup being asserted to iddig being considered valid in units of 4.369ms"]
    #[inline(always)]
    pub fn wtid(&mut self) -> WtidW<'_, LinkinfoSpec> {
        WtidW::new(self, 0)
    }
    #[doc = "Bits 4:7 - sets the wait to be applied to allow for the user's connect/disconnect filter in units of 533.3ns"]
    #[inline(always)]
    pub fn wtcon(&mut self) -> WtconW<'_, LinkinfoSpec> {
        WtconW::new(self, 4)
    }
}
#[doc = "Allows Some Delays To Be Specified\n\nYou can [`read`](crate::Reg::read) this register and get [`linkinfo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linkinfo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinkinfoSpec;
impl crate::RegisterSpec for LinkinfoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`linkinfo::R`](R) reader structure"]
impl crate::Readable for LinkinfoSpec {}
#[doc = "`write(|w| ..)` method takes [`linkinfo::W`](W) writer structure"]
impl crate::Writable for LinkinfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINKINFO to value 0x5c"]
impl crate::Resettable for LinkinfoSpec {
    const RESET_VALUE: u8 = 0x5c;
}
