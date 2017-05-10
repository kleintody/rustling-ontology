use std::{fmt, result};

use rustling::*;
use moment::{RcConstraint, Period, Grain};

/// Union of all possible values parsed by the ontology.
rustling_value! {
    #[doc="Union of all possible values parsed by the ontology."]
    #[derive(Clone,PartialEq,Debug)]
    Dimension DimensionKind
    Number(NumberValue),
    AmountOfMoney(AmountOfMoneyValue),
    Ordinal(OrdinalValue),
    Temperature(TemperatureValue),
    MoneyUnit(MoneyUnitValue),
    Time(TimeValue),
    Duration(DurationValue),
    Cycle(CycleValue),
    UnitOfDuration(UnitOfDurationValue),
    RelativeMinute(RelativeMinuteValue),
}

impl fmt::Display for Dimension {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            &Dimension::Number(ref number) => {
                match number {
                    &NumberValue::Integer(ref v) => write!(fmt, "Number: {}", v.value),
                    &NumberValue::Float(ref v) => write!(fmt, "Number: {}", v.value),
                }
            }
            &Dimension::Ordinal(_) => write!(fmt, "Ordinal"),
            &Dimension::Temperature(_) => write!(fmt, "Temperature"),
            &Dimension::AmountOfMoney(_) => write!(fmt, "AmountOfMoney"),
            &Dimension::MoneyUnit(_) => write!(fmt, "MoneyUnit"),
            &Dimension::Time(_) => write!(fmt, "Time"),
            &Dimension::Duration(_) => write!(fmt, "Duration"),
            &Dimension::Cycle(_) => write!(fmt, "Cycle"),
            &Dimension::UnitOfDuration(_) => write!(fmt, "UnitOfDuration"),
            &Dimension::RelativeMinute(_) => write!(fmt, "RelativeMinute"),
        }
    }
}

/// Payload for the ordinal numbers of Dimension
#[derive(Debug,PartialEq,Copy,Clone)]
pub struct OrdinalValue {
    pub value: i64,
}

#[derive(Debug,PartialEq,Copy,Clone)]
pub enum Precision {
    Approximate,
    Exact,
}

impl Default for Precision {
    fn default() -> Precision {
        Precision::Exact
    }
}

/// Payload for the amount of money of Dimension
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct AmountOfMoneyValue {
    pub value: f32,
    pub precision: Precision,
    pub unit: Option<&'static str>,
}

/// Payload for the unit of money of Dimension
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct MoneyUnitValue {
    pub unit: Option<&'static str>,
}

/// Payload for the integral numbers of Dimension
#[derive(Debug,PartialEq,Clone,Default)]
pub struct IntegerValue {
    pub value: i64,
    #[doc(hidden)]
    pub grain: Option<u8>,
    #[doc(hidden)]
    pub group: bool,
    #[doc(hidden)]
    pub prefixed: bool,
    #[doc(hidden)]
    pub suffixed: bool,
    #[doc(hidden)]
    pub precision: Precision,
}

impl IntegerValue {
    pub fn new(value: i64) -> RuleResult<IntegerValue> {
        Ok(IntegerValue {
               value: value,
               grain: None,
               ..IntegerValue::default()
           })
    }
    pub fn new_with_grain(value: i64, grain: u8) -> RuleResult<IntegerValue> {
        Ok(IntegerValue {
               value: value,
               grain: Some(grain),
               ..IntegerValue::default()
           })
    }
}

impl From<IntegerValue> for Dimension {
    fn from(v: IntegerValue) -> Dimension {
        Dimension::Number(NumberValue::Integer(v))
    }
}

impl From<FloatValue> for Dimension {
    fn from(v: FloatValue) -> Dimension {
        Dimension::Number(NumberValue::Float(v))
    }
}

impl From<IntegerValue> for NumberValue {
    fn from(v: IntegerValue) -> NumberValue {
        NumberValue::Integer(v)
    }
}

