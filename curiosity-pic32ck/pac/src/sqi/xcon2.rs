#[doc = "Register `XCON2` reader"]
pub type R = crate::R<Xcon2Spec>;
#[doc = "Register `XCON2` writer"]
pub type W = crate::W<Xcon2Spec>;
#[doc = "Field `MODECODE` reader - 8bit value for Mode byte"]
pub type ModecodeR = crate::FieldReader;
#[doc = "Field `MODECODE` writer - 8bit value for Mode byte"]
pub type ModecodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MODEBYTES` reader - Mode Bytes 00 - 0 Mode Bytes , 01 - 1 Mode Byte , 10 - 2 Mode Bytes , 11 - 3 Mode Bytes"]
pub type ModebytesR = crate::FieldReader;
#[doc = "Field `MODEBYTES` writer - Mode Bytes 00 - 0 Mode Bytes , 01 - 1 Mode Byte , 10 - 2 Mode Bytes , 11 - 3 Mode Bytes"]
pub type ModebytesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DEVSEL` reader - XIP SPI Device Select: This field is used to select a particular SPI device 000 - Select Device0 , 001 - Select Device1 , 010 - Select Device2 , 011 - Select Device3 , 100 - Select Device4 , 101 - Select Device5 , 110 - Select Device6 , 111 - Select Device7"]
pub type DevselR = crate::FieldReader;
#[doc = "Field `DEVSEL` writer - XIP SPI Device Select: This field is used to select a particular SPI device 000 - Select Device0 , 001 - Select Device1 , 010 - Select Device2 , 011 - Select Device3 , 100 - Select Device4 , 101 - Select Device5 , 110 - Select Device6 , 111 - Select Device7"]
pub type DevselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - 8bit value for Mode byte"]
    #[inline(always)]
    pub fn modecode(&self) -> ModecodeR {
        ModecodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Mode Bytes 00 - 0 Mode Bytes , 01 - 1 Mode Byte , 10 - 2 Mode Bytes , 11 - 3 Mode Bytes"]
    #[inline(always)]
    pub fn modebytes(&self) -> ModebytesR {
        ModebytesR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - XIP SPI Device Select: This field is used to select a particular SPI device 000 - Select Device0 , 001 - Select Device1 , 010 - Select Device2 , 011 - Select Device3 , 100 - Select Device4 , 101 - Select Device5 , 110 - Select Device6 , 111 - Select Device7"]
    #[inline(always)]
    pub fn devsel(&self) -> DevselR {
        DevselR::new(((self.bits >> 10) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8bit value for Mode byte"]
    #[inline(always)]
    pub fn modecode(&mut self) -> ModecodeW<'_, Xcon2Spec> {
        ModecodeW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Mode Bytes 00 - 0 Mode Bytes , 01 - 1 Mode Byte , 10 - 2 Mode Bytes , 11 - 3 Mode Bytes"]
    #[inline(always)]
    pub fn modebytes(&mut self) -> ModebytesW<'_, Xcon2Spec> {
        ModebytesW::new(self, 8)
    }
    #[doc = "Bits 10:12 - XIP SPI Device Select: This field is used to select a particular SPI device 000 - Select Device0 , 001 - Select Device1 , 010 - Select Device2 , 011 - Select Device3 , 100 - Select Device4 , 101 - Select Device5 , 110 - Select Device6 , 111 - Select Device7"]
    #[inline(always)]
    pub fn devsel(&mut self) -> DevselW<'_, Xcon2Spec> {
        DevselW::new(self, 10)
    }
}
#[doc = "SPI XIP Control2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`xcon2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcon2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xcon2Spec;
impl crate::RegisterSpec for Xcon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xcon2::R`](R) reader structure"]
impl crate::Readable for Xcon2Spec {}
#[doc = "`write(|w| ..)` method takes [`xcon2::W`](W) writer structure"]
impl crate::Writable for Xcon2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XCON2 to value 0xa0"]
impl crate::Resettable for Xcon2Spec {
    const RESET_VALUE: u32 = 0xa0;
}
