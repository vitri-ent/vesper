use std::any::type_name;
use crate::prelude::*;
use crate::twilight_exports::*;

const NUMBER_MAX_VALUE: i64 = 9007199254740991;

fn error(type_name: &str, required: bool, why: &str) -> ParseError {
    ParseError::Parsing {
        argument_name: String::new(),
        required,
        type_: type_name.to_string(),
        error: why.to_string()
    }
}

pub struct Range<T: Copy, const START: i64, const END: i64>(T);

impl<T: Copy, const START: i64, const END: i64> std::ops::Deref for Range<T, START, END> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Copy, const START: i64, const END: i64> std::ops::DerefMut for Range<T, START, END> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
impl<T, E: Copy, const START: i64, const END: i64> Parse<T> for Range<E, START, END>
where
    T: Send + Sync,
    E: Parse<T>
{
    async fn parse(http_client: &WrappedClient, data: &T, value: Option<&CommandOptionValue>) -> Result<Self, ParseError> {
        let value = E::parse(http_client, data, value).await?;
        let v = unsafe { *(&value as *const E as *const i64) };

        if v < START || v > END {
            return Err(error(
                &format!("Range<{}, {}, {}>", type_name::<E>(), START, END),
                true,
                "Input out of range"
            ));
        }

        Ok(Self(value))
    }

    fn option_type() -> CommandOptionType {
        E::option_type()
    }

    fn set_limits() -> Option<ArgumentLimits> {
        use twilight_model::application::command::CommandOptionValue;
        Some(ArgumentLimits {
            min: Some(CommandOptionValue::Integer(START)),
            max: Some(CommandOptionValue::Integer(END))
        })
    }
}

#[async_trait]
impl<T: Send + Sync> Parse<T> for String {
    async fn parse(
        _: &WrappedClient,
        _: &T,
        value: Option<&CommandOptionValue>,
    ) -> Result<Self, ParseError> {
        if let Some(kind) = value {
            if let CommandOptionValue::String(s) = kind {
                return Ok(s.to_owned());
            }
        }
        Err(error("String", true, "String expected"))
    }

    fn option_type() -> CommandOptionType {
        CommandOptionType::String
    }
}

#[async_trait]
impl<T: Send + Sync> Parse<T> for i64 {
    async fn parse(
        _: &WrappedClient,
        _: &T,
        value: Option<&CommandOptionValue>,
    ) -> Result<Self, ParseError> {
        if let Some(kind) = value {
            if let CommandOptionValue::Integer(i) = kind {
                return Ok(*i);
            }
        }
        Err(error("i64", true, "Integer expected"))
    }

    fn option_type() -> CommandOptionType {
        CommandOptionType::Integer
    }
}

#[async_trait]
impl<T: Send + Sync> Parse<T> for u64 {
    async fn parse(
        _: &WrappedClient,
        _: &T,
        value: Option<&CommandOptionValue>,
    ) -> Result<Self, ParseError> {
        if let Some(kind) = value {
            if let CommandOptionValue::Integer(i) = kind {
                if *i < 0 {
                    return Err(error("u64", true, "Input out of range"))
                }
                return Ok(*i as u64);
            }
        }
        Err(error("Integer", true, "Integer expected"))
    }

    fn option_type() -> CommandOptionType {
        CommandOptionType::Integer
    }

    fn set_limits() -> Option<ArgumentLimits> {
        use twilight_model::application::command::CommandOptionValue;
        Some(ArgumentLimits {
            min: Some(CommandOptionValue::Integer(0)),
            max: None
        })
    }
}

#[async_trait]
impl<T: Send + Sync> Parse<T> for f64 {
    async fn parse(
        _: &WrappedClient,
        _: &T,
        value: Option<&CommandOptionValue>,
    ) -> Result<Self, ParseError> {
        if let Some(kind) = value {
            if let CommandOptionValue::Number(i) = kind {
                return Ok(*i);
            }
        }
        Err(error("f64", true, "Number expected"))
    }

