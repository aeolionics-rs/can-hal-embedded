#[doc = "Register `CORCHDATAID` reader"]
pub type R = crate::R<CorchdataidSpec>;
#[doc = "Register `CORCHDATAID` writer"]
pub type W = crate::W<CorchdataidSpec>;
#[doc = "Field `CHRDYID` reader - Channel Read ID"]
pub type ChrdyidR = crate::FieldReader;
#[doc = "Field `CHRDYID` writer - Channel Read ID"]
pub type ChrdyidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CORDYID` reader - Core Read ID"]
pub type CordyidR = crate::FieldReader;
#[doc = "Field `CORDYID` writer - Core Read ID"]
pub type CordyidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Channel Read ID"]
    #[inline(always)]
    pub fn chrdyid(&self) -> ChrdyidR {
        ChrdyidR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Core Read ID"]
    #[inline(always)]
    pub fn cordyid(&self) -> CordyidR {
        CordyidR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Read ID"]
    #[inline(always)]
    pub fn chrdyid(&mut self) -> ChrdyidW<'_, CorchdataidSpec> {
        ChrdyidW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Core Read ID"]
    #[inline(always)]
    pub fn cordyid(&mut self) -> CordyidW<'_, CorchdataidSpec> {
        CordyidW::new(self, 4)
    }
}
#[doc = "Channel Ready DATA ID\n\nYou can [`read`](crate::Reg::read) this register and get [`corchdataid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`corchdataid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CorchdataidSpec;
impl crate::RegisterSpec for CorchdataidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`corchdataid::R`](R) reader structure"]
impl crate::Readable for CorchdataidSpec {}
#[doc = "`write(|w| ..)` method takes [`corchdataid::W`](W) writer structure"]
impl crate::Writable for CorchdataidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORCHDATAID to value 0"]
impl crate::Resettable for CorchdataidSpec {}
