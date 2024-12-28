#[cfg(test)]
mod tests {
    use crate::player::should_player_move;

    const MOVEMENT_DELAY: f32 = 0.5;

    #[test]
    fn test_should_player_move_not_moving() {
        let current_time = 1.0;
        let last_movement = 0.6;
        let is_moving = false;

        let result = should_player_move(current_time, last_movement, is_moving);

        assert_eq!(result, false);
    }

    #[test]
    fn test_should_player_move_with_delay() {
        let current_time = 1.5;
        let last_movement = 0.6;
        let is_moving = false;

        let result = should_player_move(current_time, last_movement, is_moving);
        assert_eq!(result, true);
    }

    #[test]
    fn test_should_player_move_still_moving() {
        let current_time = 1.0;
        let last_movement = 0.6;
        let is_moving = true;

        let result = should_player_move(current_time, last_movement, is_moving);
        assert_eq!(result, true);
    }
}
