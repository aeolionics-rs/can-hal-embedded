#[doc = "Register `PATTBUF` reader"]
pub type R = crate::R<PattbufSpec>;
#[doc = "Register `PATTBUF` writer"]
pub type W = crate::W<PattbufSpec>;
#[doc = "Field `PGEB0` reader - Pattern Generation 0 Output Enable Buffer"]
pub type Pgeb0R = crate::BitReader;
#[doc = "Field `PGEB0` writer - Pattern Generation 0 Output Enable Buffer"]
pub type Pgeb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGEB1` reader - Pattern Generation 1 Output Enable Buffer"]
pub type Pgeb1R = crate::BitReader;
#[doc = "Field `PGEB1` writer - Pattern Generation 1 Output Enable Buffer"]
pub type Pgeb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGEB2` reader - Pattern Generation 2 Output Enable Buffer"]
pub type Pgeb2R = crate::BitReader;
#[doc = "Field `PGEB2` writer - Pattern Generation 2 Output Enable Buffer"]
pub type Pgeb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGEB3` reader - Pattern Generation 3 Output Enable Buffer"]
pub type Pgeb3R = crate::BitReader;
#[doc = "Field `PGEB3` writer - Pattern Generation 3 Output Enable Buffer"]
pub type Pgeb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGEB4` reader - Pattern Generation 4 Output Enable Buffer"]
pub type Pgeb4R = crate::BitReader;
#[doc = "Field `PGEB4` writer - Pattern Generation 4 Output Enable Buffer"]
pub type Pgeb4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGEB5` reader - Pattern Generation 5 Output Enable Buffer"]
pub type Pgeb5R = crate::BitReader;
#[doc = "Field `PGEB5` writer - Pattern Generation 5 Output Enable Buffer"]
pub type Pgeb5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGEB6` reader - Pattern Generation 6 Output Enable Buffer"]
pub type Pgeb6R = crate::BitReader;
#[doc = "Field `PGEB6` writer - Pattern Generation 6 Output Enable Buffer"]
pub type Pgeb6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGEB7` reader - Pattern Generation 7 Output Enable Buffer"]
pub type Pgeb7R = crate::BitReader;
#[doc = "Field `PGEB7` writer - Pattern Generation 7 Output Enable Buffer"]
pub type Pgeb7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGVB0` reader - Pattern Generation 0 Output Value Buffer"]
pub type Pgvb0R = crate::BitReader;
#[doc = "Field `PGVB0` writer - Pattern Generation 0 Output Value Buffer"]
pub type Pgvb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGVB1` reader - Pattern Generation 1 Output Value Buffer"]
pub type Pgvb1R = crate::BitReader;
#[doc = "Field `PGVB1` writer - Pattern Generation 1 Output Value Buffer"]
pub type Pgvb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGVB2` reader - Pattern Generation 2 Output Value Buffer"]
pub type Pgvb2R = crate::BitReader;
#[doc = "Field `PGVB2` writer - Pattern Generation 2 Output Value Buffer"]
pub type Pgvb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGVB3` reader - Pattern Generation 3 Output Value Buffer"]
pub type Pgvb3R = crate::BitReader;
#[doc = "Field `PGVB3` writer - Pattern Generation 3 Output Value Buffer"]
pub type Pgvb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGVB4` reader - Pattern Generation 4 Output Value Buffer"]
pub type Pgvb4R = crate::BitReader;
#[doc = "Field `PGVB4` writer - Pattern Generation 4 Output Value Buffer"]
pub type Pgvb4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGVB5` reader - Pattern Generation 5 Output Value Buffer"]
pub type Pgvb5R = crate::BitReader;
#[doc = "Field `PGVB5` writer - Pattern Generation 5 Output Value Buffer"]
pub type Pgvb5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGVB6` reader - Pattern Generation 6 Output Value Buffer"]
pub type Pgvb6R = crate::BitReader;
#[doc = "Field `PGVB6` writer - Pattern Generation 6 Output Value Buffer"]
pub type Pgvb6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGVB7` reader - Pattern Generation 7 Output Value Buffer"]
pub type Pgvb7R = crate::BitReader;
#[doc = "Field `PGVB7` writer - Pattern Generation 7 Output Value Buffer"]
pub type Pgvb7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pattern Generation 0 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb0(&self) -> Pgeb0R {
        Pgeb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pattern Generation 1 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb1(&self) -> Pgeb1R {
        Pgeb1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pattern Generation 2 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb2(&self) -> Pgeb2R {
        Pgeb2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pattern Generation 3 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb3(&self) -> Pgeb3R {
        Pgeb3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pattern Generation 4 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb4(&self) -> Pgeb4R {
        Pgeb4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pattern Generation 5 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb5(&self) -> Pgeb5R {
        Pgeb5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pattern Generation 6 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb6(&self) -> Pgeb6R {
        Pgeb6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pattern Generation 7 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb7(&self) -> Pgeb7R {
        Pgeb7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pattern Generation 0 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb0(&self) -> Pgvb0R {
        Pgvb0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pattern Generation 1 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb1(&self) -> Pgvb1R {
        Pgvb1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pattern Generation 2 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb2(&self) -> Pgvb2R {
        Pgvb2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pattern Generation 3 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb3(&self) -> Pgvb3R {
        Pgvb3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pattern Generation 4 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb4(&self) -> Pgvb4R {
        Pgvb4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pattern Generation 5 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb5(&self) -> Pgvb5R {
        Pgvb5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pattern Generation 6 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb6(&self) -> Pgvb6R {
        Pgvb6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pattern Generation 7 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb7(&self) -> Pgvb7R {
        Pgvb7R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pattern Generation 0 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb0(&mut self) -> Pgeb0W<'_, PattbufSpec> {
        Pgeb0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pattern Generation 1 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb1(&mut self) -> Pgeb1W<'_, PattbufSpec> {
        Pgeb1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pattern Generation 2 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb2(&mut self) -> Pgeb2W<'_, PattbufSpec> {
        Pgeb2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pattern Generation 3 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb3(&mut self) -> Pgeb3W<'_, PattbufSpec> {
        Pgeb3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pattern Generation 4 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb4(&mut self) -> Pgeb4W<'_, PattbufSpec> {
        Pgeb4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pattern Generation 5 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb5(&mut self) -> Pgeb5W<'_, PattbufSpec> {
        Pgeb5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pattern Generation 6 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb6(&mut self) -> Pgeb6W<'_, PattbufSpec> {
        Pgeb6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pattern Generation 7 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb7(&mut self) -> Pgeb7W<'_, PattbufSpec> {
        Pgeb7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pattern Generation 0 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb0(&mut self) -> Pgvb0W<'_, PattbufSpec> {
        Pgvb0W::new(self, 8)
    }
    #[doc = "Bit 9 - Pattern Generation 1 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb1(&mut self) -> Pgvb1W<'_, PattbufSpec> {
        Pgvb1W::new(self, 9)
    }
    #[doc = "Bit 10 - Pattern Generation 2 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb2(&mut self) -> Pgvb2W<'_, PattbufSpec> {
        Pgvb2W::new(self, 10)
    }
    #[doc = "Bit 11 - Pattern Generation 3 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb3(&mut self) -> Pgvb3W<'_, PattbufSpec> {
        Pgvb3W::new(self, 11)
    }
    #[doc = "Bit 12 - Pattern Generation 4 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb4(&mut self) -> Pgvb4W<'_, PattbufSpec> {
        Pgvb4W::new(self, 12)
    }
    #[doc = "Bit 13 - Pattern Generation 5 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb5(&mut self) -> Pgvb5W<'_, PattbufSpec> {
        Pgvb5W::new(self, 13)
    }
    #[doc = "Bit 14 - Pattern Generation 6 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb6(&mut self) -> Pgvb6W<'_, PattbufSpec> {
        Pgvb6W::new(self, 14)
    }
    #[doc = "Bit 15 - Pattern Generation 7 Output Value Buffer"]
    #[inline(always)]
    pub fn pgvb7(&mut self) -> Pgvb7W<'_, PattbufSpec> {
        Pgvb7W::new(self, 15)
    }
}
#[doc = "Pattern Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`pattbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pattbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PattbufSpec;
impl crate::RegisterSpec for PattbufSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pattbuf::R`](R) reader structure"]
impl crate::Readable for PattbufSpec {}
#[doc = "`write(|w| ..)` method takes [`pattbuf::W`](W) writer structure"]
impl crate::Writable for PattbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PATTBUF to value 0"]
impl crate::Resettable for PattbufSpec {}
