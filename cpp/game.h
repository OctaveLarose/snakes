#pragma once

#include "snake.h"
#include <exception>

class GameOverException : public std::exception {
  int final_score;

public:
  GameOverException(int final_score) : final_score(final_score) {}
  int getFinalScore() { return final_score; }
};

class GameState {
public:
  static const int MAP_LENGTH = 20;
  static const int MAP_HEIGHT = 20;

  Snake snake;
  Pos food_pos;

  GameState();
  void update_game_state();

private:
  void spawnFood();
  bool check_if_game_over();
};
