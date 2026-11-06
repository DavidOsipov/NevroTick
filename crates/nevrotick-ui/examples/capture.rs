//! Bounded offscreen captures of the real app on Slint's public software platform.
//!
//! Development tool only: no native event loop, backend, service or clock work.
//! The guarded verifier must supply the admitted font/SDK resources and the
//! owned `/work` directory. Nothing is uploaded and no user path is accepted.
#![forbid(unsafe_code)]

use std::cell::Cell;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use nevrotick_ui::slint_host::{self, AppWindow, Page};
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Key, Platform, PlatformError, WindowAdapter, WindowEvent};
use slint::{ComponentHandle, Model, PhysicalSize, Rgb8Pixel};

thread_local! {
    /// Only this devtool's Slint animation/timer clock, never a backend/OS clock.
    static PRESENTATION_TIME: Cell<Duration> = const { Cell::new(Duration::ZERO) };
}

/// A quarter second settles all reached pinned Fluent finite transitions (at most 200 ms).
const FRAME_STEP: Duration = Duration::from_millis(250);
/// At most 64 fixed frames; reject another step rather than wrap, sleep or rewind.
const MAX_PRESENTATION_TIME: Duration = Duration::from_secs(16);

/// Advance only the capture platform's monotonic presentation timeline.
///
/// # Errors
/// Rejects overflow or more than 64 quarter-second frame steps, without mutation.
fn settle_frame() -> Result<(), io::Error> {
    PRESENTATION_TIME.with(|clock| -> Result<(), io::Error> {
        let next = clock
            .get()
            .checked_add(FRAME_STEP)
            .filter(|next| *next <= MAX_PRESENTATION_TIME)
            .ok_or_else(|| io::Error::other("Capture presentation clock budget exceeded"))?;
        clock.set(next);
        Ok(())
    })?;
    slint::platform::update_timers_and_animations();
    Ok(())
}

/// Bounded plain-text layout fixture; not an external catalog or semantic grant.
#[path = "../tests/private/quit_layout.rs"]
mod quit_layout;

/// One owned offscreen adapter, shared only with the installed test platform.
struct Offscreen(Rc<MinimalSoftwareWindow>);

impl Platform for Offscreen {
    /// Supply the adapter for the single real app root; no OS window is created.
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.0.clone())
    }

    /// Drive toolkit animations deterministically, without host or service time.
    fn duration_since_start(&self) -> Duration {
        PRESENTATION_TIME.with(Cell::get)
    }
}

/// Capture fixed pages and validation states into fresh bounded PPM files.
///
/// # Errors
/// Returns platform, checked initialization, rendering or owned output failures.
fn main() -> Result<(), Box<dyn Error>> {
    let output = Path::new("/work/slint-captures");
    fs::create_dir(output)?;
    let adapter = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
    slint::platform::set_platform(Box::new(Offscreen(adapter.clone())))?;
    let host = slint_host::create()?;
    let window = host.window();
    window.show()?;
    for width in [640, 1080] {
        window.invoke_mode_requested(false);
        for (page, name) in [
            (Page::Status, "status"),
            (Page::Settings, "settings"),
            (Page::About, "about"),
        ] {
            window.invoke_navigate(page);
            capture(&adapter, output, &format!("simple-{name}-{width}"), width)?;
        }
        window.invoke_mode_requested(true);
        for (page, name) in [
            (Page::Status, "status"),
            (Page::Sources, "sources"),
            (Page::Statistics, "statistics"),
            (Page::Diagnostics, "diagnostics"),
            (Page::Settings, "settings"),
            (Page::About, "about"),
        ] {
            window.invoke_navigate(page);
            capture(&adapter, output, &format!("power-{name}-{width}"), width)?;
        }
    }
    capture_preference_groups(window, &adapter, output)?;
    capture_invalid_drafts(window, &adapter, output)?;
    capture_source_workflow(window, &adapter, output)?;
    capture_recovery_shell_diagnostics(window, &adapter, output)?;
    capture_policy_reviews(window, &adapter, output)?;
    capture_source_inspection(window, &adapter, output)?;
    window.hide()?;
    println!("Captured 50 offscreen frames; no native Windows behavior was exercised.");
    Ok(())
}

