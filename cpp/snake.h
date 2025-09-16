#pragma once

#include <vector>

enum Direction { LEFT, DOWN, RIGHT, UP };

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
  Direction cur_dir;

public:
  Snake(int x, int y, Direction dir);
  std::vector<Pos> getBody();
  void changeDir(Direction dir);
  void moveBody();
  void growBody();
};
