pub mod cell_info;
pub mod draw_available_transformations;
pub mod game_finished;
pub mod task_queue;
pub mod top_bar;


/// Use this to choose the longest line, for `measure_text(longest_line)` and compute the required
/// width of a panel
fn longest<'a, S: AsRef<str>>(strings: impl Iterator<Item=&'a S>, mut default: &'a S) -> &'a S {
    let mut max_len = default.as_ref().len();
    for string in strings {
        let candidate_len = string.as_ref().len();
        if candidate_len > max_len {
            default = string;
            max_len = candidate_len;
        }
    }
    default
}
