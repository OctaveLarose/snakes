#include "snake.h"
#include <vector>

Snake::Snake(int x, int y, Direction dir) : cur_dir(dir) {
  body.emplace_back(Pos(x, y));
}

void Snake::changeDir(Direction new_dir) {
  if ((new_dir + 2) % 4 == this->cur_dir) {
    return;
  }
  this->cur_dir = new_dir;
}

void Snake::growBody() { body.push_back(body.back()); }

void Snake::moveBody() {
  auto last_pos = body[0];

  body[0].move(this->cur_dir);

  for (int i = 1; i < body.size(); i++) {
    std::swap(last_pos, body[i]);
  }
}

std::vector<Pos> Snake::getBody() { return body; }
