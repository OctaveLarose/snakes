#include "game.h"
#include "renderer.h"
#include "snake.h"
#include <cstdlib>
#include <ctime>

Game::Game()
    : snake(Snake(Game::MAP_LENGTH / 2, Game::MAP_HEIGHT / 2)),
      cur_dir(Direction::RIGHT) {
  srand(time(0));
  this->spawnFood();
}

void Game::play() {
  auto renderer = Renderer();
  renderer.render_loop(this);
}

void Game::update_game_state() {
  snake.moveBody(cur_dir);

  if (this->check_if_game_over()) {
    std::exit(0);
  }

  if (snake.getBody()[0] == food_pos) {
    snake.growBody();
    this->spawnFood();
  }
}

void Game::spawnFood() {
  food_pos = Pos(rand() % Game::MAP_LENGTH, rand() % Game::MAP_HEIGHT);
}

bool Game::check_if_game_over() {
  auto snake_head = snake.getBody()[0];

  if (snake_head.x < 0 || snake_head.y < 0 ||
      snake_head.x >= Game::MAP_LENGTH || snake_head.y >= Game::MAP_HEIGHT) {
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