/// Observe both shared preference groups at the minimum supported viewport.
///
/// # Errors
/// Returns a missing actual route or software-render/output failure.
fn capture_preference_groups(
    window: &AppWindow,
    adapter: &MinimalSoftwareWindow,
    output: &Path,
) -> Result<(), Box<dyn Error>> {
    window.invoke_mode_requested(false);
    window.invoke_navigate(Page::Settings);
    for (section, name) in [
        (2, "notifications-simple-640x600"),
        (4, "personal-simple-640x600"),
    ] {
        window.invoke_settings_section_requested(section);
        if window.get_settings_section() != section || window.get_current_page() != Page::Settings {
            return Err(io::Error::other("Preference group route was not reached").into());
        }
        capture_sized(adapter, output, name, 640, 600)?;
    }
    window.invoke_settings_section_requested(0);
    Ok(())
}

/// Admit the exact bounded raw bytes used by a code-owned capture scenario.
///
/// # Errors
/// Returns a context/byte-admission failure rather than rendering a fake draft.
fn edit_text(window: &AppWindow, field: i32, value: &str) -> Result<(), Box<dyn Error>> {
    if window.invoke_raw_text_edited(field, value.into()) != value {
        return Err(io::Error::other("Capture raw text was not admitted").into());
    }
    Ok(())
}

/// Admit a code-owned raw selector before its ordinary capture action.
///
/// # Errors
/// Returns a closed-choice/context mismatch without forging a view property.
fn edit_choice(window: &AppWindow, field: i32, value: i32) -> Result<(), Box<dyn Error>> {
    if window.invoke_raw_choice_edited(field, value) != value {
        return Err(io::Error::other("Capture raw choice was not admitted").into());
    }
    Ok(())
}

/// Capture raw rejected source/interval text through the real receiving callbacks.
///
/// # Errors
/// Returns an owned output or render failure; no rejected text enters a log.
fn capture_invalid_drafts(
    window: &AppWindow,
    adapter: &MinimalSoftwareWindow,
    output: &Path,
) -> Result<(), Box<dyn Error>> {
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
    window.invoke_edit_requested();
    edit_text(window, 1, "https://unaccepted.invalid")?;
    edit_text(window, 2, "0")?;
    window.invoke_update_requested(
        window.get_draft_label(),
        window.get_draft_host(),
        window.get_draft_protocol_choice(),
        window.get_draft_port(),
    );
    capture(adapter, output, "source-invalid-1080", 1080)?;
    window.invoke_cancel_requested();
    window.invoke_navigate(Page::Settings);
    edit_choice(window, 0, 4)?;
    edit_text(window, 4, "0")?;
    window.invoke_interval_save_requested(4, "0".into());
    window.invoke_settings_section_requested(2);
    window.invoke_notifications_requested(false);
    window.invoke_settings_section_requested(0);
    capture(adapter, output, "interval-invalid-1080", 1080)?;
    capture(adapter, output, "interval-invalid-640", 640)?;
    realize_recovery_viewport(adapter, 600)?;
    window.set_page_scroll_y(0.0);
    window.invoke_interval_save_requested(4, window.get_draft_custom_minutes());
    if window.get_interval_error().is_empty() {
        return Err(io::Error::other("Invalid interval was not rejected at minimum size").into());
    }
    capture_sized(adapter, output, "interval-invalid-640x600", 640, 600)?;
    window.invoke_interval_cancel_requested();
    Ok(())
}

