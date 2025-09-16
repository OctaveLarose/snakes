#include "game.h"
#include "renderer.h"
#include <iostream>

int main() {
  GameState game_state;
  Renderer renderer;
  try {
    renderer.render_loop(&game_state);
  } catch (GameOverException e) {
    std::cout << "Game over! Final score: " << e.getFinalScore() << std::endl;
  }
}
