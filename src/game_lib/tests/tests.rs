use super::super::*;


mod tests {
    
    use super::status::ElementStatus;
    use super::super::common;

    
    #[test]
    fn test_game_constructor() {
        // Should assemble a 10x10 matrix
        let test_game = common::test_game();

        assert_eq!(test_game.players.players[0].name, "P1")
    }

    #[test]
    fn test_grid_display_output () {
        let test_game = common::test_game();
        test_game.grid.display_grid();
    }
    

    // #[test]
    // fn test_hit_and_miss_grid_update() {
    //     let mut test_game = common::test_game();

    //     test_game.grid.update_grid("hit".into(), 0_u8, 0_u8);
    //     match test_game.grid.fetch_position_value(0_u8, 0_u8) {
    //         Some(x) => assert_eq!(x.status, ElementStatus::X),
    //         None => ()
    //     };

    //     test_game.grid.update_grid("miss".into(), 0_u8, 1_u8);
    //     match test_game.grid.fetch_position_value(0_u8, 1_u8) {
    //         Some(x) => assert_eq!(x.status, ElementStatus::M),
    //         None => ()
    //     };

    //     // Shouldn't update grid
    //     test_game.grid.update_grid("tacos".into(), 0_u8, 2_u8);
    //     match test_game.grid.fetch_position_value(0_u8, 2_u8) {
    //         Some(x) => assert_eq!(x.status, ElementStatus::O),
    //         None => ()
    //     };
    // }

    #[test]
    fn test_grid_search_overflow() {
        // let test_game = common::test_game();
        // let test_game_grid_copy = test_game.grid.grid;

        // TODO: Figure out how to compare 2-d matrices.
    }

    #[test]
    fn test_player_turn_swap() {
        let mut test_game = common::test_game();
        assert_eq!(test_game._cur_player, 0);
        test_game.update_current_player();
        assert_eq!(test_game._cur_player, 1);
        test_game.update_current_player();
        assert_eq!(test_game._cur_player, 0);
    }

    #[test]
    // fn test_turn_taking() {
    //     use super::game_loop::GameLoop;
    //     let mut test_game = GameLoop::new();
    //     test_game.turn();
    // }
    // Uncomment this test to test the CLI setup of the game
    #[test]
    fn test_game_setup() {
        use super::game_loop::GameLoop;
        let mut test_game = GameLoop::new();
    }
}