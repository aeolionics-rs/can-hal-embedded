#[doc = "Register `RSTATUSA` reader"]
pub type R = crate::R<RstatusaSpec>;
#[doc = "IDAU Region Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Typeselect {
    #[doc = "0: Region is invalid"]
    Invalid = 0,
    #[doc = "1: Region is secure"]
    Sec = 1,
    #[doc = "2: Region is non secure"]
    Nonsec = 2,
    #[doc = "3: Region is non secure, the associated register group implements a watermark configuration"]
    NonsecW = 3,
    #[doc = "4: Region is either secure or non-secure, the associated register group implements a block configuration"]
    SecnNonsecB = 4,
    #[doc = "5: Region is either secure or non-secure, configuration is linked to another IDAU region configuration bit (the linked configuration bit identification info is readable from the RSTATUSC register inside the same register group)"]
    SecnNonsecL = 5,
    #[doc = "6: Region is non-secure callable, the associated register group implements a watermark configuration"]
    NscW = 6,
    #[doc = "7: Region is exempt from security, the associated register group implements a block configuration"]
    ExemptB = 7,
}
impl From<Typeselect> for u8 {
    #[inline(always)]
    fn from(variant: Typeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Typeselect {
    type Ux = u8;
}
impl crate::IsEnum for Typeselect {}
#[doc = "Field `TYPE` reader - IDAU Region Type"]
pub type TypeR = crate::FieldReader<Typeselect>;
impl TypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Typeselect> {
        match self.bits {
            0 => Some(Typeselect::Invalid),
            1 => Some(Typeselect::Sec),
            2 => Some(Typeselect::Nonsec),
            3 => Some(Typeselect::NonsecW),
            4 => Some(Typeselect::SecnNonsecB),
            5 => Some(Typeselect::SecnNonsecL),
            6 => Some(Typeselect::NscW),
            7 => Some(Typeselect::ExemptB),
            _ => None,
        }
    }
    #[doc = "Region is invalid"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == Typeselect::Invalid
    }
    #[doc = "Region is secure"]
    #[inline(always)]
    pub fn is_sec(&self) -> bool {
        *self == Typeselect::Sec
    }
    #[doc = "Region is non secure"]
    #[inline(always)]
    pub fn is_nonsec(&self) -> bool {
        *self == Typeselect::Nonsec
    }
    #[doc = "Region is non secure, the associated register group implements a watermark configuration"]
    #[inline(always)]
    pub fn is_nonsec_w(&self) -> bool {
        *self == Typeselect::NonsecW
    }
    #[doc = "Region is either secure or non-secure, the associated register group implements a block configuration"]
    #[inline(always)]
    pub fn is_secn_nonsec_b(&self) -> bool {
        *self == Typeselect::SecnNonsecB
    }
    #[doc = "Region is either secure or non-secure, configuration is linked to another IDAU region configuration bit (the linked configuration bit identification info is readable from the RSTATUSC register inside the same register group)"]
    #[inline(always)]
    pub fn is_secn_nonsec_l(&self) -> bool {
        *self == Typeselect::SecnNonsecL
    }
    #[doc = "Region is non-secure callable, the associated register group implements a watermark configuration"]
    #[inline(always)]
    pub fn is_nsc_w(&self) -> bool {
        *self == Typeselect::NscW
    }
    #[doc = "Region is exempt from security, the associated register group implements a block configuration"]
    #[inline(always)]
    pub fn is_exempt_b(&self) -> bool {
        *self == Typeselect::ExemptB
    }
}
#[doc = "Region Granularity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Granselect {
    #[doc = "0: Not Applicable"]
    GranNa = 0,
    #[doc = "5: `101`"]
    Gran32b = 5,
    #[doc = "6: `110`"]
    Gran64b = 6,
    #[doc = "7: `111`"]
    Gran128b = 7,
    #[doc = "8: `1000`"]
    Gran256b = 8,
    #[doc = "9: `1001`"]
    Gran512b = 9,
    #[doc = "10: `1010`"]
    Gran1kb = 10,
    #[doc = "11: `1011`"]
    Gran2kb = 11,
    #[doc = "12: `1100`"]
    Gran4kb = 12,
    #[doc = "13: `1101`"]
    Gran8kb = 13,
    #[doc = "14: `1110`"]
    Gran16kb = 14,
    #[doc = "15: `1111`"]
    Gran32kb = 15,
    #[doc = "16: `10000`"]
    Gran64kb = 16,
    #[doc = "17: `10001`"]
    Gran128kb = 17,
    #[doc = "18: `10010`"]
    Gran256kb = 18,
    #[doc = "19: `10011`"]
    Gran512kb = 19,
    #[doc = "20: `10100`"]
    Gran1mb = 20,
    #[doc = "21: `10101`"]
    Gran2mb = 21,
    #[doc = "22: `10110`"]
    Gran4mb = 22,
    #[doc = "23: `10111`"]
    Gran8mb = 23,
    #[doc = "24: `11000`"]
    Gran16mb = 24,
    #[doc = "25: `11001`"]
    Gran32mb = 25,
    #[doc = "26: `11010`"]
    Gran64mb = 26,
    #[doc = "27: `11011`"]
    Gran128mb = 27,
    #[doc = "28: `11100`"]
    Gran256mb = 28,
    #[doc = "29: `11101`"]
    Gran512mb = 29,
    #[doc = "30: `11110`"]
    Gran1gb = 30,
    #[doc = "31: `11111`"]
    Gran2gb = 31,
    #[doc = "32: `100000`"]
    Gran4gb = 32,
}
impl From<Granselect> for u8 {
    #[inline(always)]
    fn from(variant: Granselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Granselect {
    type Ux = u8;
}
impl crate::IsEnum for Granselect {}
#[doc = "Field `GRAN` reader - Region Granularity"]
pub type GranR = crate::FieldReader<Granselect>;
impl GranR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Granselect> {
        match self.bits {
            0 => Some(Granselect::GranNa),
            5 => Some(Granselect::Gran32b),
            6 => Some(Granselect::Gran64b),
            7 => Some(Granselect::Gran128b),
            8 => Some(Granselect::Gran256b),
            9 => Some(Granselect::Gran512b),
            10 => Some(Granselect::Gran1kb),
            11 => Some(Granselect::Gran2kb),
            12 => Some(Granselect::Gran4kb),
            13 => Some(Granselect::Gran8kb),
            14 => Some(Granselect::Gran16kb),
            15 => Some(Granselect::Gran32kb),
            16 => Some(Granselect::Gran64kb),
            17 => Some(Granselect::Gran128kb),
            18 => Some(Granselect::Gran256kb),
            19 => Some(Granselect::Gran512kb),
            20 => Some(Granselect::Gran1mb),
            21 => Some(Granselect::Gran2mb),
            22 => Some(Granselect::Gran4mb),
            23 => Some(Granselect::Gran8mb),
            24 => Some(Granselect::Gran16mb),
            25 => Some(Granselect::Gran32mb),
            26 => Some(Granselect::Gran64mb),
            27 => Some(Granselect::Gran128mb),
            28 => Some(Granselect::Gran256mb),
            29 => Some(Granselect::Gran512mb),
            30 => Some(Granselect::Gran1gb),
            31 => Some(Granselect::Gran2gb),
            32 => Some(Granselect::Gran4gb),
            _ => None,
        }
    }
    #[doc = "Not Applicable"]
    #[inline(always)]
    pub fn is_gran_na(&self) -> bool {
        *self == Granselect::GranNa
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_gran_32b(&self) -> bool {
        *self == Granselect::Gran32b
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_gran_64b(&self) -> bool {
        *self == Granselect::Gran64b
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_gran_128b(&self) -> bool {
        *self == Granselect::Gran128b
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_gran_256b(&self) -> bool {
        *self == Granselect::Gran256b
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_gran_512b(&self) -> bool {
        *self == Granselect::Gran512b
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_gran_1kb(&self) -> bool {
        *self == Granselect::Gran1kb
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_gran_2kb(&self) -> bool {
        *self == Granselect::Gran2kb
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_gran_4kb(&self) -> bool {
        *self == Granselect::Gran4kb
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_gran_8kb(&self) -> bool {
        *self == Granselect::Gran8kb
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_gran_16kb(&self) -> bool {
        *self == Granselect::Gran16kb
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_gran_32kb(&self) -> bool {
        *self == Granselect::Gran32kb
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn is_gran_64kb(&self) -> bool {
        *self == Granselect::Gran64kb
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn is_gran_128kb(&self) -> bool {
        *self == Granselect::Gran128kb
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn is_gran_256kb(&self) -> bool {
        *self == Granselect::Gran256kb
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn is_gran_512kb(&self) -> bool {
        *self == Granselect::Gran512kb
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn is_gran_1mb(&self) -> bool {
        *self == Granselect::Gran1mb
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn is_gran_2mb(&self) -> bool {
        *self == Granselect::Gran2mb
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn is_gran_4mb(&self) -> bool {
        *self == Granselect::Gran4mb
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn is_gran_8mb(&self) -> bool {
        *self == Granselect::Gran8mb
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn is_gran_16mb(&self) -> bool {
        *self == Granselect::Gran16mb
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn is_gran_32mb(&self) -> bool {
        *self == Granselect::Gran32mb
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn is_gran_64mb(&self) -> bool {
        *self == Granselect::Gran64mb
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn is_gran_128mb(&self) -> bool {
        *self == Granselect::Gran128mb
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn is_gran_256mb(&self) -> bool {
        *self == Granselect::Gran256mb
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn is_gran_512mb(&self) -> bool {
        *self == Granselect::Gran512mb
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn is_gran_1gb(&self) -> bool {
        *self == Granselect::Gran1gb
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn is_gran_2gb(&self) -> bool {
        *self == Granselect::Gran2gb
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn is_gran_4gb(&self) -> bool {
        *self == Granselect::Gran4gb
    }
}
#[doc = "Field `MAXSZ` reader - Command Argument Maximum Size"]
pub type MaxszR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - IDAU Region Type"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Region Granularity"]
    #[inline(always)]
    pub fn gran(&self) -> GranR {
        GranR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Command Argument Maximum Size"]
    #[inline(always)]
    pub fn maxsz(&self) -> MaxszR {
        MaxszR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "Region Status A\n\nYou can [`read`](crate::Reg::read) this register and get [`rstatusa::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstatusaSpec;
impl crate::RegisterSpec for RstatusaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstatusa::R`](R) reader structure"]
impl crate::Readable for RstatusaSpec {}
#[doc = "`reset()` method sets RSTATUSA to value 0"]
impl crate::Resettable for RstatusaSpec {}
