#include "snake.h"
#include <iostream>
#include <vector>

Snake::Snake(int x, int y) {
  Pos *start_pos = new Pos(x, y);
  body.push_back(*start_pos);
}

void Snake::growBody() { body.push_back(body.back()); }

void Snake::moveBody(Direction dir) {
  for (Pos &body_part : body) {
    // std::cout << "x:" << body_part.x << ", y:" << body_part.y << std::endl;
    body_part.move(dir);
  }
}

std::vector<Pos> Snake::getBody() { return body; }
