#pragma once

#include "snake.h"

class Game {
public:
  static const int MAP_LENGTH = 20;
  static const int MAP_HEIGHT = 20;

  Snake snake;
  Direction cur_dir;
  Pos food_pos;

  Game();
  void play();
  void update_game_state();

private:
  void spawnFood();
  bool check_if_game_over();
};
