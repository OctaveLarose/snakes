#include "renderer.h"
#include "snake.h"
#include <SFML/Window/Keyboard.hpp>
#include <iostream>

#include <SFML/Graphics.hpp>

Renderer::Renderer() {
  window = sf::RenderWindow(sf::VideoMode({500u, 500u}), "Snake");
  window.setFramerateLimit(60);
}

void Renderer::render_frame() {}

void Renderer::render_loop(Game *game) {
  while (window.isOpen()) {
    while (const std::optional event = window.pollEvent()) {
      if (event->is<sf::Event::Closed>()) {
        window.close();
      } else if (const auto *keyPressed =
                     event->getIf<sf::Event::KeyPressed>()) {
        if (keyPressed->scancode == sf::Keyboard::Scancode::Up ||
            keyPressed->scancode == sf::Keyboard::Scancode::W) {
          game->cur_dir = Direction::UP;
        } else if (keyPressed->scancode == sf::Keyboard::Scancode::Left ||
                   keyPressed->scancode == sf::Keyboard::Scancode::A) {
          game->cur_dir = Direction::LEFT;
        } else if (keyPressed->scancode == sf::Keyboard::Scancode::Down ||
                   keyPressed->scancode == sf::Keyboard::Scancode::S) {
          game->cur_dir = Direction::DOWN;
        } else if (keyPressed->scancode == sf::Keyboard::Scancode::Right ||
                   keyPressed->scancode == sf::Keyboard::Scancode::D) {
          game->cur_dir = Direction::RIGHT;
        }
      }
    }

    window.clear();

    // draw snake
    for (Pos body_part : game->snake.getBody()) {
      sf::CircleShape circle;
      circle.setRadius(10);
      circle.setPosition({(float)body_part.x, (float)body_part.y});
      window.draw(circle);
    }

    // draw food
    sf::CircleShape circle;
    circle.setRadius(10);
    circle.setFillColor(sf::Color(255, 0, 0));
    circle.setPosition({(float)game->food_pos.x, (float)game->food_pos.y});
    window.draw(circle);

    window.display();

    game->update_game_state(Direction::RIGHT);
  }
}
