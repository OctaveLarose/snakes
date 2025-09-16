#pragma once

#include "game.h"
#include <SFML/Graphics.hpp>

class Renderer {
  sf::RenderWindow window;

public:
  constexpr static const float MAP_TILE_SIZE = 20.0;

  Renderer();
  // TODO: these functions should take some game state instead of
  // an overaching Game class. But hey, that works for a toy project.
  void render_loop(GameState *game);
  void poll_event(GameState *game);
  void render_frame(GameState *game);
  void draw_circle(Pos circle_pos, sf::Color color);
};
