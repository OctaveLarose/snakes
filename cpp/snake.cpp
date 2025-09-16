#include "snake.h"
#include <iostream>
#include <vector>

Snake::Snake(int x, int y) { body.emplace_back(Pos(x, y)); }

void Snake::growBody() { body.push_back(body.back()); }

void Snake::moveBody(Direction dir) {
  auto last_pos = body[0];

  body[0].move(dir);

  for (int i = 1; i < body.size(); i++) {
    std::swap(last_pos, body[i]);
  }
}

std::vector<Pos> Snake::getBody() { return body; }
