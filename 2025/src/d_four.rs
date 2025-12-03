use super::c_three::Ranges;
pub fn solve_four(input: &[u8]) -> u64 {
    Ranges::from(input).sum_invalid_ids_thorough()
}
