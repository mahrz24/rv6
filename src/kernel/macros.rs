#[macro_escape];

macro_rules! val_parser (
  (, $fmt_val1:expr) => (
    |_term, _i, _format| {
      if _i==0 {
        $fmt_val1.print(_term,_format);
      }
    }
  );
  (, $fmt_val1:expr $(, $fmt_vals:expr)*) => (
    |_term, _i, _format| {
      if _i==0 {
        $fmt_val1.print(_term,_format);
      }
      else {
        let formatter:&fn(&mut VGATerminal, uint, Format) = val_parser!($(, $fmt_vals)*);
        formatter(_term, _i-1, _format);
      }
    }
  )
)

macro_rules! kfmt (
  ($term:ident, $fmt_str:expr) =>
  (
    $term.print_string($fmt_str)
  );
  ($term:ident, $fmt_str:expr $(, $fmt_vals:expr)*) =>
  (
    {
    let formatter:&fn(&mut VGATerminal, uint, Format) = val_parser!($(, $fmt_vals)*);
    $term.print_format($fmt_str, formatter);
  }
  );
)
