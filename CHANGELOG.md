# Change Log

## [Unreleased]

### Breaking Changes

- `gtk::Widget::can_activate_accel()` now takes a `SignalId` for the
  `signal_id` argument instead of `u32`.
- The closure passed to `gtk::Widget::connect_can_activate_accel()` now
  takes a `SignalId` for the `signal_id` argument instead of a `u32`.
