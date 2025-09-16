#include "renderer.h"
#include "game.h"
#include "snake.h"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Window/Keyboard.hpp>
#include <iostream>

#include <SFML/Graphics.hpp>

Renderer::Renderer() {
  window = sf::RenderWindow(
      sf::VideoMode(
          {(unsigned int)(GameState::MAP_LENGTH * Renderer::MAP_TILE_SIZE),
           (unsigned int)(GameState::MAP_HEIGHT * Renderer::MAP_TILE_SIZE)}),
      "Snake");
  window.setFramerateLimit(10);
}

void Renderer::render_loop(GameState *game) {
  while (window.isOpen()) {
    this->poll_event(game);
    this->render_frame(game);
    game->update_game_state();
  }
}

void Renderer::poll_event(GameState *game) {
  while (const std::optional event = window.pollEvent()) {
    if (event->is<sf::Event::Closed>()) {
      window.close();
    } else if (const auto *keyPressed = event->getIf<sf::Event::KeyPressed>()) {
      auto key = keyPressed->scancode;
      if ((key == sf::Keyboard::Scancode::Up ||
           key == sf::Keyboard::Scancode::W)) {
        game->snake.changeDir(Direction::UP);
      } else if ((key == sf::Keyboard::Scancode::Left ||
                  key == sf::Keyboard::Scancode::A)) {
        game->snake.changeDir(Direction::LEFT);
      } else if ((key == sf::Keyboard::Scancode::Down ||
                  key == sf::Keyboard::Scancode::S)) {
        game->snake.changeDir(Direction::DOWN);
      } else if ((key == sf::Keyboard::Scancode::Right ||
                  key == sf::Keyboard::Scancode::D)) {
        game->snake.changeDir(Direction::RIGHT);
      }
    }
  }
}

void Renderer::render_frame(GameState *game) {
  window.clear();

  // sf::RectangleShape map_outline({Game::MAP_LENGTH * Renderer::MAP_TILE_SIZE,
  //                                 Game::MAP_HEIGHT *
  //                                 Renderer::MAP_TILE_SIZE});
  // map_outline.setPosition({0, 0});
  // map_outline.setFillColor(sf::Color(0, 0, 0));
  // map_outline.setOutlineColor(sf::Color(255, 255, 255));
  // map_outline.setOutlineThickness(5);
  // window.draw(map_outline);

  for (Pos body_part : game->snake.getBody()) {
    this->draw_circle(body_part, sf::Color(255, 255, 255));
  }

  this->draw_circle(game->food_pos, sf::Color(255, 0, 0));

  window.display();
}

void Renderer::draw_circle(Pos circle_pos, sf::Color color) {
  sf::CircleShape circle;
  circle.setRadius(Renderer::MAP_TILE_SIZE / 2);
  circle.setPosition({(float)circle_pos.x * Renderer::MAP_TILE_SIZE,
                      (float)circle_pos.y * Renderer::MAP_TILE_SIZE});
  circle.setFillColor(color);
  window.draw(circle);
}