/// Render explicit NTP editing, three-operation review and both empty-list widths.
///
/// # Errors
/// Returns capture failure or a missing expected source-workflow transition.
fn capture_source_workflow(
    window: &AppWindow,
    adapter: &MinimalSoftwareWindow,
    output: &Path,
) -> Result<(), Box<dyn Error>> {
    window.invoke_navigate(Page::Sources);
    window.invoke_add_requested();
    window.invoke_protocol_requested(1);
    if !window.get_editor_adding() || window.get_draft_port() != "123" {
        return Err(io::Error::other("Source Add editor did not open with NTP default").into());
    }
    capture(adapter, output, "source-add-ntp-1080", 1080)?;
    edit_text(window, 0, "Local plain NTP")?;
    edit_text(window, 1, "local-preview.example.invalid")?;
    edit_text(window, 2, "123")?;
    window.invoke_update_requested(
        "Local plain NTP".into(),
        "local-preview.example.invalid".into(),
        1,
        "123".into(),
    );
    window.invoke_source_selected(2);
    window.invoke_edit_requested();
    edit_text(window, 0, "Edited sample")?;
    edit_text(window, 1, "edited-preview.example.invalid")?;
    edit_text(window, 2, "4460")?;
    window.invoke_update_requested(
        "Edited sample".into(),
        "edited-preview.example.invalid".into(),
        0,
        "4460".into(),
    );
    window.invoke_remove_requested(3);
    window.invoke_review_requested();
    if !window.get_review_open() || window.get_source_changes().row_count() != 3 {
        return Err(io::Error::other("Expected Add/Edit/Remove review was not reached").into());
    }
    capture(adapter, output, "source-review-1080", 1080)?;
    window.invoke_review_close_requested();
    let identities = window
        .get_sources()
        .iter()
        .map(|row| row.view_id)
        .collect::<Vec<_>>();
    for identity in identities {
        window.invoke_remove_requested(identity);
    }
    if window.get_sources().row_count() != 0 || window.get_selected_source() != 0 {
        return Err(io::Error::other("Expected empty source preview was not reached").into());
    }
    for width in [640, 1080] {
        capture(adapter, output, &format!("source-empty-{width}"), width)?;
    }
    window.invoke_review_requested();
    if !window.get_review_open() {
        return Err(io::Error::other("Source review was not reached for discard").into());
    }
    window.invoke_discard_requested();
    if window.get_sources().row_count() != 3 || window.get_source_changes().row_count() != 0 {
        return Err(io::Error::other("Source baseline was not restored by discard").into());
    }
    Ok(())
}

/// Exercise newly reached recovery, minimum-size modals and read-only reports.
///
/// # Errors
/// Returns a missing expected transition, render failure or owned output failure.
fn capture_recovery_shell_diagnostics(
    window: &AppWindow,
    adapter: &MinimalSoftwareWindow,
    output: &Path,
) -> Result<(), Box<dyn Error>> {
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
    window.invoke_recovery_open_requested();
    if window.get_current_page() != Page::Recovery {
        return Err(io::Error::other("Recovery route was not reached").into());
    }
    for width in [640, 1080] {
        capture(adapter, output, &format!("recovery-{width}"), width)?;
    }
    edit_choice(window, 1, 1)?;
    edit_text(window, 5, "https://invalid.invalid")?;
    // Establish the final viewport before the receiver decides what to reveal.
    realize_recovery_viewport(adapter, 780)?;
    window.set_page_scroll_y(0.0);
    window.invoke_recovery_update_requested(1, "https://invalid.invalid".into(), "".into());
    if window.get_recovery_error().is_empty() {
        return Err(io::Error::other("Invalid recovery helper was not rejected").into());
    }
    capture(adapter, output, "recovery-invalid-640", 640)?;
    // Size/layout are established before the actual repeat Submit. The capture
    // must observe the receiving reveal, never invoke it as an observer repair.
    realize_recovery_viewport(adapter, 600)?;
    window.set_page_scroll_y(0.0);
    window.invoke_recovery_update_requested(
        window.get_draft_recovery_count(),
        window.get_draft_recovery_first(),
        window.get_draft_recovery_second(),
    );
    capture_sized(adapter, output, "recovery-invalid-640x600", 640, 600)?;
    let recovery_scroll = window.get_page_scroll_y();
    if !window.get_recovery_content_focused()
        || !recovery_scroll.is_finite()
        || recovery_scroll >= 0.0
    {
        return Err(io::Error::other("Recovery form was not revealed after actual Submit").into());
    }
    window.invoke_recovery_cancel_requested();
    edit_choice(window, 1, 1)?;
    edit_text(window, 5, "helper.example.invalid")?;
    window.invoke_navigate(Page::Settings);
    if !window.get_recovery_leave_open() {
        return Err(io::Error::other("Dirty recovery leave decision was not reached").into());
    }
    capture_sized(adapter, output, "recovery-leave-640x600", 640, 600)?;
    window.invoke_recovery_leave_stay_requested();
    window.invoke_recovery_cancel_requested();
    window.invoke_recovery_back_requested();
    window.invoke_mode_requested(false);
    window.invoke_settings_section_requested(1);
    capture(adapter, output, "service-simple-640", 640)?;
    window.invoke_mode_requested(true);
    capture_sized(adapter, output, "service-power-640x600", 640, 600)?;
    window.invoke_navigate(Page::Diagnostics);
    window.invoke_diagnostic_view_requested(1);
    for width in [640, 1080] {
        capture(
            adapter,
            output,
            &format!("diagnostic-report-{width}"),
            width,
        )?;
    }
    window.invoke_diagnostic_view_requested(2);
    window.invoke_diagnostic_collection_requested(2, 1);
    capture_sized(adapter, output, "diagnostic-collection-640x600", 640, 600)?;
    window.invoke_diagnostic_view_requested(3);
    capture_sized(adapter, output, "diagnostic-correction-640x600", 640, 600)?;
    window.invoke_diagnostic_view_requested(0);
    capture_quit_warning(window, adapter, output)
}

