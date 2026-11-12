#[doc = "Register `TIMESTAMP` reader"]
pub type R = crate::R<TimestampSpec>;
#[doc = "Field `SECOND` reader - Second Timestamp Value"]
pub type SecondR = crate::FieldReader;
#[doc = "Field `MINUTE` reader - Minute Timestamp Value"]
pub type MinuteR = crate::FieldReader;
#[doc = "Field `HOUR` reader - Hour Timestamp Value"]
pub type HourR = crate::FieldReader;
#[doc = "Field `DAY` reader - Day Timestamp Value"]
pub type DayR = crate::FieldReader;
#[doc = "Field `MONTH` reader - Month Timestamp Value"]
pub type MonthR = crate::FieldReader;
#[doc = "Field `YEAR` reader - Year Timestamp Value"]
pub type YearR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Second Timestamp Value"]
    #[inline(always)]
    pub fn second(&self) -> SecondR {
        SecondR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Minute Timestamp Value"]
    #[inline(always)]
    pub fn minute(&self) -> MinuteR {
        MinuteR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:16 - Hour Timestamp Value"]
    #[inline(always)]
    pub fn hour(&self) -> HourR {
        HourR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - Day Timestamp Value"]
    #[inline(always)]
    pub fn day(&self) -> DayR {
        DayR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:25 - Month Timestamp Value"]
    #[inline(always)]
    pub fn month(&self) -> MonthR {
        MonthR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:31 - Year Timestamp Value"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[doc = "MODE2 Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimestampSpec;
impl crate::RegisterSpec for TimestampSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp::R`](R) reader structure"]
impl crate::Readable for TimestampSpec {}
#[doc = "`reset()` method sets TIMESTAMP to value 0"]
impl crate::Resettable for TimestampSpec {}
