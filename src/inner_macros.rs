macro_rules! pg_counted {
    ($length: expr, $($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        {
            use tracing::Span;
            use tracing_indicatif::span_ext::IndicatifSpanExt as _;

            Span::current().pb_set_length($length as u64);
            Span::current().pb_set_message(&format!($($arg)*));
        }
    };
}

pub(crate) use pg_counted;

macro_rules! pg_inc {
    () => {
        #[cfg(feature = "tracing")]
        {
            use tracing::Span;
            use tracing_indicatif::span_ext::IndicatifSpanExt as _;

            Span::current().pb_inc(1);
        }
    };

    ($inc: expr) => {
        #[cfg(feature = "tracing")]
        {
            use tracing::Span;

            Span::current().pb_inc($inc);
        }
    };
}

pub(crate) use pg_inc;