/// Observe ordinary and deliberately long continuity warnings at minimum size.
///
/// # Errors
/// Returns an unexpected dialog/scroll transition, render or owned output failure.
fn capture_quit_warning(
    window: &AppWindow,
    adapter: &MinimalSoftwareWindow,
    output: &Path,
) -> Result<(), Box<dyn Error>> {
    window.invoke_mode_requested(false);
    window.invoke_navigate(Page::Settings);
    window.invoke_quit_requested();
    if !window.get_quit_warning_open() {
        return Err(io::Error::other("Quit continuity warning was not reached").into());
    }
    capture_sized(adapter, output, "quit-warning-top-640x600", 640, 600)?;
    for pressed in [true, false] {
        let text = Key::End.into();
        window.window().dispatch_event(if pressed {
            WindowEvent::KeyPressed { text }
        } else {
            WindowEvent::KeyReleased { text }
        });
    }
    if window.get_quit_body_scroll_y().abs().to_bits() != 0.0_f32.to_bits() {
        return Err(io::Error::other("The ordinary English warning unexpectedly scrolled").into());
    }
    capture_sized(adapter, output, "quit-warning-bottom-640x600", 640, 600)?;
    // Root visually verified the ordinary English copy fits. A separate bounded
    // long-text fixture exercises overflow without changing product wording.
    quit_layout::expand(window);
    capture_sized(adapter, output, "quit-warning-long-top-640x600", 640, 600)?;
    if window.get_quit_body_scroll_limit() <= 0.0 {
        return Err(io::Error::other("Long warning fixture did not overflow").into());
    }
    for pressed in [true, false] {
        let text = Key::End.into();
        window.window().dispatch_event(if pressed {
            WindowEvent::KeyPressed { text }
        } else {
            WindowEvent::KeyReleased { text }
        });
    }
    if window.get_quit_body_scroll_y() >= 0.0 {
        return Err(io::Error::other("Long warning fixture did not scroll using keyboard").into());
    }
    capture_sized(
        adapter,
        output,
        "quit-warning-long-bottom-640x600",
        640,
        600,
    )?;
    window.invoke_quit_cancel_requested();
    Ok(())
}

