use strum::{Display, EnumIs, EnumString};

#[expect(unused_imports)]
use crate::layout::Constraint;

/// Defines the options for layout flex justify content in a container.
///
/// This enumeration controls the distribution of space when layout constraints are met and there
/// is excess space available. The `Flex` option is used with [`Layout`](crate::layout::Layout) to
/// control how extra space is distributed among layout segments, which is particularly useful for
/// creating responsive layouts that adapt to different terminal sizes.
///
/// Available options:
///
/// - `Legacy`: Fills the available space within the container, putting excess space into the last
///   element.
/// - `Start`: Aligns items to the start of the container.
/// - `End`: Aligns items to the end of the container.
/// - `Center`: Centers items within the container.
/// - `SpaceBetween`: Adds excess space between each element.
/// - `SpaceAround`: Adds excess space around each element.
///
/// For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.
#[derive(Copy, Debug, Default, Display, EnumString, Clone, Eq, PartialEq, Hash, EnumIs)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Flex {
    /// Fills the available space within the container, putting excess space into the last
    /// constraint of the lowest priority. This matches the default behavior of ratatui and tui
    /// applications without [`Flex`]
    ///
    /// The following examples illustrate the allocation of excess in various combinations of
    /// constraints. As a refresher, the priorities of constraints are as follows:
    ///
    /// 1. [`Constraint::Min`]
    /// 2. [`Constraint::Max`]
    /// 3. [`Constraint::Length`]
    /// 4. [`Constraint::Percentage`]
    /// 5. [`Constraint::Ratio`]
    /// 6. [`Constraint::Fill`]
    ///
    /// When every constraint is `Length`, the last element gets the excess.
    ///
    /// ```plain
    /// <----------------------------------- 80 px ------------------------------------>
    /// ┌──────20 px───────┐┌──────20 px───────┐┌────────────────40 px─────────────────┐
    /// │    Length(20)    ││    Length(20)    ││              Length(20)              │
    /// └──────────────────┘└──────────────────┘└──────────────────────────────────────┘
    ///                                         ^^^^^^^^^^^^^^^^ EXCESS ^^^^^^^^^^^^^^^^
    /// ```
    ///
    /// Fill constraints have the lowest priority amongst all the constraints and hence
    /// will always take up any excess space available.
    ///
    /// ```plain
    /// <----------------------------------- 80 px ------------------------------------>
    /// ┌──────20 px───────┐┌──────20 px───────┐┌──────20 px───────┐┌──────20 px───────┐
    /// │      Fill(0)     ││      Max(20)     ││    Length(20)    ││     Length(20)   │
    /// └──────────────────┘└──────────────────┘└──────────────────┘└──────────────────┘
    /// ^^^^^^ EXCESS ^^^^^^
    /// ```
    ///
    /// # Examples
    ///
    /// ```plain
    /// <------------------------------------80 px------------------------------------->
    /// ┌──────────────────────────60 px───────────────────────────┐┌──────20 px───────┐
    /// │                          Min(20)                         ││      Max(20)     │
    /// └──────────────────────────────────────────────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    /// ┌────────────────────────────────────80 px─────────────────────────────────────┐
    /// │                                    Max(20)                                   │
    /// └──────────────────────────────────────────────────────────────────────────────┘
    /// ```
    Legacy,

    /// Aligns items to the start of the container.
    ///
    /// # Examples
    ///
    /// ```plain
    /// <------------------------------------80 px------------------------------------->
    /// ┌────16 px─────┐┌──────20 px───────┐┌──────20 px───────┐
    /// │Percentage(20)││    Length(20)    ││     Fixed(20)    │
    /// └──────────────┘└──────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    /// ┌──────20 px───────┐┌──────20 px───────┐
    /// │      Max(20)     ││      Max(20)     │
    /// └──────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    /// ┌──────20 px───────┐
    /// │      Max(20)     │
    /// └──────────────────┘
    /// ```
    #[default]
    Start,

    /// Aligns items to the end of the container.
    ///
    /// # Examples
    ///
    /// ```plain
    /// <------------------------------------80 px------------------------------------->
    ///                         ┌────16 px─────┐┌──────20 px───────┐┌──────20 px───────┐
    ///                         │Percentage(20)││    Length(20)    ││     Length(20)   │
    ///                         └──────────────┘└──────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    ///                                         ┌──────20 px───────┐┌──────20 px───────┐
    ///                                         │      Max(20)     ││      Max(20)     │
    ///                                         └──────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    ///                                                             ┌──────20 px───────┐
    ///                                                             │      Max(20)     │
    ///                                                             └──────────────────┘
    /// ```
    End,

    /// Centers items within the container.
    ///
    /// # Examples
    ///
    /// ```plain
    /// <------------------------------------80 px------------------------------------->
    ///             ┌────16 px─────┐┌──────20 px───────┐┌──────20 px───────┐
    ///             │Percentage(20)││    Length(20)    ││     Length(20)   │
    ///             └──────────────┘└──────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    ///                     ┌──────20 px───────┐┌──────20 px───────┐
    ///                     │      Max(20)     ││      Max(20)     │
    ///                     └──────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    ///                               ┌──────20 px───────┐
    ///                               │      Max(20)     │
    ///                               └──────────────────┘
    /// ```
    Center,

    /// Adds excess space between each element.
    ///
    /// # Examples
    ///
    /// ```plain
    /// <------------------------------------80 px------------------------------------->
    /// ┌────16 px─────┐            ┌──────20 px───────┐            ┌──────20 px───────┐
    /// │Percentage(20)│            │    Length(20)    │            │     Length(20)   │
    /// └──────────────┘            └──────────────────┘            └──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    /// ┌──────20 px───────┐                                        ┌──────20 px───────┐
    /// │      Max(20)     │                                        │      Max(20)     │
    /// └──────────────────┘                                        └──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    /// ┌────────────────────────────────────80 px─────────────────────────────────────┐
    /// │                                    Max(20)                                   │
    /// └──────────────────────────────────────────────────────────────────────────────┘
    /// ```
    SpaceBetween,

    /// Evenly distributes excess space between all elements, including before the first and after
    /// the last.
    ///
    /// # Examples
    ///
    /// ```plain
    /// <------------------------------------80 px------------------------------------->
    ///       ┌────16 px─────┐      ┌──────20 px───────┐      ┌──────20 px───────┐
    ///       │Percentage(20)│      │    Length(20)    │      │     Length(20)   │
    ///       └──────────────┘      └──────────────────┘      └──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    ///              ┌──────20 px───────┐              ┌──────20 px───────┐
    ///              │      Max(20)     │              │      Max(20)     │
    ///              └──────────────────┘              └──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    ///                               ┌──────20 px───────┐
    ///                               │      Max(20)     │
    ///                               └──────────────────┘
    /// ```
    SpaceEvenly,

    /// Adds excess space around each element.
    ///
    /// # Examples
    ///
    /// ```plain
    /// <------------------------------------80 px------------------------------------->
    ///     ┌────16 px─────┐       ┌──────20 px───────┐       ┌──────20 px───────┐
    ///     │Percentage(20)│       │    Length(20)    │       │     Length(20)   │
    ///     └──────────────┘       └──────────────────┘       └──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    ///      ┌──────20 px───────┐                      ┌──────20 px───────┐
    ///      │      Max(20)     │                      │      Max(20)     │
    ///      └──────────────────┘                      └──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    ///                               ┌──────20 px───────┐
    ///                               │      Max(20)     │
    ///                               └──────────────────┘
    /// ```
    SpaceAround,
}
#[cfg(test)]
mod tests {}
