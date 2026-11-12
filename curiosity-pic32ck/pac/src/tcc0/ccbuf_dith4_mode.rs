#[doc = "Register `CCBUF_DITH4_MODE[%s]` reader"]
pub type R = crate::R<CcbufDith4ModeSpec>;
#[doc = "Register `CCBUF_DITH4_MODE[%s]` writer"]
pub type W = crate::W<CcbufDith4ModeSpec>;
#[doc = "Field `DITHERBUF` reader - Channel Compare/Capture Buffer Value"]
pub type DitherbufR = crate::FieldReader;
#[doc = "Field `DITHERBUF` writer - Channel Compare/Capture Buffer Value"]
pub type DitherbufW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CCBUF` reader - Dithering Buffer Cycle Number"]
pub type CcbufR = crate::FieldReader<u32>;
#[doc = "Field `CCBUF` writer - Dithering Buffer Cycle Number"]
pub type CcbufW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ditherbuf(&self) -> DitherbufR {
        DitherbufR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ccbuf(&self) -> CcbufR {
        CcbufR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ditherbuf(&mut self) -> DitherbufW<'_, CcbufDith4ModeSpec> {
        DitherbufW::new(self, 0)
    }
    #[doc = "Bits 4:31 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ccbuf(&mut self) -> CcbufW<'_, CcbufDith4ModeSpec> {
        CcbufW::new(self, 4)
    }
}
#[doc = "Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccbuf_dith4_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccbuf_dith4_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcbufDith4ModeSpec;
impl crate::RegisterSpec for CcbufDith4ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccbuf_dith4_mode::R`](R) reader structure"]
impl crate::Readable for CcbufDith4ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`ccbuf_dith4_mode::W`](W) writer structure"]
impl crate::Writable for CcbufDith4ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCBUF_DITH4_MODE[%s] to value 0"]
impl crate::Resettable for CcbufDith4ModeSpec {}
