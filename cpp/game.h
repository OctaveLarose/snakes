#pragma once

#include "snake.h"

class Game {
public:
  const int MAP_LENGTH = 200;
  const int MAP_HEIGHT = 200;

  Snake snake;
  Direction cur_dir;
  Pos food_pos;

  Game();
  void play();
  void update_game_state(Direction input);

private:
  void spawnFood();
};
