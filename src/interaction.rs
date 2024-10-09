//! just the beginning of a notion here, but a view should be anything that takes control of the
//! user input, then yields it, either to another view or to an exit call?
//!
//! It seems appropriate that User flow can be a flowchart, in most cases starting at a root menu,
//! denoted as `root` below:
//!
//! ```ignore
//! root
//! |
//! |- new
//! |  |- resume -> resume creation flow (edit flow w/ no data)
//! |  |- contact  -> contact edit flow
//! |  |- education  -> edu edit flow
//! |  |- job  -> job edit flow
//! |  |- project  -> prj edit flow
//! |  |- skill  -> skill edit flow
//! |  |- summary  -> summary edit flow
//! |
//! |- edit data
//! |  |- resume -> data list -> go through resume edit flow
//! |  |- contact -> data list -> contact edit flow
//! |  |- education -> data list -> edu edit flow
//! |  |- job -> data list -> job edit flow
//! |  |- project -> data list -> prj edit flow
//! |  |- skill -> data list -> skill edit flow
//! |  |- summary -> data list -> summary edit flow
//! |
//! |- gen -> pick resume, format, save path -> preview as md in less -> save
//! ```
//!
//! An interaction is then anything that can do the following:
//! - render some sort of interface to the user
//! - resolve to a Result to be handled by the caller
//!
//! As such, an interaction is simply a step w/in the larger application state machine, possibly
//! each one its own sub-machine. The larger machine decides which interaction to enter & when
//! (using a routing state machine?), provides necessary data to each interaction, & updates app
//! state w/ data returned by each interaction.
//!
//! Really leads well to a Routing state machine, so let's go bikeshed on that for a bit!