    fn option_type() -> CommandOptionType {
        CommandOptionType::Number
    }

    fn set_limits() -> Option<ArgumentLimits> {
        use twilight_model::application::command::CommandOptionValue;
        Some(ArgumentLimits {
            min: Some(CommandOptionValue::Number(f64::MIN)),
            max: Some(CommandOptionValue::Number(f64::MAX))
        })
    }
}

#[async_trait]
impl<T: Send + Sync> Parse<T> for f32 {
    async fn parse(
        _: &WrappedClient,
        _: &T,
        value: Option<&CommandOptionValue>,
    ) -> Result<Self, ParseError> {
        if let Some(kind) = value {
            if let CommandOptionValue::Number(i) = kind {
                if *i > f32::MAX as f64 || *i < f32::MIN as f64 {
                    return Err(error("f32", true, "Input out of range"))
                }
                return Ok(*i as f32);
            }
        }
        Err(error("f32", true, "Number expected"))
    }

    fn option_type() -> CommandOptionType {
        CommandOptionType::Number
    }

    fn set_limits() -> Option<ArgumentLimits> {
        use twilight_model::application::command::CommandOptionValue;
        Some(ArgumentLimits {
            min: Some(CommandOptionValue::Number(f32::MIN as f64)),
            max: Some(CommandOptionValue::Number(f32::MAX as f64))
        })
    }
}

#[async_trait]
impl<T: Send + Sync> Parse<T> for bool {
    async fn parse(
        _: &WrappedClient,
        _: &T,
        value: Option<&CommandOptionValue>,
    ) -> Result<Self, ParseError> {
        if let Some(kind) = value {
            if let CommandOptionValue::Boolean(i) = kind {
                return Ok(*i);
            }
        }
        Err(error("Boolean", true, "Boolean expected"))
    }

    fn option_type() -> CommandOptionType {
        CommandOptionType::Boolean
    }
}

#[async_trait]
impl<T: Send + Sync> Parse<T> for Id<ChannelMarker> {
    async fn parse(
        _: &WrappedClient,
        _: &T,
        value: Option<&CommandOptionValue>,
    ) -> Result<Self, ParseError> {
        if let Some(kind) = value {
            if let CommandOptionValue::Channel(channel) = kind {
                return Ok(*channel);
            }
        }

        Err(error("Channel id", true, "Channel expected"))
    }

    fn option_type() -> CommandOptionType {
        CommandOptionType::Channel
    }
}

#[async_trait]
impl<T: Send + Sync> Parse<T> for Id<UserMarker> {
    async fn parse(
        _: &WrappedClient,
        _: &T,
        value: Option<&CommandOptionValue>,
    ) -> Result<Self, ParseError> {
        if let Some(kind) = value {
            if let CommandOptionValue::User(user) = kind {
                return Ok(*user);
            }
        }

        Err(error("User id", true, "User expected"))
    }

    fn option_type() -> CommandOptionType {
        CommandOptionType::User
    }
}

#[async_trait]
impl<T: Send + Sync> Parse<T> for Id<RoleMarker> {
    async fn parse(
        _: &WrappedClient,
        _: &T,
        value: Option<&CommandOptionValue>,
    ) -> Result<Self, ParseError> {
        if let Some(kind) = value {
            if let CommandOptionValue::Role(role) = kind {
                return Ok(*role);
            }
        }

        Err(error("Role id", true, "Role expected"))
    }

    fn option_type() -> CommandOptionType {
        CommandOptionType::Role
    }
}

#[async_trait]
impl<T: Send + Sync> Parse<T> for Id<GenericMarker> {
    async fn parse(
        _: &WrappedClient,
        _: &T,
        value: Option<&CommandOptionValue>,
    ) -> Result<Self, ParseError> {
        if let Some(kind) = value {
            if let CommandOptionValue::Mentionable(id) = kind {
                return Ok(*id);
            }
        }

        Err(error("Id", true, "Mentionable expected"))
    }

