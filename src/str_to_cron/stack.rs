use super::action;

#[derive(Clone, Debug)]
pub struct StartEnd {
    pub start: Option<i32>,
    pub end: Option<i32>,
}

#[derive(Clone, Debug)]
pub struct StartEndString {
    pub start: Option<String>,
    pub end: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Stack {
    pub owner: action::Kind,
    pub frequency: Option<i32>,
    pub frequency_end: Option<i32>,
    pub frequency_start: Option<i32>,
    pub min: Option<StartEnd>,
    pub hour: Option<StartEnd>,
    pub day: Option<StartEndString>,
    pub month: Option<StartEndString>,
    pub year: Option<StartEnd>,
    pub day_of_week: Option<String>,
    pub is_and_connector: bool,
    pub is_between_range: bool,
}

impl Stack {
    pub const fn builder(owner: action::Kind) -> Builder {
        Builder {
            stack: Self {
                owner,
                frequency: None,
                frequency_end: None,
                frequency_start: None,
                min: None,
                hour: None,
                day: None,
                month: None,
                year: None,
                day_of_week: None,
                is_and_connector: false,
                is_between_range: false,
            },
        }
    }
}

pub struct Builder {
    stack: Stack,
}

impl Builder {
    pub const fn frequency(mut self, frequency: i32) -> Self {
        self.stack.frequency = Some(frequency);
        self
    }

    pub const fn min(mut self, min: StartEnd) -> Self {
        self.stack.min = Some(min);
        self
    }

    pub const fn hour(mut self, hour: StartEnd) -> Self {
        self.stack.hour = Some(hour);
        self
    }

    pub fn month(mut self, month: StartEndString) -> Self {
        self.stack.month = Some(month);
        self
    }

    pub fn day_of_week(mut self, day_of_week: String) -> Self {
        self.stack.day_of_week = Some(day_of_week);
        self
    }

    pub fn build(self) -> Stack {
        self.stack
    }
}

impl Stack {
    pub fn frequency_to_string(&self) -> String {
        self.frequency.map_or("*".to_string(), |a| a.to_string())
    }
}