impl AttemptFrom<Dimension> for IntegerValue {
    fn attempt_from(v: Dimension) -> Option<IntegerValue> {
        if let Dimension::Number(value) = v {
            if let NumberValue::Integer(integer) = value {
                Some(integer)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl AttemptTo<i64> for Dimension {
    fn attempt_to(&self) -> Option<i64> {
        IntegerValue::attempt_from(self.clone()).map(|it| it.value)
    }
}

impl AttemptFrom<Dimension> for FloatValue {
    fn attempt_from(v: Dimension) -> Option<FloatValue> {
        if let Dimension::Number(value) = v {
            if let NumberValue::Float(float) = value {
                Some(float)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Payload for the floating numbers of Dimension
#[derive(Debug,PartialEq,Clone,Default)]
pub struct FloatValue {
    pub value: f32,
    #[doc(hidden)]
    pub prefixed: bool,
    #[doc(hidden)]
    pub suffixed: bool,
    #[doc(hidden)]
    pub precision: Precision,
}

impl FloatValue {
    pub fn new(value: f32) -> RuleResult<FloatValue> {
        Ok(FloatValue {
               value: value,
               ..FloatValue::default()
           })
    }
}

impl From<FloatValue> for NumberValue {
    fn from(v: FloatValue) -> NumberValue {
        NumberValue::Float(v)
    }
}

/// Enumeration acting as a Number supertype for IntegerValue and FloatValue.
#[derive(Debug, PartialEq, Clone)]
pub enum NumberValue {
    Float(FloatValue),
    Integer(IntegerValue),
}

impl NumberValue {
    #[doc(hidden)]
    pub fn prefixed(&self) -> bool {
        match self {
            &NumberValue::Float(ref v) => v.prefixed,
            &NumberValue::Integer(ref v) => v.prefixed,
        }
    }

    #[doc(hidden)]
    pub fn suffixed(&self) -> bool {
        match self {
            &NumberValue::Float(ref v) => v.suffixed,
            &NumberValue::Integer(ref v) => v.suffixed,
        }
    }

    #[doc(hidden)]
    pub fn value(&self) -> f32 {
        match self {
            &NumberValue::Float(ref v) => v.value,
            &NumberValue::Integer(ref v) => v.value as f32,
        }
    }

    #[doc(hidden)]
    pub fn grain(&self) -> Option<u8> {
        match self {
            &NumberValue::Float(_) => None,
            &NumberValue::Integer(ref v) => v.grain,
        }
    }
}

/// Payload for the temperatures of Dimension
#[derive(Debug,PartialEq,Clone)]
pub struct TemperatureValue {
    pub value: f32,
    /// Celsius, Fahrenheit, ...
    pub unit: Option<&'static str>,
    /// true if it can not be confirmed that the value is actually a temperature
    pub latent: bool,
}

/// Payload for the cycle of Dimension
#[derive(Debug,PartialEq,Clone)]
pub struct CycleValue {
    pub grain: Grain,
}

impl CycleValue {
    pub fn new(grain: Grain) -> RuleResult<CycleValue> {
        Ok(CycleValue { grain: grain })
    }
}

/// Payload for the unit of duration of Dimension
#[derive(Debug,PartialEq,Clone)]
pub struct UnitOfDurationValue {
    pub grain: Grain,
}

impl UnitOfDurationValue {
    pub fn new(grain: Grain) -> UnitOfDurationValue {
        UnitOfDurationValue { grain: grain }
    }
}

/// Payload for the time of Dimension
#[derive(Clone)]
pub struct TimeValue {
    pub constraint: RcConstraint,
    pub form: Form,
    pub direction: Option<Direction>,
    pub precision: Precision,
    pub latent: bool,
}

// We need partial eq to make Dimension partial eq happy, but this is only
// useful for testing.
impl PartialEq for TimeValue {
    fn eq(&self, _other: &TimeValue) -> bool {
        unimplemented!()
    }
}

impl ::std::fmt::Debug for TimeValue {
    fn fmt(&self,fmt: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        write!(fmt, "<TimeValue>")
    }
}

#[derive(Debug,PartialEq,Clone)]
pub enum Form {
    Month(u32),
    TimeOfDay(Option<TimeOfDayForm>),
    DayOfWeek { not_immediate: bool },
    PartOfDay,
    Empty,
}

impl Form {
    pub fn not_immediate(&self) -> Option<bool> {
        match self {
            &Form::Month(_) => None,
            &Form::TimeOfDay(_) => None,
            &Form::DayOfWeek { not_immediate } => Some(not_immediate),
            &Form::Empty => None,
            &Form::PartOfDay => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    After,
    Before,
}

#[derive(Debug,PartialEq,Clone)]
pub struct TimeOfDayForm {
    pub full_hour: u32,
    pub is_12_clock: bool,
}

#[derive(Debug,PartialEq,Clone)]
pub struct DurationValue {
    pub period: Period, 
    pub precision: Precision
}

impl DurationValue {
    pub fn new(period: Period) -> DurationValue {
        DurationValue { period: period, precision: Precision::Exact }
    }

    pub fn precision(self, precision: Precision) -> DurationValue {
        DurationValue { precision: precision, .. self }
    }
}

#[derive(Debug,PartialEq,Clone)]
pub struct RelativeMinuteValue(pub i32);
