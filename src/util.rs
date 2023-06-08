#[macro_export]
macro_rules! f_as_str {
    ($($arg:tt)*) => {
        format!($($arg)*).as_str()
    };
}

#[macro_export]
macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr, $fill:expr, $extra:expr) => {
        format!(
            "<rect x='{x}' y='{y}' width='{w}' height='{h}' fill='{fill}' {extra} />",
            x = $x,
            y = $y,
            w = $w,
            h = $h,
            fill = $fill,
            extra = $extra
        )
        .as_str()
    };
}

#[macro_export]
macro_rules! text {
    ($text:expr, $x:expr, $y:expr, $fill:expr, $extra:expr) => {
        format!(
            "<text x='{x}' y='{y}' fill='{fill}' {extra}>{text}</text>",
            x = $x,
            y = $y,
            fill = $fill,
            extra = $extra,
            text = $text
        )
        .as_str()
    };
}

#[macro_export]
macro_rules! gradient {
    ($id:expr, $color:expr) => {
        format!(
            "<defs>
           <linearGradient id='{id}' x1='0' x2='1' y1='0' y2='1'>
              <stop offset='0%' stop-color='white'/>
              <stop offset='100%' stop-color='{color}'/>
           </linearGradient>
        </defs>",
            id = $id,
            color = $color
        )
        .as_str()
    };
}
