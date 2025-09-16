#include "game.h"
#include "snake.h"
#include <cstdlib>
#include <ctime>

GameState::GameState()
    : snake(Snake(GameState::MAP_LENGTH / 2, GameState::MAP_HEIGHT / 2,
                  Direction::RIGHT)) {
  srand(time(0));
  this->spawnFood();
}

void GameState::update_game_state() {
  snake.moveBody();

  if (this->check_if_game_over()) {
    throw GameOverException(this->snake.getBody().size());
  }

  if (snake.getBody()[0] == food_pos) {
    snake.growBody();
    this->spawnFood();
  }
}

void GameState::spawnFood() {
  food_pos =
      Pos(rand() % GameState::MAP_LENGTH, rand() % GameState::MAP_HEIGHT);
}

bool GameState::check_if_game_over() {
  auto snake_head = snake.getBody()[0];

  if (snake_head.x < 0 || snake_head.y < 0 ||
      snake_head.x >= GameState::MAP_LENGTH ||
      snake_head.y >= GameState::MAP_HEIGHT) {
    return true;
  }

  // going downwards for performance reasons since most likely to hit back of
  // snake. does it actually matter a difference in practice? not really, no
  int i = snake.getBody().size() - 1;
  while (i > 0) {
    if (snake.getBody()[i] == snake_head) {
      return true;
    }
    i -= 1;
  }

  return false;
}
