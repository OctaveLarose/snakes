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

void Game::update_game_state(Direction input) {
  snake.moveBody(cur_dir);

  if (snake.getBody()[0] == food_pos) {
    // TODO: get bigger
    this->spawnFood();
  }
}

void Game::spawnFood() {
  food_pos = Pos(rand() % Game::MAP_LENGTH, rand() % Game::MAP_HEIGHT);
}
