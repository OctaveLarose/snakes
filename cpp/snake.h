#pragma once

#include <vector>

enum Direction { LEFT, RIGHT, UP, DOWN };

class Pos {
public:
  int x;
  int y;

  Pos() : x(-1), y(-1) {};
  Pos(int x, int y) : x(x), y(y) {};

  bool operator==(Pos other_pos) {
    return x == other_pos.x && y == other_pos.y;
  };

  void move(Direction dir) {
    switch (dir) {
    case Direction::LEFT:
      x -= 1;
      break;
    case Direction::RIGHT:
      x += 1;
      break;
    case Direction::UP:
      y -= 1;
      break;
    case Direction::DOWN:
      y += 1;
      break;
    };
  }
};

class Snake {
  std::vector<Pos> body = {};

public:
  Snake(int x, int y);
  std::vector<Pos> getBody();
  void moveBody(Direction dir);
  void growBody();
};