    fn option_type() -> CommandOptionType {
        CommandOptionType::Mentionable
    }
}

#[async_trait]
impl<T: Parse<E>, E: Send + Sync> Parse<E> for Option<T> {
    async fn parse(
        http_client: &WrappedClient,
        data: &E,
        value: Option<&CommandOptionValue>,
    ) -> Result<Self, ParseError> {
        match T::parse(http_client, data, value).await {
            Ok(parsed) => Ok(Some(parsed)),
            Err(mut why) => {
                if value.is_some() {
                    match &mut why {
                        ParseError::Parsing {required, ..} => *required = false,
                        _ => ()
                    }
                    Err(why)
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn option_type() -> CommandOptionType {
        T::option_type()
    }

    fn is_required() -> bool {
        false
    }

    fn add_choices() -> Option<Vec<CommandOptionChoice>> {
        T::add_choices()
    }

    fn set_limits() -> Option<ArgumentLimits> {
        T::set_limits()
    }
}

#[async_trait]
impl<T, E, C> Parse<C> for Result<T, E>
where
    T: Parse<C>,
    E: From<ParseError>,
    C: Send + Sync,
{
    async fn parse(
        http_client: &WrappedClient,
        data: &C,
        value: Option<&CommandOptionValue>,
    ) -> Result<Self, ParseError> {
        // as we want to return the error if occurs, we'll map the error and always return Ok
        Ok(T::parse(http_client, data, value).await.map_err(From::from))
    }

    fn option_type() -> CommandOptionType {
        T::option_type()
    }

    fn is_required() -> bool {
        T::is_required()
    }

    fn add_choices() -> Option<Vec<CommandOptionChoice>> {
        T::add_choices()
    }

    fn set_limits() -> Option<ArgumentLimits> {
        T::set_limits()
    }
}

macro_rules! impl_derived_parse {
    ($([$($derived:ty),+] from $prim:ty),* $(,)?) => {
        $($(
            #[async_trait]
            impl<T: Send + Sync> Parse<T> for $derived {
                async fn parse(
                    http_client: &WrappedClient,
                    data: &T,
                    value: Option<&CommandOptionValue>
                ) -> Result<Self, ParseError> {
                    let p = <$prim>::parse(http_client, data, value).await?;

                    if p > <$derived>::MAX as $prim {
                        Err(error(
                            stringify!($derived),
                            true,
                            concat!(
                                "Failed to parse to ",
                                stringify!($derived),
                                ": the value is greater than ",
                                stringify!($derived),
                                "'s ",
                                "range of values"
                            )
                        ))
                    } else if p < <$derived>::MIN as $prim {
                        Err(error(
                            stringify!($derived),
                            true,
                            concat!(
                                "Failed to parse to ",
                                stringify!($derived),
                                ": the value is less than ",
                                stringify!($derived),
                                "'s ",
                                "range of values"
                            )
                        ))
                    } else {
                        Ok(p as $derived)
                    }
                }

                fn option_type() -> CommandOptionType {
                    <$prim as Parse<T>>::option_type()
                }

                fn set_limits() -> Option<ArgumentLimits> {
                    use twilight_model::application::command::CommandOptionValue;
                    Some(ArgumentLimits {
                        min: Some(CommandOptionValue::Integer(<$derived>::MIN as i64)),
                        max: Some(CommandOptionValue::Integer({
                            if <$derived>::MAX as i64 > NUMBER_MAX_VALUE {
                                NUMBER_MAX_VALUE
                            } else {
                                <$derived>::MAX as i64
                            }
                        }))
                    })
                }
            }
        )*)*
    };
}

impl_derived_parse! {
    [i8, i16, i32, isize] from i64,
    [u8, u16, u32, usize] from u64,
}
