{$mode objfpc}{$H+}{$J-}
{$R+}

program snakeProgram;

uses
    ncurses,
    sysutils;

const
  SNAKE_CHAR = 'X';
  FOOD_CHAR = 'O';

type
   TPos = array [ 0..1 ] of integer;
   snakeArray = array of TPos;
   TDir = (drUp, drRight, drDown, drLeft);

operator = (pos1, pos2 : TPos) b : boolean;
begin
     Result := (pos1[0] = pos2[0]) and (pos1[1] = pos2[1]);
end;

procedure initGame;
begin
   Randomize;
   initscr();
   timeout(1);
   raw();
   keypad(stdscr, true);
   attroff(A_INVIS);
   curs_set(0);
   noecho();
end;

function initSnake (const snakeLength: Integer): snakeArray;
var
     I: Integer;
begin
     setlength(Result, snakeLength);
     for I := 0 to snakeLength - 1 do
       begin
         Result[I][0] := 10 - I;
         Result[I][1] := 5;
       end;
end;

procedure setSnakeDir(const ch: LongInt; var snakeDir: TDir);
begin
     case ch of
       KEY_UP: snakeDir := drUp;
       KEY_DOWN: snakeDir := drDown;
       KEY_LEFT: snakeDir := drLeft;
       KEY_RIGHT: snakeDir := drRight;
       end; // TODO : prevent player from being able to turn around
end;

procedure moveSnake(var snake: snakeArray; const snakeLength: Integer; const snakeDir: TDir);
var
     I: Integer;
begin
     I := snakeLength - 1;
     while I > 0 do
       begin
         snake[I] := snake[I - 1];
         I := I - 1;
       end;
     case snakeDir of
       drUp: snake[0][1] := snake[0][1] - 1;
       drDown: snake[0][1] := snake[0][1] + 1;
       drLeft: snake[0][0] := snake[0][0] - 1;
       drRight: snake[0][0] := snake[0][0] + 1;
       end;
end;

function checkForDeath (const snake: snakeArray; const snakeLength: Integer) : Bool;
var
     I: Integer;
begin
     for I := 1 to snakeLength - 1 do
       if snake[I] = snake[0] then
         Exit(true);
     Exit(false);
end;

procedure drawSnake(const snake: array of TPos);
var
   snkBlock: TPos;
begin
   for snkBlock in snake do
       mvprintw(snkBlock[1], snkBlock[0], SNAKE_CHAR);
end;

function getRandomPos : TPos;
begin
     Result[0] := Random(34); // TODO : needs to use map coordinates
     Result[1] := Random(34);
end;

var
   snake: snakeArray;
   snakeLength: Integer;
   snakeDir: TDir;
   foodPos: TPos;
   ch: LongInt;

begin
     snakeLength := 6;
     snakeDir := drRight;

     initGame();
     snake := initSnake(snakeLength);

     foodPos := getRandomPos();
     repeat
       ch := getch();
       setSnakeDir(ch, snakeDir);

       if (snake[0] = foodPos) then
       begin
          snakeLength := snakeLength + 1;
          setlength(snake, snakeLength);
          foodPos := getRandomPos();
       end;
       if checkForDeath(snake, snakeLength) = True then
          break;

       moveSnake(snake, snakeLength, snakeDir);
       drawSnake(snake);
       mvprintw(foodPos[1], foodPos[0], FOOD_CHAR);
       refresh();
       Clear();

       Sleep(100); // TODO : change, for obvious reasons
     until (ch = KEY_NPAGE);

     endwin();
end.
