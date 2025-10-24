#[macro_export]
macro_rules! legacy_block {
  ($legacy:block, $new:block) => {{
    #[cfg(feature = "legacy")]
    {
      $legacy // * === LEGACY BLOCK === *
    }
    #[cfg(not(feature = "legacy"))]
    {
      $new // * === EXPERIMENTAL BLOCK === *
    }
  }};
}

// *

#[macro_export]
macro_rules! scalable_block {
  ($scalable:block, $normal:block) => {{
    #[cfg(feature = "scalability")]
    {
      $scalable // * === LEGACY BLOCK === *
    }
    #[cfg(not(feature = "scalability"))]
    {
      $normal // * === EXPERIMENTAL BLOCK === *
    }
  }};
}

// * >>> *

#[macro_export]
macro_rules! legacy_code {
  ($legacy:block, $experimental:block) => {{
    #[cfg(feature = "legacy")]
    {
      $legacy // * === LEGACY === *
    }
    #[cfg(not(feature = "legacy"))]
    {
      $experimental // * === EXPERIMENTAL === *
    }
  }};
}