/// Reach checked NTP and observe-only reviews without activating a policy.
///
/// # Errors
/// Returns a missing expected transition, bounded render or owned output failure.
fn capture_policy_reviews(
    window: &AppWindow,
    adapter: &MinimalSoftwareWindow,
    output: &Path,
) -> Result<(), Box<dyn Error>> {
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Settings);
    window.invoke_settings_section_requested(3);
    if window.get_settings_section() != 3 || !window.get_settings_section_focused() {
        return Err(io::Error::other("Focused Time policy section was not reached").into());
    }
    for (choice, profile) in [(1, "ntp"), (2, "observe")] {
        window.invoke_policy_edit_requested();
        edit_choice(window, 3, choice)?;
        window.invoke_policy_review_requested(choice);
        if window.get_policy_phase() != 2
            || !window.get_policy_has_preview()
            || window.get_policy_accepted_choice() != choice
            || !window.get_policy_error().is_empty()
            || window.get_page_scroll_y().to_bits() != 0.0_f32.to_bits()
        {
            return Err(io::Error::other("Expected local profile review was not reached").into());
        }
        for (width, height) in [(640, 600), (1080, 780)] {
            capture_sized(
                adapter,
                output,
                &format!("policy-{profile}-review-{width}x{height}"),
                width,
                height,
            )?;
            window.set_page_scroll_y(0.0);
        }
        window.invoke_policy_discard_requested();
        if window.get_policy_has_preview() || window.get_policy_phase() != 0 {
            return Err(io::Error::other("Profile preview was not discarded").into());
        }
    }
    Ok(())
}

/// Render a separate query/no-match projection and current checked source details.
///
/// # Errors
/// Returns a missing expected transition, bounded render or owned output failure.
fn capture_source_inspection(
    window: &AppWindow,
    adapter: &MinimalSoftwareWindow,
    output: &Path,
) -> Result<(), Box<dyn Error>> {
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
    window.invoke_source_query_clear_requested();
    window.invoke_source_details_close_requested();
    let baseline_ids = window
        .get_sources()
        .iter()
        .map(|row| row.view_id)
        .collect::<Vec<_>>();
    if baseline_ids != [1, 2, 3] || window.get_source_changes().row_count() != 0 {
        return Err(io::Error::other("Expected source baseline was not restored").into());
    }
    if !window.invoke_source_search_requested(true) {
        return Err(io::Error::other("Expected source search editor was not reached").into());
    }
    edit_text(window, 3, "south")?;
    window.invoke_source_query_requested(window.get_draft_source_query());
    let visible_ids = window
        .get_visible_sources()
        .iter()
        .map(|row| row.view_id)
        .collect::<Vec<_>>();
    if window.get_accepted_source_query() != "south"
        || visible_ids != [2]
        || !window.get_source_query_error().is_empty()
    {
        return Err(io::Error::other("Expected filtered source projection was not reached").into());
    }
    capture_sized(adapter, output, "sources-query-640x600", 640, 600)?;
    if !window.invoke_source_search_requested(true) {
        return Err(io::Error::other("Expected source search editor was not reached").into());
    }
    edit_text(window, 3, "no-match.example.invalid")?;
    window.invoke_source_query_requested(window.get_draft_source_query());
    if window.get_visible_sources().row_count() != 0
        || window.get_accepted_source_query() != "no-match.example.invalid"
        || window
            .get_sources()
            .iter()
            .map(|row| row.view_id)
            .collect::<Vec<_>>()
            != baseline_ids
    {
        return Err(io::Error::other("Expected empty query projection was not reached").into());
    }
    capture_sized(adapter, output, "sources-no-match-640x600", 640, 600)?;
    window.invoke_source_query_clear_requested();
    let selected = window.get_selected_source();
    window.invoke_source_details_requested(2);
    if !window.get_inspection_detail_open()
        || window.get_inspection_detail_source().view_id != 2
        || window.get_inspection_detail_hidden()
        || !window.get_source_inspection_focused()
        || window.get_page_scroll_y().to_bits() != 0.0_f32.to_bits()
        || window.get_selected_source() != selected
    {
        return Err(io::Error::other("Expected focused source details were not reached").into());
    }
    capture_sized(adapter, output, "source-details-640x600", 640, 600)?;
    window.invoke_source_details_close_requested();
    if window.get_inspection_detail_open()
        || window.get_accepted_source_query() != ""
        || window
            .get_sources()
            .iter()
            .map(|row| row.view_id)
            .collect::<Vec<_>>()
            != baseline_ids
        || window.get_source_changes().row_count() != 0
    {
        return Err(io::Error::other("Source inspection did not restore its baseline view").into());
    }
    window.invoke_mode_requested(false);
    window.invoke_navigate(Page::Settings);
    window.invoke_settings_section_requested(0);
    window.set_page_scroll_y(0.0);
    Ok(())
}

