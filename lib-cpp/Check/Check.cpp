#include "Check.h"

bool Check::check_win() {
    int _x = current->x;
    int _y = current->y;
    bool win = false;

    int dir[8][2] = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}, {1, 1}, {-1, -1}, {-1, 1}, {1, -1}}; //first: x, second: y
    int turn = board->board[_y][_x];
    if (turn == 0) {
        throw std::invalid_argument("Invalid turn");
    } // board[y][x] is not 1 or -1 -> input value is invalid
 
    // check win
    for (int i = 0; i < 8; i+=2) {
        int dx_1 = dir[i][0];
        int dy_1 = dir[i][1];
        int cnt_1 = 0;
        int dx_2 = dir[i+1][0];
        int dy_2 = dir[i+1][1];
        int cnt_2 = 0;

        int x = _x, y = _y;
        // direction 1
        while (true) {
            x += dx_1, y += dy_1;

            if (board->isValid(x, y) && board->board[y][x] == turn) {
                cnt_1++;
            } else {
                break;
            }
        }

        x = _x, y = _y;
        // direction 2
        while (true) {
            x += dx_2, y += dy_2;

            if (board->isValid(x, y) && board->board[y][x] == turn) {
                cnt_2++;
            } else {
                break;
            }
        }

        if (turn == 1 && (cnt_1 + cnt_2 == 4)) {
            win = true;
            break;
        } else if (turn == -1 && ((cnt_1 + cnt_2 == 4) || (cnt_1 + cnt_2 == 5))) {
            win = true;
            break;
        }
    }

    return win;
}