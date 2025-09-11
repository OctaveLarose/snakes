#pragma once

#include <SFML/Graphics.hpp>
#include "game.h"

class Renderer {
  sf::RenderWindow window;

public:
  Renderer();
  void render_frame();
  void render_loop(Game *game);
};