/// Render one of two fixed widths at the ordinary 780-pixel height.
///
/// # Errors
/// Rejects an unlisted name/dimension, blank rendering, overflow or I/O failure.
fn capture(
    adapter: &MinimalSoftwareWindow,
    output: &Path,
    name: &str,
    width: u32,
) -> Result<(), Box<dyn Error>> {
    capture_sized(adapter, output, name, width, 780)
}

/// Render an admitted width/height, rejecting blank buffers before I/O.
///
/// `name` is a code-owned ASCII basename from the fixed capture matrix. It is
/// checked again here before joining the owned output directory.
///
/// # Errors
/// Rejects unlisted dimensions/names, blank rendering, overflow and I/O failure.
fn capture_sized(
    adapter: &MinimalSoftwareWindow,
    output: &Path,
    name: &str,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn Error>> {
    if ![640, 1080].contains(&width)
        || ![600, 780].contains(&height)
        || name.is_empty()
        || name.len() > 64
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    {
        return Err(io::Error::other("Capture requires a fixed size and basename").into());
    }
    let width_usize = usize::try_from(width)?;
    let count = width_usize
        .checked_mul(usize::try_from(height)?)
        .ok_or_else(|| io::Error::other("Capture size overflow"))?;
    adapter.set_size(PhysicalSize::new(width, height));
    adapter.window().request_redraw();
    let sentinel = Rgb8Pixel {
        r: 255,
        g: 0,
        b: 255,
    };
    let mut pixels = vec![sentinel; count];
    // Dynamic children can create their initial style animation during layout.
    // Realize that subtree first, then settle and record the actual final frame.
    let mut realized = false;
    adapter.draw_if_needed(|renderer| {
        renderer.render(&mut pixels, width_usize);
        realized = true;
    });
    if !realized {
        return Err(io::Error::other("Offscreen layout did not realize a frame").into());
    }
    settle_frame()?;
    adapter.window().request_redraw();
    pixels.fill(sentinel);
    let mut rendered = false;
    adapter.draw_if_needed(|renderer| {
        renderer.render(&mut pixels, width_usize);
        rendered = true;
    });
    if !rendered || pixels.iter().all(|pixel| *pixel == sentinel) {
        return Err(io::Error::other("Offscreen rendering produced no frame").into());
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output.join(format!("{name}.ppm")))?;
    write!(file, "P6\n{width} {height}\n255\n")?;
    let mut row = Vec::with_capacity(width_usize * 3);
    for pixels in pixels.chunks_exact(width_usize) {
        row.clear();
        for pixel in pixels {
            row.extend_from_slice(&[pixel.r, pixel.g, pixel.b]);
        }
        file.write_all(&row)?;
    }
    Ok(())
}

/// Realize a fixed viewport before observing the receiving form reveal.
///
/// # Errors
/// Rejects an unlisted height or missing actual draw; owns one bounded buffer.
fn realize_recovery_viewport(
    adapter: &MinimalSoftwareWindow,
    height: u32,
) -> Result<(), io::Error> {
    if ![600, 780].contains(&height) {
        return Err(io::Error::other("Unlisted receiving viewport"));
    }
    adapter.set_size(PhysicalSize::new(640, height));
    adapter.window().request_redraw();
    let sentinel = Rgb8Pixel {
        r: 255,
        g: 0,
        b: 255,
    };
    let mut frame = vec![sentinel; 640 * usize::try_from(height).map_err(io::Error::other)?];
    let mut painted = false;
    adapter.draw_if_needed(|renderer| {
        renderer.render(&mut frame, 640);
        painted = true;
    });
    if !painted || frame.iter().all(|pixel| *pixel == sentinel) {
        return Err(io::Error::other("Recovery minimum layout was not painted"));
    }
    Ok(())
}
